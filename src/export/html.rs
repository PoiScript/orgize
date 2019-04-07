#![allow(unused_variables)]

use crate::{
    elements::{Key, Planning},
    headline::Headline,
    objects::{Cookie, Timestamp},
    parser::Parser,
};
use jetscii::ascii_chars;
use std::{
    convert::From,
    fmt,
    io::{Error, Write},
    marker::PhantomData,
};

pub trait HtmlHandler<W: Write, E: From<Error>> {
    fn headline_beg(&mut self, w: &mut W, hdl: Headline) -> Result<(), E> {
        let level = if hdl.level <= 6 { hdl.level } else { 6 };
        Ok(write!(w, "<h{0}>{1}</h{0}>", level, Escape(hdl.title))?)
    }
    fn headline_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn section_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<section>")?)
    }
    fn section_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</section>")?)
    }
    fn drawer_beg(&mut self, w: &mut W, name: &str) -> Result<(), E> {
        Ok(())
    }
    fn drawer_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn paragraph_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<p>")?)
    }
    fn paragraph_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</p>")?)
    }
    fn ctr_block_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, r#"<div style="text-align: center">"#)?)
    }
    fn ctr_block_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</div>")?)
    }
    fn qte_block_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<blockquote>")?)
    }
    fn qte_block_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</blockquote>")?)
    }
    fn spl_block_beg(&mut self, w: &mut W, name: &str, args: Option<&str>) -> Result<(), E> {
        Ok(write!(w, "<div>")?)
    }
    fn spl_block_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</div>")?)
    }
    fn comment_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn example_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(write!(w, "<pre><code>{}</code></pre>", Escape(cont))?)
    }
    fn export_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn src_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(write!(w, "<pre><code>{}</code></pre>", Escape(cont))?)
    }
    fn verse_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn dyn_block_beg(&mut self, w: &mut W, name: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn dyn_block_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn list_beg(&mut self, w: &mut W, ordered: bool) -> Result<(), E> {
        if ordered {
            Ok(write!(w, "<ol>")?)
        } else {
            Ok(write!(w, "<ul>")?)
        }
    }
    fn list_end(&mut self, w: &mut W, ordered: bool) -> Result<(), E> {
        if ordered {
            Ok(write!(w, "</ol>")?)
        } else {
            Ok(write!(w, "</ul>")?)
        }
    }
    fn list_beg_item(&mut self, w: &mut W, bullet: &str) -> Result<(), E> {
        Ok(write!(w, "<li>")?)
    }
    fn list_end_item(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</li>")?)
    }
    fn call(&mut self, w: &mut W, value: &str) -> Result<(), E> {
        Ok(())
    }
    fn clock(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn comment(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        Ok(())
    }
    fn fixed_width(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        for line in cont.lines() {
            // remove leading colon
            write!(w, "<pre>{}</pre>", Escape(&line[1..]))?;
        }

        Ok(())
    }
    fn table_start(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn table_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn table_cell(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn latex_env(&mut self, w: &mut W) -> Result<(), E> {
        Ok(())
    }
    fn fn_def(&mut self, w: &mut W, label: &str, cont: &str) -> Result<(), E> {
        Ok(())
    }
    fn keyword(&mut self, w: &mut W, key: Key<'_>, value: &str) -> Result<(), E> {
        Ok(())
    }
    fn rule(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<hr>")?)
    }
    fn cookie(&mut self, w: &mut W, cookie: Cookie) -> Result<(), E> {
        Ok(())
    }
    fn fn_ref(&mut self, w: &mut W, label: Option<&str>, def: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn inline_call(
        &mut self,
        w: &mut W,
        name: &str,
        args: &str,
        inside_header: Option<&str>,
        end_header: Option<&str>,
    ) -> Result<(), E> {
        Ok(())
    }
    fn inline_src(
        &mut self,
        w: &mut W,
        lang: &str,
        option: Option<&str>,
        body: &str,
    ) -> Result<(), E> {
        Ok(write!(w, "<code>{}</code>", Escape(body))?)
    }
    fn link(&mut self, w: &mut W, path: &str, desc: Option<&str>) -> Result<(), E> {
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
    fn macros(&mut self, w: &mut W, name: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn radio_target(&mut self, w: &mut W, target: &str) -> Result<(), E> {
        Ok(())
    }
    fn snippet(&mut self, w: &mut W, name: &str, value: &str) -> Result<(), E> {
        if name.eq_ignore_ascii_case("HTML") {
            Ok(write!(w, "{}", value)?)
        } else {
            Ok(())
        }
    }
    fn target(&mut self, w: &mut W, target: &str) -> Result<(), E> {
        Ok(())
    }
    fn timestamp(&mut self, w: &mut W, timestamp: Timestamp) -> Result<(), E> {
        Ok(())
    }
    fn bold_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<b>")?)
    }
    fn bold_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</b>")?)
    }
    fn italic_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<i>")?)
    }
    fn italic_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</i>")?)
    }
    fn strike_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<s>")?)
    }
    fn strike_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</s>")?)
    }
    fn underline_beg(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<u>")?)
    }
    fn underline_end(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "</u>")?)
    }
    fn verbatim(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        Ok(write!(w, "<code>{}</code>", Escape(cont))?)
    }
    fn code(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        Ok(write!(w, "<code>{}</code>", Escape(cont))?)
    }
    fn text(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        Ok(write!(w, "{}", Escape(cont))?)
    }
    fn planning(&mut self, w: &mut W, planning: Planning) -> Result<(), E> {
        Ok(())
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
            handle_event!(event, &mut self.handler, self.writer);
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
