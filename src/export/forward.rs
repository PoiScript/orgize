/// Forward handler method implement to other handler
///
/// This macros is commonly used if you want to extend
/// some builtin handlers like HtmlExport.
///
/// ```rust
/// use orgize::{
///     ast::HeadlineTitle,
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
///     fn headline_title(&mut self, event: WalkEvent<&HeadlineTitle>, _ctx: &mut TraversalContext) {
///         match event {
///             WalkEvent::Enter(title) => {
///                 let level = title.headline().and_then(|h| h.level()).unwrap_or(1);
///                 let level = min(level, 6);
///                 let raw = title.syntax().to_string();
///                 self.0.output += &format!("<h{level}><a id=\"{0}\" href=\"#{0}\">", slugify!(&raw));
///             }
///             WalkEvent::Leave(title) => {
///                 let level = title.headline().and_then(|h| h.level()).unwrap_or(1);
///                 let level = min(level, 6);
///                 self.0.output += &format!("</a></h{level}>");
///             }
///         }
///     }
///
///     forward_handler! {
///         HtmlExport,
///         link text document headline paragraph section rule comment
///         inline_src inline_call code bold verbatim italic strike underline list list_item list_item_tag
///         special_block quote_block center_block verse_block comment_block example_block export_block
///         source_block babel_call clock cookie radio_target drawer dyn_block fn_def fn_ref macros
///         snippet timestamp target fixed_width org_table org_table_row org_table_cell list_item_content
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
        forward_handler!(@method $handler, text, ::orgize::SyntaxToken);
    };
    (@method $handler:ty, document) => {
        forward_handler!(@method $handler, document, WalkEvent<&::orgize::ast::Document>);
    };
    (@method $handler:ty, headline) => {
        forward_handler!(@method $handler, headline, WalkEvent<&::orgize::ast::Headline>);
    };
    (@method $handler:ty, paragraph) => {
        forward_handler!(@method $handler, paragraph, WalkEvent<&::orgize::ast::Paragraph>);
    };
    (@method $handler:ty, section) => {
        forward_handler!(@method $handler, section, WalkEvent<&::orgize::ast::Section>);
    };
    (@method $handler:ty, rule) => {
        forward_handler!(@method $handler, rule, WalkEvent<&::orgize::ast::Rule>);
    };
    (@method $handler:ty, comment) => {
        forward_handler!(@method $handler, comment, WalkEvent<&::orgize::ast::Comment>);
    };
    (@method $handler:ty, inline_src) => {
        forward_handler!(@method $handler, inline_src, WalkEvent<&::orgize::ast::InlineSrc>);
    };
    (@method $handler:ty, inline_call) => {
        forward_handler!(@method $handler, inline_call, WalkEvent<&::orgize::ast::InlineCall>);
    };
    (@method $handler:ty, code) => {
        forward_handler!(@method $handler, code, WalkEvent<&::orgize::ast::Code>);
    };
    (@method $handler:ty, bold) => {
        forward_handler!(@method $handler, bold, WalkEvent<&::orgize::ast::Bold>);
    };
    (@method $handler:ty, verbatim) => {
        forward_handler!(@method $handler, verbatim, WalkEvent<&::orgize::ast::Verbatim>);
    };
    (@method $handler:ty, italic) => {
        forward_handler!(@method $handler, italic, WalkEvent<&::orgize::ast::Italic>);
    };
    (@method $handler:ty, strike) => {
        forward_handler!(@method $handler, strike, WalkEvent<&::orgize::ast::Strike>);
    };
    (@method $handler:ty, underline) => {
        forward_handler!(@method $handler, underline, WalkEvent<&::orgize::ast::Underline>);
    };
    (@method $handler:ty, list) => {
        forward_handler!(@method $handler, list, WalkEvent<&::orgize::ast::List>);
    };
    (@method $handler:ty, list_item) => {
        forward_handler!(@method $handler, list_item, WalkEvent<&::orgize::ast::ListItem>);
    };
    (@method $handler:ty, list_item_tag) => {
        forward_handler!(@method $handler, list_item_tag, WalkEvent<&::orgize::ast::ListItemTag>);
    };
    (@method $handler:ty, list_item_content) => {
        forward_handler!(@method $handler, list_item_content, WalkEvent<&::orgize::ast::ListItemContent>);
    };
    (@method $handler:ty, special_block) => {
        forward_handler!(@method $handler, special_block, WalkEvent<&::orgize::ast::SpecialBlock>);
    };
    (@method $handler:ty, quote_block) => {
        forward_handler!(@method $handler, quote_block, WalkEvent<&::orgize::ast::QuoteBlock>);
    };
    (@method $handler:ty, center_block) => {
        forward_handler!(@method $handler, center_block, WalkEvent<&::orgize::ast::CenterBlock>);
    };
    (@method $handler:ty, verse_block) => {
        forward_handler!(@method $handler, verse_block, WalkEvent<&::orgize::ast::VerseBlock>);
    };
    (@method $handler:ty, comment_block) => {
        forward_handler!(@method $handler, comment_block, WalkEvent<&::orgize::ast::CommentBlock>);
    };
    (@method $handler:ty, example_block) => {
        forward_handler!(@method $handler, example_block, WalkEvent<&::orgize::ast::ExampleBlock>);
    };
    (@method $handler:ty, export_block) => {
        forward_handler!(@method $handler, export_block, WalkEvent<&::orgize::ast::ExportBlock>);
    };
    (@method $handler:ty, source_block) => {
        forward_handler!(@method $handler, source_block, WalkEvent<&::orgize::ast::SourceBlock>);
    };
    (@method $handler:ty, babel_call) => {
        forward_handler!(@method $handler, babel_call, WalkEvent<&::orgize::ast::BabelCall>);
    };
    (@method $handler:ty, clock) => {
        forward_handler!(@method $handler, clock, WalkEvent<&::orgize::ast::Clock>);
    };
    (@method $handler:ty, cookie) => {
        forward_handler!(@method $handler, cookie, WalkEvent<&::orgize::ast::Cookie>);
    };
    (@method $handler:ty, radio_target) => {
        forward_handler!(@method $handler, radio_target, WalkEvent<&::orgize::ast::RadioTarget>);
    };
    (@method $handler:ty, drawer) => {
        forward_handler!(@method $handler, drawer, WalkEvent<&::orgize::ast::Drawer>);
    };
    (@method $handler:ty, dyn_block) => {
        forward_handler!(@method $handler, dyn_block, WalkEvent<&::orgize::ast::DynBlock>);
    };
    (@method $handler:ty, fn_def) => {
        forward_handler!(@method $handler, fn_def, WalkEvent<&::orgize::ast::FnDef>);
    };
    (@method $handler:ty, fn_ref) => {
        forward_handler!(@method $handler, fn_ref, WalkEvent<&::orgize::ast::FnRef>);
    };
    (@method $handler:ty, macros) => {
        forward_handler!(@method $handler, macros, WalkEvent<&::orgize::ast::Macros>);
    };
    (@method $handler:ty, snippet) => {
        forward_handler!(@method $handler, snippet, WalkEvent<&::orgize::ast::Snippet>);
    };
    (@method $handler:ty, timestamp) => {
        forward_handler!(@method $handler, timestamp, WalkEvent<&::orgize::ast::Timestamp>);
    };
    (@method $handler:ty, target) => {
        forward_handler!(@method $handler, target, WalkEvent<&::orgize::ast::Target>);
    };
    (@method $handler:ty, fixed_width) => {
        forward_handler!(@method $handler, fixed_width, WalkEvent<&::orgize::ast::FixedWidth>);
    };
    (@method $handler:ty, headline_title) => {
        forward_handler!(@method $handler, headline_title, WalkEvent<&::orgize::ast::HeadlineTitle>);
    };
    (@method $handler:ty, org_table) => {
        forward_handler!(@method $handler, org_table, WalkEvent<&::orgize::ast::OrgTable>);
    };
    (@method $handler:ty, org_table_row) => {
        forward_handler!(@method $handler, org_table_row, WalkEvent<&::orgize::ast::OrgTableRow>);
    };
    (@method $handler:ty, org_table_cell) => {
        forward_handler!(@method $handler, org_table_cell, WalkEvent<&::orgize::ast::OrgTableCell>);
    };
    (@method $handler:ty, link) => {
        forward_handler!(@method $handler, link, WalkEvent<&::orgize::ast::Link>);
    };
    (@method $handler:ty, $x:ident) => {
        std::compile_error!(std::concat!(std::stringify!($x), " is not a method"));
    };

    (@method $handler:ty, $name:ident, $type:ty) => {
        fn $name(&mut self, item: $type, ctx: &mut ::orgize::export::TraversalContext) {
            <Self as AsMut<$handler>>::as_mut(self).$name(item, ctx)
        }
    };
}
