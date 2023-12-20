// TODO: :noweb support

use orgize::{
    ast::{Headline, SourceBlock},
    rowan::{ast::AstNode, Direction},
    SyntaxKind,
};
use resolve_path::PathResolveExt;
use std::fmt::Write;
use std::path::PathBuf;

use crate::{
    header_argument::{header_argument, property_drawer, property_keyword},
    utils::language_comments,
};

pub fn tangle(
    block: SourceBlock,
    path: &PathBuf,
) -> anyhow::Result<Option<(PathBuf, Option<u32>, String, bool)>> {
    let arg1 = block.parameters().unwrap_or_default();
    let arg2 = property_drawer(block.syntax()).unwrap_or_default();
    let arg3 = property_keyword(block.syntax()).unwrap_or_default();
    let language = block.language().unwrap_or_default();

    let tangle = header_argument(&arg1, &arg2, &arg3, ":tangle", "no");

    if tangle == "no" {
        return Ok(None);
    }

    let comments = header_argument(&arg1, &arg2, &arg3, ":comments", "no");
    let padline = header_argument(&arg1, &arg2, &arg3, ":padline", "no");
    let shebang = header_argument(&arg1, &arg2, &arg3, ":shebang", "no");
    let mode = header_argument(
        &arg1,
        &arg2,
        &arg3,
        ":tangle-mode",
        if shebang == "yea" { "o755" } else { "no" },
    );
    let is_mkdir = header_argument(&arg1, &arg2, &arg3, ":mkdir", "no");

    let parent = block
        .syntax()
        .ancestors()
        .find(|n| n.kind() == SyntaxKind::HEADLINE || n.kind() == SyntaxKind::DOCUMENT);

    let nth = parent
        .as_ref()
        .and_then(|n| n.children().position(|c| &c == block.syntax()))
        .unwrap_or(1);

    let headline_title = parent.and_then(Headline::cast).map(|headline| {
        headline
            .title()
            .fold(String::new(), |a, n| a + &n.to_string())
    });

    let path = tangle.try_resolve_in(path)?.to_path_buf();

    let mut permission = None;
    let mut content = String::new();

    if mode != "no"
        && mode.len() == 4
        && mode.starts_with('o')
        && mode.bytes().skip(1).all(|b| (b'0'..=b'7').contains(&b))
    {
        permission = u32::from_str_radix(&mode[1..], 8).ok();
    }

    if shebang != "no" && !shebang.is_empty() {
        content.push_str(shebang);
    }

    if comments == "org" || comments == "both" {
        if let Some((begin, end)) = language_comments(&language) {
            let start = block
                .syntax()
                .siblings(Direction::Prev)
                .skip(1) // skip self
                .take_while(|n| n.kind() != SyntaxKind::SOURCE_BLOCK)
                .last();

            for sibling in start
                .into_iter()
                .flat_map(|start| start.siblings(Direction::Next))
                .take_while(|node| node != block.syntax())
            {
                for line in sibling.to_string().lines() {
                    if line.is_empty() {
                        let _ = writeln!(content);
                    } else {
                        let _ = writeln!(content, "{begin} {line} {end}");
                    }
                }
            }
        }
    }

    if comments == "yes" || comments == "link" || comments == "noweb" || comments == "both" {
        if let Some((begin, end)) = language_comments(&language) {
            let _ = writeln!(
                content,
                "{begin} [[file:{path}::*{title}][{title}:{nth}]] {end}",
                title = headline_title.as_deref().unwrap_or("No heading"),
                path = path.to_string_lossy(),
            );
        }
    }

    content.push_str(&block.value());

    if padline != "no" {
        let _ = writeln!(content);
    }

    if comments == "yes" || comments == "link" || comments == "noweb" || comments == "both" {
        if let Some((begin, end)) = language_comments(&language) {
            let _ = writeln!(
                content,
                "{begin} {title}:{nth} ends here {end}",
                title = headline_title.as_deref().unwrap_or("No heading"),
            );
        }
    }

    Ok(Some((path, permission, content, is_mkdir != "no")))
}
