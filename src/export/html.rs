#![allow(unused_variables)]

use crate::elements::Key;
use crate::headline::Headline;
use crate::objects::Cookie;
use crate::parser::Parser;
use jetscii::ascii_chars;
use std::convert::From;
use std::fmt;
use std::io::{Error, Write};
use std::marker::PhantomData;

pub trait HtmlHandler<W: Write, E: From<Error>> {
    fn handle_headline_beg(&mut self, w: &mut W, hdl: Headline) -> Result<(), E> {
        let level = if hdl.level <= 6 { hdl.level } else { 6 };
        Ok(write!(w, "<h{0}>{1}</h{0}>", level, Escape(hdl.title))?)
    }
    fn handle_headline_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn handle_section_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<section>")?)
    }
    fn handle_section_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</section>")?)
    }
    fn handle_paragraph_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<p>")?)
    }
    fn handle_paragraph_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</p>")?)
    }
    fn handle_ctr_block_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, r#"<div style="text-align: center">"#)?)
    }
    fn handle_ctr_block_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</div>")?)
    }
    fn handle_qte_block_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<blockquote>")?)
    }
    fn handle_qte_block_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</blockquote>")?)
    }
    fn handle_spl_block_beg(&mut self, w: &mut W, name: &str, args: Option<&str>) -> Result<(), E> {
        Ok(write!(w, "<div>")?)
    }
    fn handle_spl_block_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</div>")?)
    }
    fn handle_comment_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn handle_example_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(write!(w, "<pre><code>{}</code></pre>", Escape(cont))?)
    }
    fn handle_export_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn handle_src_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(write!(w, "<pre><code>{}</code></pre>", Escape(cont))?)
    }
    fn handle_verse_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn handle_dyn_block_beg(&mut self, w: &mut W, name: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn handle_dyn_block_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn handle_list_beg(&mut self, w: &mut W, ordered: bool) -> Result<(), E> {
        if ordered {
            Ok(write!(w, "<ol>")?)
        } else {
            Ok(write!(w, "<ul>")?)
        }
    }
    fn handle_list_end(&mut self, w: &mut W, ordered: bool) -> Result<(), E> {
        if ordered {
            Ok(write!(w, "</ol>")?)
        } else {
            Ok(write!(w, "</ul>")?)
        }
    }
    fn handle_list_beg_item(&mut self, w: &mut W, bullet: &str) -> Result<(), E> {
        Ok(write!(w, "<li>")?)
    }
    fn handle_list_end_item(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</li>")?)
    }
    fn handle_call(&mut self, w: &mut W, value: &str) -> Result<(), E> {
        Ok(())
    }
    fn handle_clock(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn handle_comment(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        Ok(())
    }
    fn handle_fixed_width(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        for line in cont.lines() {
            // remove leading colon
            write!(w, "<pre>{}</pre>", Escape(&line[1..]))?;
        }

        Ok(())
    }
    fn handle_table_start(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn handle_table_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn handle_table_cell(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn handle_latex_env(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn handle_fn_def(&mut self, w: &mut W, label: &str, cont: &str) -> Result<(), E> {
        Ok(())
    }
    fn handle_keyword(&mut self, w: &mut W, key: Key<'_>, value: &str) -> Result<(), E> {
        Ok(())
    }
    fn handle_rule(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<hr>")?)
    }
    fn handle_cookie(&mut self, w: &mut W, cookie: Cookie) -> Result<(), E> {
        Ok(())
    }
    fn handle_fn_ref(
        &mut self,
        w: &mut W,
        label: Option<&str>,
        def: Option<&str>,
    ) -> Result<(), E> {
        Ok(())
    }
    fn handle_inline_call(
        &mut self,
        w: &mut W,
        name: &str,
        args: &str,
        inside_header: Option<&str>,
        end_header: Option<&str>,
    ) -> Result<(), E> {
        Ok(())
    }
    fn handle_inline_src(
        &mut self,
        w: &mut W,
        lang: &str,
        option: Option<&str>,
        body: &str,
    ) -> Result<(), E> {
        Ok(write!(w, "<code>{}</code>", Escape(body))?)
    }
    fn handle_link(&mut self, w: &mut W, path: &str, desc: Option<&str>) -> Result<(), E> {
        if let Some(desc) = desc {
            Ok(write!(
                w,
                r#"<a href="{}">{}</a>"#,
                Escape(path),
                Escape(desc)
            )?)
        } else {
            Ok(write!(w, r#"<a href="{0}">{0}</a>"#, Escape(path))?)
        }
    }
    fn handle_macros(&mut self, w: &mut W, name: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn handle_radio_target(&mut self, w: &mut W, target: &str) -> Result<(), E> {
        Ok(())
    }
    fn handle_snippet(&mut self, w: &mut W, name: &str, value: &str) -> Result<(), E> {
        if name.eq_ignore_ascii_case("HTML") {
            Ok(write!(w, "{}", value)?)
        } else {
            Ok(())
        }
    }
    fn handle_target(&mut self, w: &mut W, target: &str) -> Result<(), E> {
        Ok(())
    }
    fn handle_bold_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<b>")?)
    }
    fn handle_bold_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</b>")?)
    }
    fn handle_italic_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<i>")?)
    }
    fn handle_italic_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</i>")?)
    }
    fn handle_strike_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<s>")?)
    }
    fn handle_strike_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</s>")?)
    }
    fn handle_underline_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<u>")?)
    }
    fn handle_underline_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</u>")?)
    }
    fn handle_verbatim(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        Ok(write!(w, "<code>{}</code>", Escape(cont))?)
    }
    fn handle_code(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        Ok(write!(w, "<code>{}</code>", Escape(cont))?)
    }
    fn handle_text(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        Ok(write!(w, "{}", Escape(cont))?)
    }
}

