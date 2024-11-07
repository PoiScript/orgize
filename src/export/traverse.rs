use crate::ast::*;
use crate::syntax::{SyntaxElement, SyntaxKind};
use rowan::ast::AstNode;
use SyntaxKind::*;

use super::event::{Container, Event};

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
enum TraversalControl {
    Up,
    Stop,
    Skip,
    #[default]
    Continue,
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

/// A trait for enumerating org syntax tree
///
/// ### `TraversalContext`
///
/// `TraversalContext` can be used to control the traversal.
///
/// For example, `ctx.skip()` will skips the traversal for current
/// element and its descendants and improve the traversal performance.
///
/// ```rust
/// use orgize::{
///     export::{Container, Event, HtmlExport, TraversalContext, Traverser},
///     Org,
/// };
/// use slugify::slugify;
///
/// #[derive(Default)]
/// struct Toc(HtmlExport);
///
/// impl Traverser for Toc {
///     fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
///         match event {
///             Event::Enter(Container::Headline(headline)) => {
///                 let title = headline.title().map(|e| e.to_string()).collect::<String>();
///                 self.0.push_str(&format!("<a href='#{}'>", slugify!(&title)));
///                 for elem in headline.title() {
///                     self.element(elem, ctx);
///                 }
///                 self.0.push_str("</a>");
///                 if headline.headlines().count() > 0 {
///                     self.0.push_str("<ul>");
///                 }
///             }
///             Event::Leave(Container::Headline(headline)) => {
///                 if headline.headlines().count() > 0 {
///                     self.0.push_str("</ul>");
///                 }
///             }
///             Event::Enter(Container::Section(_)) | Event::Leave(Container::Section(_)) => ctx.skip(),
///             Event::Enter(Container::Document(_)) | Event::Leave(Container::Document(_)) => {}
///             _ => self.0.event(event, ctx),
///         }
///     }
/// }
///
/// let org = Org::parse(r#"
/// * heading 1
/// section 1
/// ** heading 1.1
/// ** heading 1.2
/// * heading 2
/// section 2
/// * heading 3
/// **** heading 3.1"#);
/// let mut toc = Toc::default();
/// org.traverse(&mut toc);
/// assert_eq!(toc.0.finish(), "\
/// <a href='#heading-1'>heading 1</a>\
/// <ul><a href='#heading-1-1'>heading 1.1</a><a href='#heading-1-2'>heading 1.2</a></ul>\
/// <a href='#heading-2'>heading 2</a>\
/// <a href='#heading-3'>heading 3</a>\
/// <ul><a href='#heading-3-1'>heading 3.1</a></ul>");
/// ```
pub trait Traverser {
    /// Handles traversal event
    fn event(&mut self, event: Event, ctx: &mut TraversalContext);

