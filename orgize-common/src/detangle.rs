use orgize::{
    ast::{Headline, SourceBlock},
    rowan::ast::AstNode,
    SyntaxKind,
};
use resolve_path::PathResolveExt;
use std::{fs, path::PathBuf};

use crate::{
    header_argument::{header_argument, property_drawer, property_keyword},
    utils::language_comments,
};

pub fn detangle(
    block: SourceBlock,
    file_path: &PathBuf,
) -> anyhow::Result<Option<(usize, usize, String)>> {
    let arg1 = block.parameters().unwrap_or_default();
    let arg2 = property_drawer(block.syntax()).unwrap_or_default();
    let arg3 = property_keyword(block.syntax()).unwrap_or_default();
    let language = block.language().unwrap_or_default();

    let tangle = header_argument(&arg1, &arg2, &arg3, ":tangle", "no");

    if tangle == "no" {
        return Ok(None);
    }

    let comments = header_argument(&arg1, &arg2, &arg3, ":comments", "no");

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

    let dest_path = tangle.try_resolve_in(file_path)?.to_path_buf();

    let content = fs::read_to_string(dest_path)?;

    let Some(begin) = block
        .syntax()
        .children()
        .find(|n| n.kind() == SyntaxKind::BLOCK_CONTENT)
    else {
        return Ok(None);
    };

    let text_range = begin.text_range();

    if comments == "yes" || comments == "link" || comments == "noweb" || comments == "both" {
        let begin_comments = format!(
            "[[file:{path}::*{title}][{title}:{nth}]]",
            title = headline_title.as_deref().unwrap_or("No heading"),
            path = file_path.to_string_lossy(),
        );
        let end_comments = format!(
            "{title}:{nth} ends here",
            title = headline_title.as_deref().unwrap_or("No heading"),
        );

        let mut block_content = String::new();

        for line in content
            .lines()
            .skip_while(|line| trim_comments(line, &language).unwrap_or_default() != begin_comments)
            .skip(1)
        {
            if trim_comments(line, &language).unwrap_or_default() == end_comments {
                return Ok(Some((
                    text_range.start().into(),
                    text_range.end().into(),
                    block_content,
                )));
            } else {
                block_content += line;
                block_content += "\n";
            }
        }

        tracing::warn!(
            "Cannot found contents wrapped by comments for code block {path}*{title}:{nth}.",
            title = headline_title.as_deref().unwrap_or("No heading"),
            path = file_path.to_string_lossy(),
        );

        return Ok(None);
    }

    Ok(Some((
        text_range.start().into(),
        text_range.end().into(),
        content,
    )))
}

fn trim_comments<'a>(input: &'a str, language: &str) -> Option<&'a str> {
    let (begin, end) = language_comments(language)?;
    Some(input.trim().strip_prefix(begin)?.strip_suffix(end)?.trim())
}
