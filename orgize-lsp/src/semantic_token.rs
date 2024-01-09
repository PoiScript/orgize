use orgize::{
    export::{Container, Event, TraversalContext, Traverser},
    rowan::{ast::AstNode, TextRange},
    SyntaxKind,
};
use tower_lsp::lsp_types::{Range, SemanticToken, SemanticTokenModifier, SemanticTokenType};

use crate::org_document::OrgDocument;

const TIMESTAMP: SemanticTokenType = SemanticTokenType::new("timestamp");
const HEADLINE_TODO_KEYWORD: SemanticTokenType = SemanticTokenType::new("headlineTodoKeyword");
const HEADLINE_DONE_KEYWORD: SemanticTokenType = SemanticTokenType::new("headlineDoneKeyword");
const HEADLINE_PRIORITY: SemanticTokenType = SemanticTokenType::new("headlinePriority");
const HEADLINE_TAGS: SemanticTokenType = SemanticTokenType::new("headlineTags");

pub const TYPES: &[SemanticTokenType] = &[
    TIMESTAMP,
    HEADLINE_TODO_KEYWORD,
    HEADLINE_DONE_KEYWORD,
    HEADLINE_PRIORITY,
    HEADLINE_TAGS,
];

pub const MODIFIERS: &[SemanticTokenModifier] = &[];

pub struct SemanticTokenTraverser<'a> {
    pub doc: &'a OrgDocument,

    pub range: Option<TextRange>,

    pub tokens: Vec<SemanticToken>,
    pub previous_line: u32,
    pub previous_start: u32,
}

impl<'a> Traverser for SemanticTokenTraverser<'a> {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        macro_rules! m {
            ($range:expr, $ty:expr $(,$modifiers:expr)*) => {{
                if let Some(token) =
                    self.create_token($range.start().into(), $range.end().into(), $ty)
                {
                    self.tokens.push(token);
                }
            }};
        }

        macro_rules! s {
            ($range:expr) => {
                if let Some(range) = self.range {
                    if !range.contains_range($range) {
                        return ctx.skip();
                    }
                }
            };
        }

        match event {
            Event::Enter(Container::Section(section)) => s!(section.syntax().text_range()),
            Event::Enter(Container::Paragraph(paragraph)) => s!(paragraph.syntax().text_range()),
            Event::Enter(Container::OrgTable(table)) => s!(table.syntax().text_range()),
            Event::Enter(Container::List(list)) => s!(list.syntax().text_range()),
            Event::Enter(Container::Drawer(drawer)) => s!(drawer.syntax().text_range()),
            Event::Enter(Container::DynBlock(block)) => s!(block.syntax().text_range()),

            Event::Enter(Container::Headline(headline)) => {
                s!(headline.syntax().text_range());

                for ch in headline.syntax().children_with_tokens() {
                    match ch.kind() {
                        SyntaxKind::HEADLINE_KEYWORD_DONE => {
                            m!(ch.text_range(), HEADLINE_DONE_KEYWORD)
                        }
                        SyntaxKind::HEADLINE_KEYWORD_TODO => {
                            m!(ch.text_range(), HEADLINE_TODO_KEYWORD)
                        }
                        SyntaxKind::HEADLINE_TAGS => m!(ch.text_range(), HEADLINE_TAGS),
                        SyntaxKind::HEADLINE_PRIORITY => m!(ch.text_range(), HEADLINE_PRIORITY),
                        SyntaxKind::NEW_LINE => break,
                        _ => {}
                    }
                }
            }

            Event::Timestamp(timestamp) => m!(timestamp.syntax().text_range(), TIMESTAMP),

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

    fn create_token(
        &mut self,
        start: u32,
        end: u32,
        kind: SemanticTokenType,
    ) -> Option<SemanticToken> {
        let length = end - start;
        let token_type = TYPES.iter().position(|item| item == &kind)? as u32;

        let line = self.doc.line_of(start);

        let start = start - self.doc.line_starts[line as usize];

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
