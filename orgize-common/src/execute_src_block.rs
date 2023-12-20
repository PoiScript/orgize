use orgize::{
    ast::{AffiliatedKeyword, SourceBlock},
    rowan::ast::AstNode,
    SyntaxKind,
};
use std::{fs::File, io::Write, iter::once, path::Path, process};

use crate::{
    header_argument::{header_argument, property_drawer, property_keyword},
    utils::language_execute_command,
};

#[derive(Debug)]
enum Format {
    Code,
    List,
    Verbatim,
    Html,
    Latex,
    Raw,
}

pub fn execute(block: SourceBlock, path: &Path) -> anyhow::Result<Option<(usize, usize, String)>> {
    let arg1 = block.parameters().unwrap_or_default();
    let arg2 = property_drawer(block.syntax()).unwrap_or_default();
    let arg3 = property_keyword(block.syntax()).unwrap_or_default();
    let language = block.language().unwrap_or_default();
    let results = header_argument(&arg1, &arg2, &arg3, ":results", "no");

    if results == "no" {
        return Ok(None);
    }

    let Some(command) = language_execute_command(&language) else {
        anyhow::bail!("{language:?} is not supported.")
    };

    let mut segs = results.split(&[' ', '\t']).filter(|x| !x.is_empty());

    let format = match (segs.next(), segs.next()) {
        (Some("output"), Some("code")) | (Some("code"), None) => Format::Code,
        (Some("output"), Some("list")) | (Some("list"), None) => Format::List,
        (Some("output"), Some("scalar"))
        | (Some("scalar"), None)
        | (Some("output"), Some("verbatim"))
        | (Some("verbatim"), None) => Format::Verbatim,
        (Some("output"), Some("html")) | (Some("html"), None) => Format::Html,
        (Some("output"), Some("latex")) | (Some("latex"), None) => Format::Latex,
        (Some("output"), Some("raw")) | (Some("raw"), None) => Format::Raw,
        (Some("value"), _) => anyhow::bail!("{language:?} is not supported."),
        _ => return Ok(None),
    };

    let results = collect_output(command, &block.value(), format, path)?;

    if let Some((start, end)) = find_existing_results(&block) {
        Ok(Some((start, end, results)))
    } else {
        let start = block.end() as usize;
        Ok(Some((start, start, format!("\n#+RESULTS:\n{}\n", results))))
    }
}

fn collect_output(
    command: &str,
    value: &str,
    format: Format,
    path: &Path,
) -> anyhow::Result<String> {
    let path = path.join("orgize-temporary");

    let mut file = File::create(&path)?;

    file.write_all(value.as_bytes())?;

    let output = process::Command::new(command).arg(path).output()?;

    let output = String::from_utf8_lossy(&output.stdout);

    match format {
        Format::Code => Ok(once("#+begin_src")
            .chain(output.lines())
            .chain(once("#+end_src"))
            .fold(String::new(), |acc, line| acc + line + "\n")),

        Format::Html => Ok(once("#+begin_export html")
            .chain(output.lines())
            .chain(once("#+end_export"))
            .fold(String::new(), |acc, line| acc + line + "\n")),

        Format::Latex => Ok(once("#+begin_export latex")
            .chain(output.lines())
            .chain(once("#+end_export"))
            .fold(String::new(), |acc, line| acc + line + "\n")),

        Format::List => Ok(output
            .lines()
            .fold(String::new(), |acc, line| acc + "- " + line + "\n")),

        Format::Verbatim => Ok(output
            .lines()
            .fold(String::new(), |acc, line| acc + ": " + line + "\n")),

        Format::Raw => Ok(output.to_string()),
    }
}

fn find_existing_results(block: &SourceBlock) -> Option<(usize, usize)> {
    let results = block
        .syntax()
        .next_sibling()
        .filter(|n| {
            matches!(
                n.kind(),
                SyntaxKind::ORG_TABLE
                    | SyntaxKind::FIXED_WIDTH
                    | SyntaxKind::LIST
                    | SyntaxKind::SOURCE_BLOCK
                    | SyntaxKind::EXPORT_BLOCK
            )
        })
        .filter(|n| {
            n.children()
                .filter_map(AffiliatedKeyword::cast)
                .any(|k| k.key().eq_ignore_ascii_case("results"))
        })?;

    let mut iter = results
        .children_with_tokens()
        .skip_while(|n| n.kind() == SyntaxKind::AFFILIATED_KEYWORD)
        .take_while(|n| n.kind() != SyntaxKind::BLANK_LINE);

    let first = iter.next();

    let last = iter.last();

    let start = first.as_ref().map(|n| n.text_range().start())?;

    let end = last.or(first).map(|x| x.text_range().end())?;

    Some((start.into(), end.into()))
}