pub struct DefaultHtmlHandler;

impl<W: Write> HtmlHandler<W, Error> for DefaultHtmlHandler {}

pub struct HtmlRender<'a, W: Write, E: From<Error>, H: HtmlHandler<W, E>> {
    pub parser: Parser<'a>,
    pub writer: &'a mut W,
    handler: H,
    error_type: PhantomData<E>,
}

impl<'a, W: Write> HtmlRender<'a, W, Error, DefaultHtmlHandler> {
    pub fn default(writer: &'a mut W, text: &'a str) -> Self {
        HtmlRender::new(DefaultHtmlHandler, writer, text)
    }
}

impl<'a, W: Write, E: From<Error>, H: HtmlHandler<W, E>> HtmlRender<'a, W, E, H> {
    pub fn new(handler: H, writer: &'a mut W, text: &'a str) -> Self {
        HtmlRender {
            parser: Parser::new(text),
            handler,
            writer,
            error_type: PhantomData,
        }
    }

    pub fn render(&mut self) -> Result<(), E> {
        for event in &mut self.parser {
            handle_event!(event, &mut self.handler, &mut self.writer);
        }

        Ok(())
    }
}

pub struct Escape<'a>(pub &'a str);

impl<'a> fmt::Display for Escape<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut pos = 0;
        while let Some(off) = ascii_chars!('<', '>', '&', '\'', '"').find(&self.0[pos..]) {
            fmt.write_str(&self.0[pos..pos + off])?;

            pos += off + 1;

            match &self.0.as_bytes()[pos - 1] {
                b'"' => fmt.write_str("&quot;")?,
                b'&' => fmt.write_str("&amp;")?,
                b'<' => fmt.write_str("&lt;")?,
                b'>' => fmt.write_str("&gt;")?,
                b'\'' => fmt.write_str("&#39;")?,
                b'\n' => fmt.write_str(" ")?,
                _ => unreachable!(),
            }
        }

        fmt.write_str(&self.0[pos..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape() {
        assert_eq!(format!("{}", Escape("<<<<<<")), "&lt;&lt;&lt;&lt;&lt;&lt;");
        assert_eq!(format!("{}", Escape(" <> <> ")), " &lt;&gt; &lt;&gt; ");
    }
}