    fn element(&mut self, element: SyntaxElement, ctx: &mut TraversalContext) {
        macro_rules! take_control {
            () => {
                match ctx.control {
                    TraversalControl::Stop => {
                        ctx.control = TraversalControl::Stop;
                        return;
                    }
                    TraversalControl::Up => {
                        ctx.control = TraversalControl::Skip;
                        return;
                    }
                    TraversalControl::Skip => {
                        ctx.control = TraversalControl::Continue;
                        return;
                    }
                    TraversalControl::Continue => {}
                }
            };
        }

        match element {
            SyntaxElement::Node(node) => {
                macro_rules! walk {
                    ($ast:ident) => {{
                        debug_assert!($ast::can_cast(node.kind()));
                        let node = $ast { syntax: node };
                        self.event(Event::Enter(Container::$ast(node.clone())), ctx);
                        take_control!();
                        for child in node.syntax.children_with_tokens() {
                            self.element(child, ctx);
                            take_control!();
                        }
                        self.event(Event::Leave(Container::$ast(node.clone())), ctx);
                        take_control!();
                    }};
                    (@$ast:ident) => {{
                        debug_assert!($ast::can_cast(node.kind()));
                        let node = $ast { syntax: node };
                        self.event(Event::$ast(node), ctx);
                        take_control!();
                    }};
                }

                match node.kind() {
                    DOCUMENT => walk!(Document),
                    HEADLINE => walk!(Headline),
                    SECTION => walk!(Section),
                    PARAGRAPH => walk!(Paragraph),
                    BOLD => walk!(Bold),
                    ITALIC => walk!(Italic),
                    STRIKE => walk!(Strike),
                    UNDERLINE => walk!(Underline),
                    LIST => walk!(List),
                    LIST_ITEM => walk!(ListItem),
                    CODE => walk!(Code),
                    INLINE_CALL => walk!(@InlineCall),
                    INLINE_SRC => walk!(@InlineSrc),
                    RULE => walk!(@Rule),
                    VERBATIM => walk!(Verbatim),
                    SPECIAL_BLOCK => walk!(SpecialBlock),
                    QUOTE_BLOCK => walk!(QuoteBlock),
                    CENTER_BLOCK => walk!(CenterBlock),
                    VERSE_BLOCK => walk!(VerseBlock),
                    COMMENT_BLOCK => walk!(CommentBlock),
                    EXAMPLE_BLOCK => walk!(ExampleBlock),
                    EXPORT_BLOCK => walk!(ExportBlock),
                    SOURCE_BLOCK => walk!(SourceBlock),
                    BABEL_CALL => walk!(BabelCall),
                    CLOCK => walk!(@Clock),
                    COOKIE => walk!(@Cookie),
                    RADIO_TARGET => walk!(RadioTarget),
                    DRAWER => walk!(Drawer),
                    DYN_BLOCK => walk!(DynBlock),
                    FN_DEF => walk!(FnDef),
                    FN_REF => walk!(FnRef),
                    FN_CONTENT => walk!(FnContent),
                    MACROS => walk!(@Macros),
                    SNIPPET => walk!(@Snippet),
                    TIMESTAMP_ACTIVE | TIMESTAMP_INACTIVE | TIMESTAMP_DIARY => walk!(@Timestamp),
                    TARGET => walk!(Target),
                    COMMENT => walk!(Comment),
                    FIXED_WIDTH => walk!(FixedWidth),
                    ORG_TABLE => walk!(OrgTable),
                    ORG_TABLE_RULE_ROW | ORG_TABLE_STANDARD_ROW => walk!(OrgTableRow),
                    ORG_TABLE_CELL => walk!(OrgTableCell),
                    LINK => walk!(Link),
                    LATEX_FRAGMENT => walk!(@LatexFragment),
                    LATEX_ENVIRONMENT => walk!(@LatexEnvironment),
                    ENTITY => walk!(@Entity),
                    LINE_BREAK => walk!(@LineBreak),
                    SUPERSCRIPT => walk!(Superscript),
                    SUBSCRIPT => walk!(Subscript),
                    KEYWORD => walk!(Keyword),
                    PROPERTY_DRAWER => walk!(PropertyDrawer),
                    #[cfg(feature = "syntax-org-fc")]
                    CLOZE => walk!(@Cloze),
                    BLOCK_CONTENT | LIST_ITEM_CONTENT => {
                        for child in node.children_with_tokens() {
                            self.element(child, ctx);
                            take_control!();
                        }
                    }
                    _ => {}
                }
            }
            SyntaxElement::Token(token) => match token.kind() {
                TEXT => {
                    self.event(Event::Text(Token(token)), ctx);
                    take_control!();
                }
                FN_LABEL => {
                    self.event(Event::FnLabel(Token(token)), ctx);
                    take_control!();
                }
                _ => {}
            },
        };
    }
}

pub struct FromFn<F: FnMut(Event)>(F);

impl<F: FnMut(Event)> Traverser for FromFn<F> {
    fn event(&mut self, event: Event, _: &mut TraversalContext) {
        (self.0)(event)
    }
}

pub struct FromFnWithCtx<F: FnMut(Event, &mut TraversalContext)>(F);

impl<F: FnMut(Event, &mut TraversalContext)> Traverser for FromFnWithCtx<F> {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        (self.0)(event, ctx)
    }
}

/// A helper for creating traverser
///
/// ```rust
/// use orgize::{
///     export::{from_fn, Container, Event, Traverser},
///     Org,
/// };
///
/// let mut count = 0;
/// let mut handler = from_fn(|event| {
///     if matches!(event, Event::Enter(Container::Headline(_))) {
///         count += 1;
///     }
/// });
/// Org::parse("* 1\n** 2\n*** 3\n****4").traverse(&mut handler);
/// assert_eq!(count, 3);
/// ```
pub fn from_fn<F: FnMut(Event)>(f: F) -> FromFn<F> {
    FromFn(f)
}

/// A helper for creating traverser
///
/// ```rust
/// use orgize::{
///     export::{from_fn_with_ctx, Container, Event, Traverser},
///     Org,
/// };
///
/// let mut count = 0;
/// let mut handler = from_fn_with_ctx(|event, ctx| {
///     if let Event::Enter(Container::Headline(hdl)) = event {
///         count += 1;
///         if &hdl.title_raw() == "cow" {
///             ctx.stop();
///         }
///     }
/// });
/// Org::parse("* 1\n* cow\n* 3").traverse(&mut handler);
/// assert_eq!(count, 2);
/// ```
pub fn from_fn_with_ctx<F: FnMut(Event, &mut TraversalContext)>(f: F) -> FromFnWithCtx<F> {
    FromFnWithCtx(f)
}
