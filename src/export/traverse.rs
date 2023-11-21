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
            LINE_BREAK => traverse!(LineBreak, line_break),
            SUPERSCRIPT => traverse!(Superscript, superscript),
            SUBSCRIPT => traverse!(Subscript, subscript),

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
    fn text(&mut self, token: SyntaxToken, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Document` node
    fn document(&mut self, event: WalkEvent<&Document>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Headline` node
    fn headline(&mut self, event: WalkEvent<&Headline>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Paragraph` node
    fn paragraph(&mut self, event: WalkEvent<&Paragraph>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Section` node
    fn section(&mut self, event: WalkEvent<&Section>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Rule` node
    fn rule(&mut self, event: WalkEvent<&Rule>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Comment` node
    fn comment(&mut self, event: WalkEvent<&Comment>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `InlineSrc` node
    fn inline_src(&mut self, event: WalkEvent<&InlineSrc>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `InlineCall` node
    fn inline_call(&mut self, event: WalkEvent<&InlineCall>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Code` node
    fn code(&mut self, event: WalkEvent<&Code>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Bold` node
    fn bold(&mut self, event: WalkEvent<&Bold>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Verbatim` node
    fn verbatim(&mut self, event: WalkEvent<&Verbatim>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Italic` node
    fn italic(&mut self, event: WalkEvent<&Italic>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Strike` node
    fn strike(&mut self, event: WalkEvent<&Strike>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Underline` node
    fn underline(&mut self, event: WalkEvent<&Underline>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `List` node
    fn list(&mut self, event: WalkEvent<&List>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `ListItem` node
    fn list_item(&mut self, event: WalkEvent<&ListItem>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `SpecialBlock` node
    fn special_block(&mut self, event: WalkEvent<&SpecialBlock>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `QuoteBlock` node
    fn quote_block(&mut self, event: WalkEvent<&QuoteBlock>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `CenterBlock` node
    fn center_block(&mut self, event: WalkEvent<&CenterBlock>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `VerseBlock` node
    fn verse_block(&mut self, event: WalkEvent<&VerseBlock>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `CommentBlock` node
    fn comment_block(&mut self, event: WalkEvent<&CommentBlock>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `ExampleBlock` node
    fn example_block(&mut self, event: WalkEvent<&ExampleBlock>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `ExportBlock` node
    fn export_block(&mut self, event: WalkEvent<&ExportBlock>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `SourceBlock` node
    fn source_block(&mut self, event: WalkEvent<&SourceBlock>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `BabelCall` node
    fn babel_call(&mut self, event: WalkEvent<&BabelCall>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Clock` node
    fn clock(&mut self, event: WalkEvent<&Clock>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Cookie` node
    fn cookie(&mut self, event: WalkEvent<&Cookie>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `RadioTarget` node
    fn radio_target(&mut self, event: WalkEvent<&RadioTarget>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Drawer` node
    fn drawer(&mut self, event: WalkEvent<&Drawer>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `DynBlock` node
    fn dyn_block(&mut self, event: WalkEvent<&DynBlock>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `FnDef` node
    fn fn_def(&mut self, event: WalkEvent<&FnDef>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `FnRef` node
    fn fn_ref(&mut self, event: WalkEvent<&FnRef>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Macros` node
    fn macros(&mut self, event: WalkEvent<&Macros>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Snippet` node
    fn snippet(&mut self, event: WalkEvent<&Snippet>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Timestamp` node
    fn timestamp(&mut self, event: WalkEvent<&Timestamp>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Target` node
    fn target(&mut self, event: WalkEvent<&Target>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `FixedWidth` node
    fn fixed_width(&mut self, event: WalkEvent<&FixedWidth>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `OrgTable` node
    fn org_table(&mut self, event: WalkEvent<&OrgTable>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `OrgTableRow` node
    fn org_table_row(&mut self, event: WalkEvent<&OrgTableRow>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `OrgTableCell` node
    fn org_table_cell(&mut self, event: WalkEvent<&OrgTableCell>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Link` node
    fn link(&mut self, event: WalkEvent<&Link>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `LatexFragment` node
    fn latex_fragment(&mut self, event: WalkEvent<&LatexFragment>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `LatexEnvironment` node
    fn latex_environment(
        &mut self,
        event: WalkEvent<&LatexEnvironment>,
        ctx: &mut TraversalContext,
    );
    /// Called when entering or leaving `Entity` node
    fn entity(&mut self, event: WalkEvent<&Entity>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `LineBreak` node
    fn line_break(&mut self, event: WalkEvent<&LineBreak>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Superscript` node
    fn superscript(&mut self, event: WalkEvent<&Superscript>, ctx: &mut TraversalContext);
    /// Called when entering or leaving `Subscript` node
    fn subscript(&mut self, event: WalkEvent<&Subscript>, ctx: &mut TraversalContext);
}
