mod html;

pub use self::html::HtmlHandler;

use crate::parser::Parser;
use std::io::{Result, Write};

macro_rules! create_render {
    ($handler:ident, $default_handler:ident, $render:ident, $default_render:ident) => {
        struct $default_handler;

        impl<W: Write> $handler<W> for $default_handler {}

        pub struct $default_render<'a, W: Write>($render<'a, W, $default_handler>);

        impl<'a, W: Write> $default_render<'a, W> {
            #[inline]
            pub fn new(writer: W, text: &'a str) -> Self {
                $default_render($render::new($default_handler, writer, text))
            }

            #[inline]
            pub fn into_wirter(self) -> W {
                self.0.writer
            }

            #[inline]
            pub fn render(&mut self) -> Result<()> {
                self.0.render()
            }
        }

        pub struct $render<'a, W: Write, H: $handler<W>> {
            pub parser: Parser<'a>,
            handler: H,
            writer: W,
        }

        impl<'a, W: Write, H: $handler<W>> $render<'a, W, H> {
            pub fn new(handler: H, writer: W, text: &'a str) -> Self {
                $render {
                    parser: Parser::new(text),
                    handler,
                    writer,
                }
            }

            pub fn into_wirter(self) -> W {
                self.writer
            }

            pub fn render(&mut self) -> Result<()> {
                use crate::parser::Event::*;

                let w = &mut self.writer;
                let h = &mut self.handler;

                for event in &mut self.parser {
                    match event {
                        HeadlineBeg(hdl) => h.handle_headline_beg(w, hdl)?,
                        HeadlineEnd => h.handle_headline_end(w)?,
                        SectionBeg => h.handle_section_beg(w)?,
                        SectionEnd => h.handle_section_end(w)?,
                        ParagraphBeg => h.handle_paragraph_beg(w)?,
                        ParagraphEnd => h.handle_paragraph_end(w)?,
                        CtrBlockBeg => h.handle_ctr_block_beg(w)?,
                        CtrBlockEnd => h.handle_ctr_block_end(w)?,
                        QteBlockBeg => h.handle_qte_block_beg(w)?,
                        QteBlockEnd => h.handle_qte_block_end(w)?,
                        SplBlockBeg { name, args } => h.handle_spl_block_beg(w, name, args)?,
                        SplBlockEnd => h.handle_spl_block_end(w)?,
                        CommentBlock { cont, args } => h.handle_comment_block(w, cont, args)?,
                        ExampleBlock { cont, args } => h.handle_example_block(w, cont, args)?,
                        ExportBlock { cont, args } => h.handle_export_block(w, cont, args)?,
                        SrcBlock { cont, args } => h.handle_src_block(w, cont, args)?,
                        VerseBlock { cont, args } => h.handle_verse_block(w, cont, args)?,
                        DynBlockBeg { name, args } => h.handle_dyn_block_beg(w, name, args)?,
                        DynBlockEnd => h.handle_dyn_block_end(w)?,
                        ListBeg { ordered } => h.handle_list_beg(w, ordered)?,
                        ListEnd { ordered } => h.handle_list_end(w, ordered)?,
                        ListItemBeg { bullet } => h.handle_list_beg_item(w, bullet)?,
                        ListItemEnd => h.handle_list_end_item(w)?,
                        Call { value } => h.handle_call(w, value)?,
                        Clock => h.handle_clock(w)?,
                        Comment(c) => h.handle_comment(w, c)?,
                        FixedWidth(f) => h.handle_fixed_width(w, f)?,
                        TableStart => h.handle_table_start(w)?,
                        TableEnd => h.handle_table_end(w)?,
                        TableCell => h.handle_table_cell(w)?,
                        LatexEnv => h.handle_latex_env(w)?,
                        FnDef { label, cont } => h.handle_fn_def(w, label, cont)?,
                        Keyword { key, value } => h.handle_keyword(w, key, value)?,
                        Rule => h.handle_rule(w)?,
                        Cookie(cookie) => h.handle_cookie(w, cookie)?,
                        FnRef { label, def } => h.handle_fn_ref(w, label, def)?,
                        InlineSrc { lang, option, body } => {
                            h.handle_inline_src(w, lang, option, body)?
                        }
                        InlineCall {
                            name,
                            args,
                            inside_header,
                            end_header,
                        } => h.handle_inline_call(w, name, args, inside_header, end_header)?,
                        Link { path, desc } => h.handle_link(w, path, desc)?,
                        Macros { name, args } => h.handle_macros(w, name, args)?,
                        RadioTarget { target } => h.handle_radio_target(w, target)?,
                        Snippet { name, value } => h.handle_snippet(w, name, value)?,
                        Target { target } => h.handle_target(w, target)?,
                        BoldBeg => h.handle_bold_beg(w)?,
                        BoldEnd => h.handle_bold_end(w)?,
                        ItalicBeg => h.handle_italic_beg(w)?,
                        ItalicEnd => h.handle_italic_end(w)?,
                        StrikeBeg => h.handle_strike_beg(w)?,
                        StrikeEnd => h.handle_strike_end(w)?,
                        UnderlineBeg => h.handle_underline_beg(w)?,
                        UnderlineEnd => h.handle_underline_end(w)?,
                        Verbatim(cont) => h.handle_verbatim(w, cont)?,
                        Code(cont) => h.handle_code(w, cont)?,
                        Text(cont) => h.handle_text(w, cont)?,
                    }
                }

                Ok(())
            }
        }
    };
}

create_render!(
    HtmlHandler,
    DefaultHtmlHandller,
    HtmlRender,
    DefaultHtmlRender
);
