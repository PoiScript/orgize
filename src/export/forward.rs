/// Forward traverser method implement to other
///
/// Used to "extend" some builtin traverser like `HtmlExport`.
///
/// ```rust
/// use orgize::{
///     ast::Headline,
///     export::{HtmlExport, TraversalContext, Traverser},
///     forward_handler,
///     rowan::{ast::AstNode, WalkEvent},
///     Org,
/// };
/// use slugify::slugify;
/// use std::cmp::min;
///
/// #[derive(Default)]
/// struct SlugifyTitleHandler(pub HtmlExport);
///
/// // AsMut trait is required
/// impl AsMut<HtmlExport> for SlugifyTitleHandler {
///     fn as_mut(&mut self) -> &mut HtmlExport {
///         &mut self.0
///     }
/// }
///
/// impl Traverser for SlugifyTitleHandler {
///     fn headline(&mut self, event: WalkEvent<&Headline>, ctx: &mut TraversalContext) {
///         if let WalkEvent::Enter(headline) = event {
///             let level = min(headline.level(), 6);
///             let title = headline.title().map(|e| e.to_string()).collect::<String>();
///             self.0.push_str(format!(
///                 "<h{level}><a id=\"{0}\" href=\"#{0}\">",
///                 slugify!(&title)
///             ));
///             for elem in headline.title() {
///                 self.element(elem, ctx);
///             }
///             self.0.push_str(format!("</a></h{level}>"));
///         }
///     }
///
///     forward_handler! {
///         HtmlExport,
///         link text document paragraph section rule comment
///         inline_src inline_call code bold verbatim italic strike underline list list_item
///         special_block quote_block center_block verse_block comment_block example_block export_block
///         source_block babel_call clock cookie radio_target drawer dyn_block fn_def fn_ref macros
///         snippet timestamp target fixed_width org_table org_table_row org_table_cell latex_fragment
///         latex_environment entity line_break superscript subscript keyword property_drawer
///     }
/// }
///
/// let mut handler = SlugifyTitleHandler::default();
/// Org::parse("* hello world!").traverse(&mut handler);
/// assert_eq!(handler.0.finish(), r##"<main><h1><a id="hello-world" href="#hello-world">hello world!</a></h1></main>"##);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! forward_handler {
    ($handler:ty, $($func:ident)*) => {
        $(
            forward_handler!(@method $handler, $func);
        )*
    };

    (@method $handler:ty, text) => {
        forward_handler!(@method $handler, text, $crate::SyntaxToken);
    };
    (@method $handler:ty, document) => {
        forward_handler!(@method $handler, document, WalkEvent<&$crate::ast::Document>);
    };
    (@method $handler:ty, headline) => {
        forward_handler!(@method $handler, headline, WalkEvent<&$crate::ast::Headline>);
    };
    (@method $handler:ty, paragraph) => {
        forward_handler!(@method $handler, paragraph, WalkEvent<&$crate::ast::Paragraph>);
    };
    (@method $handler:ty, section) => {
        forward_handler!(@method $handler, section, WalkEvent<&$crate::ast::Section>);
    };
    (@method $handler:ty, rule) => {
        forward_handler!(@method $handler, rule, WalkEvent<&$crate::ast::Rule>);
    };
    (@method $handler:ty, comment) => {
        forward_handler!(@method $handler, comment, WalkEvent<&$crate::ast::Comment>);
    };
    (@method $handler:ty, inline_src) => {
        forward_handler!(@method $handler, inline_src, WalkEvent<&$crate::ast::InlineSrc>);
    };
    (@method $handler:ty, inline_call) => {
        forward_handler!(@method $handler, inline_call, WalkEvent<&$crate::ast::InlineCall>);
    };
    (@method $handler:ty, code) => {
        forward_handler!(@method $handler, code, WalkEvent<&$crate::ast::Code>);
    };
    (@method $handler:ty, bold) => {
        forward_handler!(@method $handler, bold, WalkEvent<&$crate::ast::Bold>);
    };
    (@method $handler:ty, verbatim) => {
        forward_handler!(@method $handler, verbatim, WalkEvent<&$crate::ast::Verbatim>);
    };
    (@method $handler:ty, italic) => {
        forward_handler!(@method $handler, italic, WalkEvent<&$crate::ast::Italic>);
    };
    (@method $handler:ty, strike) => {
        forward_handler!(@method $handler, strike, WalkEvent<&$crate::ast::Strike>);
    };
    (@method $handler:ty, underline) => {
        forward_handler!(@method $handler, underline, WalkEvent<&$crate::ast::Underline>);
    };
    (@method $handler:ty, list) => {
        forward_handler!(@method $handler, list, WalkEvent<&$crate::ast::List>);
    };
    (@method $handler:ty, list_item) => {
        forward_handler!(@method $handler, list_item, WalkEvent<&$crate::ast::ListItem>);
    };
    (@method $handler:ty, special_block) => {
        forward_handler!(@method $handler, special_block, WalkEvent<&$crate::ast::SpecialBlock>);
    };
    (@method $handler:ty, quote_block) => {
        forward_handler!(@method $handler, quote_block, WalkEvent<&$crate::ast::QuoteBlock>);
    };
    (@method $handler:ty, center_block) => {
        forward_handler!(@method $handler, center_block, WalkEvent<&$crate::ast::CenterBlock>);
    };
    (@method $handler:ty, verse_block) => {
        forward_handler!(@method $handler, verse_block, WalkEvent<&$crate::ast::VerseBlock>);
    };
    (@method $handler:ty, comment_block) => {
        forward_handler!(@method $handler, comment_block, WalkEvent<&$crate::ast::CommentBlock>);
    };
    (@method $handler:ty, example_block) => {
        forward_handler!(@method $handler, example_block, WalkEvent<&$crate::ast::ExampleBlock>);
    };
    (@method $handler:ty, export_block) => {
        forward_handler!(@method $handler, export_block, WalkEvent<&$crate::ast::ExportBlock>);
    };
    (@method $handler:ty, source_block) => {
        forward_handler!(@method $handler, source_block, WalkEvent<&$crate::ast::SourceBlock>);
    };
    (@method $handler:ty, babel_call) => {
        forward_handler!(@method $handler, babel_call, WalkEvent<&$crate::ast::BabelCall>);
    };
    (@method $handler:ty, clock) => {
        forward_handler!(@method $handler, clock, WalkEvent<&$crate::ast::Clock>);
    };
    (@method $handler:ty, cookie) => {
        forward_handler!(@method $handler, cookie, WalkEvent<&$crate::ast::Cookie>);
    };
    (@method $handler:ty, radio_target) => {
        forward_handler!(@method $handler, radio_target, WalkEvent<&$crate::ast::RadioTarget>);
    };
    (@method $handler:ty, drawer) => {
        forward_handler!(@method $handler, drawer, WalkEvent<&$crate::ast::Drawer>);
    };
    (@method $handler:ty, dyn_block) => {
        forward_handler!(@method $handler, dyn_block, WalkEvent<&$crate::ast::DynBlock>);
    };
    (@method $handler:ty, fn_def) => {
        forward_handler!(@method $handler, fn_def, WalkEvent<&$crate::ast::FnDef>);
    };
    (@method $handler:ty, fn_ref) => {
        forward_handler!(@method $handler, fn_ref, WalkEvent<&$crate::ast::FnRef>);
    };
    (@method $handler:ty, macros) => {
        forward_handler!(@method $handler, macros, WalkEvent<&$crate::ast::Macros>);
    };
    (@method $handler:ty, snippet) => {
        forward_handler!(@method $handler, snippet, WalkEvent<&$crate::ast::Snippet>);
    };
    (@method $handler:ty, timestamp) => {
        forward_handler!(@method $handler, timestamp, WalkEvent<&$crate::ast::Timestamp>);
    };
    (@method $handler:ty, target) => {
        forward_handler!(@method $handler, target, WalkEvent<&$crate::ast::Target>);
    };
    (@method $handler:ty, fixed_width) => {
        forward_handler!(@method $handler, fixed_width, WalkEvent<&$crate::ast::FixedWidth>);
    };
    (@method $handler:ty, org_table) => {
        forward_handler!(@method $handler, org_table, WalkEvent<&$crate::ast::OrgTable>);
    };
    (@method $handler:ty, org_table_row) => {
        forward_handler!(@method $handler, org_table_row, WalkEvent<&$crate::ast::OrgTableRow>);
    };
    (@method $handler:ty, org_table_cell) => {
        forward_handler!(@method $handler, org_table_cell, WalkEvent<&$crate::ast::OrgTableCell>);
    };
    (@method $handler:ty, link) => {
        forward_handler!(@method $handler, link, WalkEvent<&$crate::ast::Link>);
    };
    (@method $handler:ty, latex_fragment) => {
        forward_handler!(@method $handler, latex_fragment, WalkEvent<&$crate::ast::LatexFragment>);
    };
    (@method $handler:ty, latex_environment) => {
        forward_handler!(@method $handler, latex_environment, WalkEvent<&$crate::ast::LatexEnvironment>);
    };
    (@method $handler:ty, entity) => {
        forward_handler!(@method $handler, entity, WalkEvent<&$crate::ast::Entity>);
    };
    (@method $handler:ty, line_break) => {
        forward_handler!(@method $handler, line_break, WalkEvent<&$crate::ast::LineBreak>);
    };
    (@method $handler:ty, superscript) => {
        forward_handler!(@method $handler, superscript, WalkEvent<&$crate::ast::Superscript>);
    };
    (@method $handler:ty, subscript) => {
        forward_handler!(@method $handler, subscript, WalkEvent<&$crate::ast::Subscript>);
    };
    (@method $handler:ty, keyword) => {
        forward_handler!(@method $handler, keyword, WalkEvent<&$crate::ast::Keyword>);
    };
    (@method $handler:ty, property_drawer) => {
        forward_handler!(@method $handler, property_drawer, WalkEvent<&$crate::ast::PropertyDrawer>);
    };
    (@method $handler:ty, $x:ident) => {
        std::compile_error!(std::concat!(std::stringify!($x), " is not a method"));
    };

    (@method $handler:ty, $name:ident, $type:ty) => {
        fn $name(&mut self, item: $type, ctx: &mut $crate::export::TraversalContext) {
            <Self as AsMut<$handler>>::as_mut(self).$name(item, ctx)
        }
    };
}
