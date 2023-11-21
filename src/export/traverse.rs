use crate::ast::*;
use crate::syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::{ast::AstNode, WalkEvent};
use SyntaxKind::*;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
enum TraversalControl {
    Up,
    Stop,
    Skip,
    #[default]
    Continue,
}

macro_rules! take_control {
    ($ctrl:expr) => {
        match $ctrl.control {
            TraversalControl::Stop => {
                $ctrl.control = TraversalControl::Stop;
                return;
            }
            TraversalControl::Up => {
                $ctrl.control = TraversalControl::Skip;
                return;
            }
            TraversalControl::Skip => {
                $ctrl.control = TraversalControl::Continue;
                return;
            }
            TraversalControl::Continue => {}
        }
    };
}

#[derive(Default)]
pub struct TraversalContext {
    control: TraversalControl,
}

impl TraversalContext {
    /// Stops traversal completely
    pub fn stop(&mut self) {
        self.control = TraversalControl::Stop;
    }
    /// Skips traversal of the current node's siblings
    pub fn up(&mut self) {
        self.control = TraversalControl::Up;
    }
    /// Skips traversal of the current node's descendants
    pub fn skip(&mut self) {
        self.control = TraversalControl::Skip;
    }
    /// Continues traversal
    pub fn r#continue(&mut self) {
        self.control = TraversalControl::Continue;
    }
}

/// Enumerates org syntax tree
///
/// Traverser enumerates org syntax tree and calls handle method on each
/// enumerated node and token.
///
/// Each handle method can returns a `TraversalControl` to control the traversal.
pub trait Traverser {
    fn element(&mut self, element: SyntaxElement, ctx: &mut TraversalContext) {
        match element {
            SyntaxElement::Node(node) => self.node(node, ctx),
            SyntaxElement::Token(token) => self.token(token, ctx),
        };
    }

    /// Called when visiting any node
    fn node(&mut self, node: SyntaxNode, ctx: &mut TraversalContext) {
        macro_rules! traverse_children {
            ($node:expr) => {{
                for child in $node.children_with_tokens() {
                    self.element(child, ctx);
                    take_control!(ctx);
                }
            }};
        }

        macro_rules! traverse {
            ($node:ident, $method:ident) => {{
                debug_assert!($node::can_cast(node.kind()));
                let node = $node { syntax: node };
                self.$method(WalkEvent::Enter(&node), ctx);
                take_control!(ctx);
                traverse_children!(&node.syntax);
                self.$method(WalkEvent::Leave(&node), ctx);
                take_control!(ctx);
            }};
        }

        match node.kind() {
            DOCUMENT => traverse!(Document, document),
            HEADLINE => traverse!(Headline, headline),
            SECTION => traverse!(Section, section),
            PARAGRAPH => traverse!(Paragraph, paragraph),
            BOLD => traverse!(Bold, bold),
            ITALIC => traverse!(Italic, italic),
            STRIKE => traverse!(Strike, strike),
            UNDERLINE => traverse!(Underline, underline),
            LIST => traverse!(List, list),
            LIST_ITEM => traverse!(ListItem, list_item),
            CODE => traverse!(Code, code),
            INLINE_CALL => traverse!(InlineCall, inline_call),
            INLINE_SRC => traverse!(InlineSrc, inline_src),
            RULE => traverse!(Rule, rule),
            VERBATIM => traverse!(Verbatim, verbatim),
            SPECIAL_BLOCK => traverse!(SpecialBlock, special_block),
            QUOTE_BLOCK => traverse!(QuoteBlock, quote_block),
            CENTER_BLOCK => traverse!(CenterBlock, center_block),
            VERSE_BLOCK => traverse!(VerseBlock, verse_block),
            COMMENT_BLOCK => traverse!(CommentBlock, comment_block),
            EXAMPLE_BLOCK => traverse!(ExampleBlock, example_block),
            EXPORT_BLOCK => traverse!(ExportBlock, export_block),
            SOURCE_BLOCK => traverse!(SourceBlock, source_block),
            BABEL_CALL => traverse!(BabelCall, babel_call),
            CLOCK => traverse!(Clock, clock),
            COOKIE => traverse!(Cookie, cookie),
            RADIO_TARGET => traverse!(RadioTarget, radio_target),
            DRAWER => traverse!(Drawer, drawer),
            DYN_BLOCK => traverse!(DynBlock, dyn_block),
            FN_DEF => traverse!(FnDef, fn_def),
            FN_REF => traverse!(FnRef, fn_ref),
            MACROS => traverse!(Macros, macros),
            SNIPPET => traverse!(Snippet, snippet),
            TIMESTAMP_ACTIVE | TIMESTAMP_INACTIVE | TIMESTAMP_DIARY => {
                traverse!(Timestamp, timestamp)
            }
            TARGET => traverse!(Target, target),
            COMMENT => traverse!(Comment, comment),
            FIXED_WIDTH => traverse!(FixedWidth, fixed_width),
            ORG_TABLE => traverse!(OrgTable, org_table),
            ORG_TABLE_RULE_ROW | ORG_TABLE_STANDARD_ROW => traverse!(OrgTableRow, org_table_row),
            ORG_TABLE_CELL => traverse!(OrgTableCell, org_table_cell),
            LINK => traverse!(Link, link),
            LATEX_FRAGMENT => traverse!(LatexFragment, latex_fragment),
            LATEX_ENVIRONMENT => traverse!(LatexEnvironment, latex_environment),
            ENTITY => traverse!(Entity, entity),

            BLOCK_CONTENT | LIST_ITEM_CONTENT => traverse_children!(node),

            kind => debug_assert!(!kind.is_element() && !kind.is_object()),
        }
    }

