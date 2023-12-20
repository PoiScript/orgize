use orgize::{
    ast::{Link, SourceBlock},
    export::{Container, Event, TraversalContext, Traverser},
    rowan::ast::{support, AstNode},
    SyntaxKind,
};
use orgize_common::header_argument;
use resolve_path::PathResolveExt;
use serde_json::json;
use std::path::PathBuf;
use tower_lsp::lsp_types::{DocumentLink, Url};

use crate::org_document::OrgDocument;

pub struct DocumentLinkTraverser<'a> {
    pub doc: &'a OrgDocument,
    pub links: Vec<DocumentLink>,
    pub path: Option<PathBuf>,
}

impl<'a> Traverser for DocumentLinkTraverser<'a> {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        let Some(base) = &self.path else {
            return ctx.skip();
        };

        match event {
            Event::Enter(Container::Link(link)) => {
                if let Some(link) = link_path(link, base, self.doc) {
                    self.links.push(link);
                }
                ctx.skip();
            }
            Event::Enter(Container::SourceBlock(block)) => {
                if let Some(link) = block_tangle(block, base, self.doc) {
                    self.links.push(link);
                }
                ctx.skip();
            }

            _ => {}
        }
    }
}

fn link_path(link: Link, base: &PathBuf, doc: &OrgDocument) -> Option<DocumentLink> {
    let path = support::token(link.syntax(), SyntaxKind::LINK_PATH)
        .or_else(|| support::token(link.syntax(), SyntaxKind::TEXT))?;

    let path_str = path.text();

    let (target, data) = if let Some(file) = path_str.strip_prefix("file:") {
        let path = file.try_resolve_in(base).ok()?;
        (Some(Url::from_file_path(path).ok()?), None)
    } else if path_str.starts_with('/') || path_str.starts_with("./") || path_str.starts_with("~/")
    {
        let path = path_str.try_resolve_in(base).ok()?;
        (Some(Url::from_file_path(path).ok()?), None)
    } else if path_str.starts_with("http://") || path_str.starts_with("https://") {
        (Some(Url::parse(path_str).ok()?), None)
    } else if let Some(id) = path_str.strip_prefix('#') {
        let url = Url::from_file_path(base).ok()?;
        (
            None,
            Some(json!(vec![
                "headline-id".to_string(),
                url.to_string(),
                id.to_string()
            ])),
        )
    } else {
        return None;
    };

    Some(DocumentLink {
        range: doc.range_of(
            path.text_range().start().into(),
            path.text_range().end().into(),
        ),
        tooltip: Some("Jump to link".into()),
        target,
        data,
    })
}

fn block_tangle(block: SourceBlock, base: &PathBuf, doc: &OrgDocument) -> Option<DocumentLink> {
    let parameters = block
        .syntax()
        .children()
        .find(|e| e.kind() == SyntaxKind::BLOCK_BEGIN)
        .into_iter()
        .flat_map(|n| n.children_with_tokens())
        .filter_map(|n| n.into_token())
        .find(|n| n.kind() == SyntaxKind::SRC_BLOCK_PARAMETERS)?;

    let tangle = header_argument(parameters.text(), "", "", ":tangle", "no");

    if tangle == "no" {
        return None;
    }

    let path = tangle.try_resolve_in(base).ok()?;
    let url = Url::from_file_path(path).ok()?;

    let start: u32 = parameters.text_range().start().into();

    let index = parameters.text().find(tangle).unwrap_or_default() as u32;

    let len = tangle.len() as u32;

    Some(DocumentLink {
        range: doc.range_of(start + index, start + index + len),
        tooltip: Some("Jump to tangle destination".into()),
        target: Some(url),
        data: None,
    })
}

impl<'a> DocumentLinkTraverser<'a> {
    pub fn new(doc: &'a OrgDocument, path: Option<PathBuf>) -> Self {
        DocumentLinkTraverser {
            path,
            links: vec![],
            doc,
        }
    }
}

pub fn resolve() {}
