use orgize::{
    export::{Container, Event, TraversalContext, Traverser},
    rowan::{ast::AstNode, TextRange},
};
use tower_lsp::lsp_types::{Range, SemanticToken, SemanticTokenType};

use crate::org_document::OrgDocument;

/// Semantic token types that are used for highlighting
pub const LEGEND_TYPE: &[SemanticTokenType] = &[SemanticTokenType::COMMENT];

pub struct SemanticTokenTraverser<'a> {
    pub doc: &'a OrgDocument,

    pub range: Option<TextRange>,

    pub tokens: Vec<SemanticToken>,
    pub previous_line: u32,
    pub previous_start: u32,
}

impl<'a> Traverser for SemanticTokenTraverser<'a> {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        match event {
            Event::Enter(Container::Comment(comment)) => {
                let range = comment.syntax().text_range();

                if self.contains_range(range) {
                    if let Some(token) = self.create_token(
                        comment.begin(),
                        comment.end(),
                        SemanticTokenType::COMMENT,
                    ) {
                        self.tokens.push(token);
                    }
                }

                ctx.skip();
            }
            Event::Enter(Container::CommentBlock(comment)) => {
                let range = comment.syntax().text_range();

                if self.contains_range(range) {
                    if let Some(token) = self.create_token(
                        comment.begin(),
                        comment.end(),
                        SemanticTokenType::COMMENT,
                    ) {
                        self.tokens.push(token);
                    }
                }

                ctx.skip();
            }

            _ => {}
        }
    }
}

impl<'a> SemanticTokenTraverser<'a> {
    pub fn new(doc: &'a OrgDocument) -> Self {
        SemanticTokenTraverser {
            doc,
            range: None,
            previous_line: 0,
            previous_start: 0,
            tokens: vec![],
        }
    }

    pub fn with_range(doc: &'a OrgDocument, range: Range) -> Self {
        let start = doc.offset_of(range.start);
        let end = doc.offset_of(range.end);

        SemanticTokenTraverser {
            doc,
            range: Some(TextRange::new(start.into(), end.into())),
            previous_line: 0,
            previous_start: 0,
            tokens: vec![],
        }
    }

    fn contains_range(&self, range: TextRange) -> bool {
        match self.range {
            Some(r) => r.contains_range(range),
            None => true,
        }
    }

    fn create_token(
        &mut self,
        start: u32,
        end: u32,
        kind: SemanticTokenType,
    ) -> Option<SemanticToken> {
        let length = end - start;
        let token_type = LEGEND_TYPE.iter().position(|item| item == &kind)? as u32;

        let line = self.doc.line_of(start);
        let first = self.doc.line_of(line);
        let start = self.doc.line_of(start) - first;

        let delta_line = line - self.previous_line;
        let delta_start = if delta_line == 0 {
            start - self.previous_start
        } else {
            start
        };

        self.previous_line = line;
        self.previous_start = start;

        Some(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset: 0,
        })
    }
}
