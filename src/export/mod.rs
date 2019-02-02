mod html;

pub use self::html::HtmlHandler;

use elements::Key;
use headline::Headline;
use objects::{Cookie, FnRef, InlineCall, InlineSrc, Link, Macros, RadioTarget, Snippet, Target};
use parser::Parser;
use std::io::{Result, Write};

pub trait Handler<W: Write> {
    fn handle_headline_beg(&mut self, w: &mut W, hdl: Headline) -> Result<()>;
    fn handle_headline_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_section_beg(&mut self, w: &mut W) -> Result<()>;
    fn handle_section_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_paragraph_beg(&mut self, w: &mut W) -> Result<()>;
    fn handle_paragraph_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_ctr_block_beg(&mut self, w: &mut W) -> Result<()>;
    fn handle_ctr_block_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_qte_block_beg(&mut self, w: &mut W) -> Result<()>;
    fn handle_qte_block_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_spl_block_beg(&mut self, w: &mut W, name: &str, args: Option<&str>) -> Result<()>;
    fn handle_spl_block_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_comment_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<()>;
    fn handle_example_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<()>;
    fn handle_export_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<()>;
    fn handle_src_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<()>;
    fn handle_verse_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<()>;
    fn handle_dyn_block_beg(&mut self, w: &mut W, name: &str, args: Option<&str>) -> Result<()>;
    fn handle_dyn_block_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_list_beg(&mut self, w: &mut W, ordered: bool) -> Result<()>;
    fn handle_list_end(&mut self, w: &mut W, ordered: bool) -> Result<()>;
    fn handle_list_beg_item(&mut self, w: &mut W, bullet: &str) -> Result<()>;
    fn handle_list_end_item(&mut self, w: &mut W) -> Result<()>;
    fn handle_call(&mut self, w: &mut W, value: &str) -> Result<()>;
    fn handle_clock(&mut self, w: &mut W) -> Result<()>;
    fn handle_comment(&mut self, w: &mut W, cont: &str) -> Result<()>;
    fn handle_fixed_width(&mut self, w: &mut W, cont: &str) -> Result<()>;
    fn handle_table_start(&mut self, w: &mut W) -> Result<()>;
    fn handle_table_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_table_cell(&mut self, w: &mut W) -> Result<()>;
    fn handle_latex_env(&mut self, w: &mut W) -> Result<()>;
    fn handle_fn_def(&mut self, w: &mut W, label: &str, cont: &str) -> Result<()>;
    fn handle_keyword(&mut self, w: &mut W, key: Key<'_>, value: &str) -> Result<()>;
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
    fn handle_bold_beg(&mut self, w: &mut W) -> Result<()>;
    fn handle_bold_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_italic_beg(&mut self, w: &mut W) -> Result<()>;
    fn handle_italic_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_strike_beg(&mut self, w: &mut W) -> Result<()>;
    fn handle_strike_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_underline_beg(&mut self, w: &mut W) -> Result<()>;
    fn handle_underline_end(&mut self, w: &mut W) -> Result<()>;
    fn handle_verbatim(&mut self, w: &mut W, cont: &str) -> Result<()>;
    fn handle_code(&mut self, w: &mut W, cont: &str) -> Result<()>;
    fn handle_text(&mut self, w: &mut W, cont: &str) -> Result<()>;
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
                FnRef(fnref) => h.handle_fn_ref(w, fnref)?,
                InlineCall(inlinecall) => h.handle_inline_call(w, inlinecall)?,
                InlineSrc(inlinesrc) => h.handle_inline_src(w, inlinesrc)?,
                Link(link) => h.handle_link(w, link)?,
                Macros(macros) => h.handle_macros(w, macros)?,
                RadioTarget(radiotarget) => h.handle_radio_target(w, radiotarget)?,
                Snippet(snippet) => h.handle_snippet(w, snippet)?,
                Target(target) => h.handle_target(w, target)?,
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
