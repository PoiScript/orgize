mod html;

pub use self::html::HtmlHandler;

use headline::Headline;
use objects::{Cookie, FnRef, InlineCall, InlineSrc, Link, Macros, RadioTarget, Snippet, Target};
use parser::Parser;
use std::io::{Result, Write};

pub trait Handler<W: Write> {
    fn handle_start_headline(&mut self, w: &mut W, hdl: Headline) -> Result<()>;
    fn handle_end_headline(&mut self, w: &mut W) -> Result<()>;
    fn handle_start_section(&mut self, w: &mut W) -> Result<()>;
    fn handle_end_section(&mut self, w: &mut W) -> Result<()>;
    fn handle_start_paragraph(&mut self, w: &mut W) -> Result<()>;
    fn handle_end_paragraph(&mut self, w: &mut W) -> Result<()>;
    fn handle_start_center_block(&mut self, w: &mut W) -> Result<()>;
    fn handle_end_center_block(&mut self, w: &mut W) -> Result<()>;
    fn handle_start_quote_block(&mut self, w: &mut W) -> Result<()>;
    fn handle_end_quote_block(&mut self, w: &mut W) -> Result<()>;
    fn handle_start_special_block(
        &mut self,
        w: &mut W,
        name: &str,
        args: Option<&str>,
    ) -> Result<()>;
    fn handle_end_special_block(&mut self, w: &mut W) -> Result<()>;
    fn handle_comment_block(&mut self, w: &mut W, contents: &str, args: Option<&str>)
        -> Result<()>;
    fn handle_example_block(&mut self, w: &mut W, contents: &str, args: Option<&str>)
        -> Result<()>;
    fn handle_export_block(&mut self, w: &mut W, contents: &str, args: Option<&str>) -> Result<()>;
    fn handle_src_block(&mut self, w: &mut W, contents: &str, args: Option<&str>) -> Result<()>;
    fn handle_verse_block(&mut self, w: &mut W, contents: &str, args: Option<&str>) -> Result<()>;
    fn handle_start_dyn_block(&mut self, w: &mut W, name: &str, args: Option<&str>) -> Result<()>;
    fn handle_end_dyn_block(&mut self, w: &mut W) -> Result<()>;
    fn handle_list_start(&mut self, w: &mut W) -> Result<()>;
    fn handle_list_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_aff_keywords(&mut self, w: &mut W) -> Result<()>;
    fn handle_call(&mut self, w: &mut W) -> Result<()>;
    fn handle_clock(&mut self, w: &mut W) -> Result<()>;
    fn handle_comment(&mut self, w: &mut W, contents: &str) -> Result<()>;
    fn handle_table_start(&mut self, w: &mut W) -> Result<()>;
    fn handle_table_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_table_cell(&mut self, w: &mut W) -> Result<()>;
    fn handle_latex_env(&mut self, w: &mut W) -> Result<()>;
    fn handle_fn_def(&mut self, w: &mut W, label: &str, contents: &str) -> Result<()>;
    fn handle_keyword(&mut self, w: &mut W, key: &str, value: &str) -> Result<()>;
    fn handle_rule(&mut self, w: &mut W) -> Result<()>;
    fn handle_cookie(&mut self, w: &mut W, cookie: Cookie) -> Result<()>;
    fn handle_fn_ref(&mut self, w: &mut W, fn_ref: FnRef) -> Result<()>;
    fn handle_inline_call(&mut self, w: &mut W, inline_call: InlineCall) -> Result<()>;
    fn handle_inline_src(&mut self, w: &mut W, inline_src: InlineSrc) -> Result<()>;
    fn handle_link(&mut self, w: &mut W, link: Link) -> Result<()>;
    fn handle_macros(&mut self, w: &mut W, macros: Macros) -> Result<()>;
    fn handle_radio_target(&mut self, w: &mut W, target: RadioTarget) -> Result<()>;
    fn handle_snippet(&mut self, w: &mut W, snippet: Snippet) -> Result<()>;
    fn handle_target(&mut self, w: &mut W, target: Target) -> Result<()>;
    fn handle_start_bold(&mut self, w: &mut W) -> Result<()>;
    fn handle_end_bold(&mut self, w: &mut W) -> Result<()>;
    fn handle_start_italic(&mut self, w: &mut W) -> Result<()>;
    fn handle_end_italic(&mut self, w: &mut W) -> Result<()>;
    fn handle_start_strike(&mut self, w: &mut W) -> Result<()>;
    fn handle_end_strike(&mut self, w: &mut W) -> Result<()>;
    fn handle_start_underline(&mut self, w: &mut W) -> Result<()>;
    fn handle_end_underline(&mut self, w: &mut W) -> Result<()>;
    fn handle_verbatim(&mut self, w: &mut W, contents: &str) -> Result<()>;
    fn handle_code(&mut self, w: &mut W, contents: &str) -> Result<()>;
    fn handle_text(&mut self, w: &mut W, contents: &str) -> Result<()>;
}

pub struct Render<'a, W: Write, H: Handler<W>> {
    pub parser: Parser<'a>,
    pub handler: H,
    writer: W,
}

