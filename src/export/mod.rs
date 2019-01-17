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
    fn handle_start_list(&mut self, w: &mut W, is_ordered: bool) -> Result<()>;
    fn handle_end_list(&mut self, w: &mut W, is_ordered: bool) -> Result<()>;
    fn handle_start_list_item(&mut self, w: &mut W) -> Result<()>;
    fn handle_end_list_item(&mut self, w: &mut W) -> Result<()>;
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

        let w = &mut self.writer;

        for event in &mut self.parser {
            match event {
                StartHeadline(hdl) => self.handler.handle_start_headline(w, hdl)?,
                EndHeadline => self.handler.handle_end_headline(w)?,
                StartSection => self.handler.handle_start_section(w)?,
                EndSection => self.handler.handle_end_section(w)?,
                StartParagraph => self.handler.handle_start_paragraph(w)?,
                EndParagraph => self.handler.handle_end_paragraph(w)?,
                StartCenterBlock => self.handler.handle_start_center_block(w)?,
                EndCenterBlock => self.handler.handle_end_center_block(w)?,
                StartQuoteBlock => self.handler.handle_start_quote_block(w)?,
                EndQuoteBlock => self.handler.handle_end_quote_block(w)?,
                StartSpecialBlock { name, args } => {
                    self.handler.handle_start_special_block(w, name, args)?
                }
                EndSpecialBlock => self.handler.handle_end_special_block(w)?,
                CommentBlock { contents, args } => {
                    self.handler.handle_comment_block(w, contents, args)?
                }
                ExampleBlock { contents, args } => {
                    self.handler.handle_example_block(w, contents, args)?
                }
                ExportBlock { contents, args } => {
                    self.handler.handle_export_block(w, contents, args)?
                }
                SrcBlock { contents, args } => self.handler.handle_src_block(w, contents, args)?,
                VerseBlock { contents, args } => {
                    self.handler.handle_verse_block(w, contents, args)?
                }
                StartDynBlock { name, args } => {
                    self.handler.handle_start_dyn_block(w, name, args)?
                }
                EndDynBlock => self.handler.handle_end_dyn_block(w)?,
                StartList { is_ordered } => self.handler.handle_start_list(w, is_ordered)?,
                EndList { is_ordered } => self.handler.handle_end_list(w, is_ordered)?,
                StartListItem => self.handler.handle_start_list_item(w)?,
                EndListItem => self.handler.handle_end_list_item(w)?,
                AffKeywords => self.handler.handle_aff_keywords(w)?,
                Call => self.handler.handle_call(w)?,
                Clock => self.handler.handle_clock(w)?,
                Comment(c) => self.handler.handle_comment(w, c)?,
                TableStart => self.handler.handle_table_start(w)?,
                TableEnd => self.handler.handle_table_end(w)?,
                TableCell => self.handler.handle_table_cell(w)?,
                LatexEnv => self.handler.handle_latex_env(w)?,
                FnDef { label, contents } => self.handler.handle_fn_def(w, label, contents)?,
                Keyword { key, value } => self.handler.handle_keyword(w, key, value)?,
                Rule => self.handler.handle_rule(w)?,
                Cookie(cookie) => self.handler.handle_cookie(w, cookie)?,
                FnRef(fnref) => self.handler.handle_fn_ref(w, fnref)?,
                InlineCall(inlinecall) => self.handler.handle_inline_call(w, inlinecall)?,
                InlineSrc(inlinesrc) => self.handler.handle_inline_src(w, inlinesrc)?,
                Link(link) => self.handler.handle_link(w, link)?,
                Macros(macros) => self.handler.handle_macros(w, macros)?,
                RadioTarget(radiotarget) => self.handler.handle_radio_target(w, radiotarget)?,
                Snippet(snippet) => self.handler.handle_snippet(w, snippet)?,
                Target(target) => self.handler.handle_target(w, target)?,
                StartBold => self.handler.handle_start_bold(w)?,
                EndBold => self.handler.handle_end_bold(w)?,
                StartItalic => self.handler.handle_start_italic(w)?,
                EndItalic => self.handler.handle_end_italic(w)?,
                StartStrike => self.handler.handle_start_strike(w)?,
                EndStrike => self.handler.handle_end_strike(w)?,
                StartUnderline => self.handler.handle_start_underline(w)?,
                EndUnderline => self.handler.handle_end_underline(w)?,
                Verbatim(contents) => self.handler.handle_verbatim(w, contents)?,
                Code(contents) => self.handler.handle_code(w, contents)?,
                Text(contents) => self.handler.handle_text(w, contents)?,
            }
        }

        Ok(())
    }
}
