use elements::{FnDef, Keyword};
use export::Handler;
use headline::Headline;
use objects::{Cookie, FnRef, InlineCall, InlineSrc, Link, Macros, RadioTarget, Snippet, Target};
use std::io::{Result, Write};

pub struct HtmlHandler;

impl<W: Write> Handler<W> for HtmlHandler {
    fn handle_start_headline(&mut self, w: &mut W, hdl: Headline) -> Result<()> {
        write!(
            w,
            "<h{0}>{1}</h{0}>",
            if hdl.level <= 6 { hdl.level } else { 6 },
            hdl.title
        )
    }
    fn handle_end_headline(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_start_section(&mut self, w: &mut W) -> Result<()> {
        write!(w, "<section>")
    }
    fn handle_end_section(&mut self, w: &mut W) -> Result<()> {
        write!(w, "</section>")
    }
    fn handle_start_paragraph(&mut self, w: &mut W) -> Result<()> {
        write!(w, "<p>")
    }
    fn handle_end_paragraph(&mut self, w: &mut W) -> Result<()> {
        write!(w, "</p>")
    }
    fn handle_start_center_block(&mut self, w: &mut W) -> Result<()> {
        write!(w, "<div style=\"text-align: center\">")
    }
    fn handle_end_center_block(&mut self, w: &mut W) -> Result<()> {
        write!(w, "</div>")
    }
    fn handle_start_quote_block(&mut self, w: &mut W) -> Result<()> {
        write!(w, "<blockquote>")
    }
    fn handle_end_quote_block(&mut self, w: &mut W) -> Result<()> {
        write!(w, "</blockquote>")
    }
    fn handle_start_special_block(
        &mut self,
        w: &mut W,
        name: &str,
        args: Option<&str>,
    ) -> Result<()> {
        write!(w, "<div>")
    }
    fn handle_end_special_block(&mut self, w: &mut W) -> Result<()> {
        write!(w, "</div>")
    }
    fn handle_comment_block(&mut self, w: &mut W, content: &str, args: Option<&str>) -> Result<()> {
        Ok(())
    }
    fn handle_example_block(&mut self, w: &mut W, content: &str, args: Option<&str>) -> Result<()> {
        write!(w, "<pre><code>{}</code></pre>", content)
    }
    fn handle_export_block(&mut self, w: &mut W, content: &str, args: Option<&str>) -> Result<()> {
        Ok(())
    }
    fn handle_src_block(&mut self, w: &mut W, content: &str, args: Option<&str>) -> Result<()> {
        write!(w, "<pre><code>{}</code></pre>", content)
    }
    fn handle_verse_block(&mut self, w: &mut W, content: &str, args: Option<&str>) -> Result<()> {
        Ok(())
    }
    fn handle_dyn_block_start(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_dyn_block_end(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_list_start(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_list_end(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_aff_keywords(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_call(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_clock(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_comment(&mut self, w: &mut W, content: &str) -> Result<()> {
        Ok(())
    }
    fn handle_table_start(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_table_end(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_table_cell(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_latex_env(&mut self, w: &mut W) -> Result<()> {
        Ok(())
    }
    fn handle_fn_def(&mut self, w: &mut W, fn_def: FnDef) -> Result<()> {
        Ok(())
    }
    fn handle_keyword(&mut self, w: &mut W, kw: Keyword) -> Result<()> {
        Ok(())
    }
    fn handle_rule(&mut self, w: &mut W) -> Result<()> {
        write!(w, "<hr>")
    }
    fn handle_cookie(&mut self, w: &mut W, cookie: Cookie) -> Result<()> {
        Ok(())
    }
    fn handle_fn_ref(&mut self, w: &mut W, fn_ref: FnRef) -> Result<()> {
        Ok(())
    }
    fn handle_inline_call(&mut self, w: &mut W, inline_call: InlineCall) -> Result<()> {
        Ok(())
    }
    fn handle_inline_src(&mut self, w: &mut W, inline_src: InlineSrc) -> Result<()> {
        write!(w, "<code>{}</code>", inline_src.body)
    }
    fn handle_link(&mut self, w: &mut W, link: Link) -> Result<()> {
        write!(
            w,
            "<a href=\"{}\">{}</a>",
            link.path,
            link.desc.unwrap_or(link.path)
        )
    }
    fn handle_macros(&mut self, w: &mut W, macros: Macros) -> Result<()> {
        Ok(())
    }
    fn handle_radio_target(&mut self, w: &mut W, target: RadioTarget) -> Result<()> {
        Ok(())
    }
    fn handle_snippet(&mut self, w: &mut W, snippet: Snippet) -> Result<()> {
        Ok(())
    }
    fn handle_target(&mut self, w: &mut W, target: Target) -> Result<()> {
        Ok(())
    }
    fn handle_start_bold(&mut self, w: &mut W) -> Result<()> {
        write!(w, "<b>")
    }
    fn handle_end_bold(&mut self, w: &mut W) -> Result<()> {
        write!(w, "</b>")
    }
    fn handle_start_italic(&mut self, w: &mut W) -> Result<()> {
        write!(w, "<i>")
    }
    fn handle_end_italic(&mut self, w: &mut W) -> Result<()> {
        write!(w, "</i>")
    }
    fn handle_start_strike(&mut self, w: &mut W) -> Result<()> {
        write!(w, "<s>")
    }
    fn handle_end_strike(&mut self, w: &mut W) -> Result<()> {
        write!(w, "</s>")
    }
    fn handle_start_underline(&mut self, w: &mut W) -> Result<()> {
        write!(w, "<u>")
    }
    fn handle_end_underline(&mut self, w: &mut W) -> Result<()> {
        write!(w, "</u>")
    }
    fn handle_verbatim(&mut self, w: &mut W, content: &str) -> Result<()> {
        write!(w, "<code>{}</code>", content)
    }
    fn handle_code(&mut self, w: &mut W, content: &str) -> Result<()> {
        write!(w, "<code>{}</code>", content)
    }
    fn handle_text(&mut self, w: &mut W, content: &str) -> Result<()> {
        write!(w, "{}", content.replace('\n', " "))
    }
}