    /// Called when visiting any token
    fn token(&mut self, token: SyntaxToken, ctx: &mut TraversalContext) {
        if token.kind() == TEXT {
            self.text(token, ctx);
        }
        take_control!(ctx);
    }

    /// Called when visiting `Text` token
    fn text(&mut self, _token: SyntaxToken, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Document` node
    fn document(&mut self, _event: WalkEvent<&Document>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Headline` node
    fn headline(&mut self, _event: WalkEvent<&Headline>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Paragraph` node
    fn paragraph(&mut self, _event: WalkEvent<&Paragraph>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Section` node
    fn section(&mut self, _event: WalkEvent<&Section>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Rule` node
    fn rule(&mut self, _event: WalkEvent<&Rule>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Comment` node
    fn comment(&mut self, _event: WalkEvent<&Comment>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `InlineSrc` node
    fn inline_src(&mut self, _event: WalkEvent<&InlineSrc>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `InlineCall` node
    fn inline_call(&mut self, _event: WalkEvent<&InlineCall>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Code` node
    fn code(&mut self, _event: WalkEvent<&Code>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Bold` node
    fn bold(&mut self, _event: WalkEvent<&Bold>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Verbatim` node
    fn verbatim(&mut self, _event: WalkEvent<&Verbatim>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Italic` node
    fn italic(&mut self, _event: WalkEvent<&Italic>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Strike` node
    fn strike(&mut self, _event: WalkEvent<&Strike>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Underline` node
    fn underline(&mut self, _event: WalkEvent<&Underline>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `List` node
    fn list(&mut self, _event: WalkEvent<&List>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `ListItem` node
    fn list_item(&mut self, _event: WalkEvent<&ListItem>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `SpecialBlock` node
    fn special_block(&mut self, _event: WalkEvent<&SpecialBlock>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `QuoteBlock` node
    fn quote_block(&mut self, _event: WalkEvent<&QuoteBlock>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `CenterBlock` node
    fn center_block(&mut self, _event: WalkEvent<&CenterBlock>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `VerseBlock` node
    fn verse_block(&mut self, _event: WalkEvent<&VerseBlock>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `CommentBlock` node
    fn comment_block(&mut self, _event: WalkEvent<&CommentBlock>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `ExampleBlock` node
    fn example_block(&mut self, _event: WalkEvent<&ExampleBlock>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `ExportBlock` node
    fn export_block(&mut self, _event: WalkEvent<&ExportBlock>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `SourceBlock` node
    fn source_block(&mut self, _event: WalkEvent<&SourceBlock>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `BabelCall` node
    fn babel_call(&mut self, _event: WalkEvent<&BabelCall>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Clock` node
    fn clock(&mut self, _event: WalkEvent<&Clock>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Cookie` node
    fn cookie(&mut self, _event: WalkEvent<&Cookie>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `RadioTarget` node
    fn radio_target(&mut self, _event: WalkEvent<&RadioTarget>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Drawer` node
    fn drawer(&mut self, _event: WalkEvent<&Drawer>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `DynBlock` node
    fn dyn_block(&mut self, _event: WalkEvent<&DynBlock>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `FnDef` node
    fn fn_def(&mut self, _event: WalkEvent<&FnDef>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `FnRef` node
    fn fn_ref(&mut self, _event: WalkEvent<&FnRef>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Macros` node
    fn macros(&mut self, _event: WalkEvent<&Macros>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Snippet` node
    fn snippet(&mut self, _event: WalkEvent<&Snippet>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Timestamp` node
    fn timestamp(&mut self, _event: WalkEvent<&Timestamp>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Target` node
    fn target(&mut self, _event: WalkEvent<&Target>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `FixedWidth` node
    fn fixed_width(&mut self, _event: WalkEvent<&FixedWidth>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `OrgTable` node
    fn org_table(&mut self, _event: WalkEvent<&OrgTable>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `OrgTableRow` node
    fn org_table_row(&mut self, _event: WalkEvent<&OrgTableRow>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `OrgTableCell` node
    fn org_table_cell(&mut self, _event: WalkEvent<&OrgTableCell>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `Link` node
    fn link(&mut self, _event: WalkEvent<&Link>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `LatexFragment` node
    fn latex_fragment(&mut self, _event: WalkEvent<&LatexFragment>, _ctx: &mut TraversalContext);
    /// Called when entering or leaving `LatexEnvironment` node
    fn latex_environment(
        &mut self,
        _event: WalkEvent<&LatexEnvironment>,
        _ctx: &mut TraversalContext,
    );
    /// Called when entering or leaving `Entity` node
    fn entity(&mut self, _event: WalkEvent<&Entity>, _ctx: &mut TraversalContext);
}
