#![allow(deprecated)]

use orgize::{
    export::{Container, Event, TraversalContext, Traverser},
    rowan::ast::AstNode,
    SyntaxKind,
};
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::org_document::OrgDocument;

pub struct DocumentSymbolTraverser<'a> {
    pub doc: &'a OrgDocument,
    pub stack: Vec<usize>,
    pub symbols: Vec<DocumentSymbol>,
}

impl<'a> Traverser for DocumentSymbolTraverser<'a> {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        match event {
            Event::Enter(Container::Headline(headline)) => {
                let mut symbols = &mut self.symbols;
                for &i in &self.stack {
                    symbols = symbols[i].children.get_or_insert(vec![]);
                }

                let name = headline
                    .syntax()
                    .children_with_tokens()
                    .take_while(|n| n.kind() != SyntaxKind::NEW_LINE)
                    .map(|n| n.to_string())
                    .collect::<String>();

                let start = headline.begin();
                let end = headline.end() - 1;

                self.stack.push(symbols.len());
                symbols.push(DocumentSymbol {
                    children: None,
                    name,
                    detail: None,
                    kind: SymbolKind::STRING,
                    tags: Some(vec![]),
                    range: self.doc.range_of(start, end),
                    selection_range: self.doc.range_of(start, end),
                    deprecated: None,
                });
            }
            Event::Leave(Container::Headline(_)) => {
                self.stack.pop();
            }
            Event::Enter(Container::Section(_)) => ctx.skip(),
            _ => {}
        }
    }
}

impl<'a> DocumentSymbolTraverser<'a> {
    pub fn new(doc: &'a OrgDocument) -> Self {
        DocumentSymbolTraverser {
            doc,
            stack: vec![],
            symbols: vec![],
        }
    }
}