impl<'a, W: Write, H: Handler<W>> Render<'a, W, H> {
    pub fn new(handler: H, writer: W, text: &'a str) -> Render<'a, W, H> {
        Render {
            parser: Parser::new(text),
            handler,
            writer,
        }
    }

    pub fn into_wirter(self) -> W {
        self.writer
    }

    pub fn render(&mut self) -> Result<()> {
        use parser::Event::*;

        for event in &mut self.parser {
            match event {
                StartHeadline(hdl) => self.handler.handle_start_headline(&mut self.writer, hdl)?,
                EndHeadline => self.handler.handle_end_headline(&mut self.writer)?,
                StartSection => self.handler.handle_start_section(&mut self.writer)?,
                EndSection => self.handler.handle_end_section(&mut self.writer)?,
                StartParagraph => self.handler.handle_start_paragraph(&mut self.writer)?,
                EndParagraph => self.handler.handle_end_paragraph(&mut self.writer)?,
                StartCenterBlock => self.handler.handle_start_center_block(&mut self.writer)?,
                EndCenterBlock => self.handler.handle_end_center_block(&mut self.writer)?,
                StartQuoteBlock => self.handler.handle_start_quote_block(&mut self.writer)?,
                EndQuoteBlock => self.handler.handle_end_quote_block(&mut self.writer)?,
                StartSpecialBlock { name, args } => {
                    self.handler
                        .handle_start_special_block(&mut self.writer, name, args)?
                }
                EndSpecialBlock => self.handler.handle_end_special_block(&mut self.writer)?,
                CommentBlock { contents, args } => {
                    self.handler
                        .handle_comment_block(&mut self.writer, contents, args)?
                }
                ExampleBlock { contents, args } => {
                    self.handler
                        .handle_example_block(&mut self.writer, contents, args)?
                }
                ExportBlock { contents, args } => {
                    self.handler
                        .handle_export_block(&mut self.writer, contents, args)?
                }
                SrcBlock { contents, args } => {
                    self.handler
                        .handle_src_block(&mut self.writer, contents, args)?
                }
                VerseBlock { contents, args } => {
                    self.handler
                        .handle_verse_block(&mut self.writer, contents, args)?
                }
                StartDynBlock { name, args } => {
                    self.handler
                        .handle_start_dyn_block(&mut self.writer, name, args)?
                }
                EndDynBlock => self.handler.handle_end_dyn_block(&mut self.writer)?,
                ListStart => self.handler.handle_list_start(&mut self.writer)?,
                ListEnd => self.handler.handle_list_end(&mut self.writer)?,
                AffKeywords => self.handler.handle_aff_keywords(&mut self.writer)?,
                Call => self.handler.handle_call(&mut self.writer)?,
                Clock => self.handler.handle_clock(&mut self.writer)?,
                Comment(c) => self.handler.handle_comment(&mut self.writer, c)?,
                TableStart => self.handler.handle_table_start(&mut self.writer)?,
                TableEnd => self.handler.handle_table_end(&mut self.writer)?,
                TableCell => self.handler.handle_table_cell(&mut self.writer)?,
                LatexEnv => self.handler.handle_latex_env(&mut self.writer)?,
                FnDef { label, contents } => {
                    self.handler
                        .handle_fn_def(&mut self.writer, label, contents)?
                }
                Keyword { key, value } => {
                    self.handler.handle_keyword(&mut self.writer, key, value)?
                }
                Rule => self.handler.handle_rule(&mut self.writer)?,
                Cookie(cookie) => self.handler.handle_cookie(&mut self.writer, cookie)?,
                FnRef(fnref) => self.handler.handle_fn_ref(&mut self.writer, fnref)?,
                InlineCall(inlinecall) => self
                    .handler
                    .handle_inline_call(&mut self.writer, inlinecall)?,
                InlineSrc(inlinesrc) => self
                    .handler
                    .handle_inline_src(&mut self.writer, inlinesrc)?,
                Link(link) => self.handler.handle_link(&mut self.writer, link)?,
                Macros(macros) => self.handler.handle_macros(&mut self.writer, macros)?,
                RadioTarget(radiotarget) => self
                    .handler
                    .handle_radio_target(&mut self.writer, radiotarget)?,
                Snippet(snippet) => self.handler.handle_snippet(&mut self.writer, snippet)?,
                Target(target) => self.handler.handle_target(&mut self.writer, target)?,
                StartBold => self.handler.handle_start_bold(&mut self.writer)?,
                EndBold => self.handler.handle_end_bold(&mut self.writer)?,
                StartItalic => self.handler.handle_start_italic(&mut self.writer)?,
                EndItalic => self.handler.handle_end_italic(&mut self.writer)?,
                StartStrike => self.handler.handle_start_strike(&mut self.writer)?,
                EndStrike => self.handler.handle_end_strike(&mut self.writer)?,
                StartUnderline => self.handler.handle_start_underline(&mut self.writer)?,
                EndUnderline => self.handler.handle_end_underline(&mut self.writer)?,
                Verbatim(contents) => self.handler.handle_verbatim(&mut self.writer, contents)?,
                Code(contents) => self.handler.handle_code(&mut self.writer, contents)?,
                Text(contents) => self.handler.handle_text(&mut self.writer, contents)?,
            }
        }

        Ok(())
    }
}
