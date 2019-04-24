#![allow(unused_variables)]

use crate::{elements::*, headline::Headline, objects::*, Parser};
use jetscii::bytes;
use std::{
    convert::From,
    io::{Error, Write},
    marker::PhantomData,
};

pub trait HtmlHandler<W: Write, E: From<Error>> {
    fn escape(&mut self, w: &mut W, text: &str) -> Result<(), E> {
        let mut pos = 0;
        let bytes = text.as_bytes();
        while let Some(off) = bytes!(b'<', b'>', b'&', b'\'', b'"').find(&bytes[pos..]) {
            w.write_all(&bytes[pos..pos + off])?;

            pos += off + 1;

            match text.as_bytes()[pos - 1] {
                b'<' => w.write_all(b"&lt;")?,
                b'>' => w.write_all(b"&gt;")?,
                b'&' => w.write_all(b"&amp;")?,
                b'\'' => w.write_all(b"&#39;")?,
                b'"' => w.write_all(b"&quot;")?,
                _ => unreachable!(),
            }
        }

        Ok(w.write_all(&bytes[pos..])?)
    }
    fn headline_beg(&mut self, w: &mut W, hdl: Headline) -> Result<(), E> {
        let level = if hdl.level <= 6 { hdl.level } else { 6 };
        write!(w, "<h{}>", level)?;
        self.text(w, hdl.title)?;
        write!(w, "</h{}>", level)?;
        Ok(())
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
        write!(w, "<pre><code>")?;
        self.escape(w, cont)?;
        write!(w, "</pre></code>")?;
        Ok(())
    }
    fn export_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        Ok(())
    }
    fn src_block(&mut self, w: &mut W, cont: &str, args: Option<&str>) -> Result<(), E> {
        write!(w, "<pre><code>")?;
        self.escape(w, cont)?;
        write!(w, "</pre></code>")?;
        Ok(())
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
    fn clock(&mut self, w: &mut W, clock: Clock<'_>) -> Result<(), E> {
        Ok(())
    }
    fn comment(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        Ok(())
    }
    fn fixed_width(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        for line in cont.lines() {
            // remove leading colon
            write!(w, "<pre>")?;
            self.escape(w, &line[1..])?;
            write!(w, "</pre>")?;
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
    fn keyword(&mut self, w: &mut W, keyword: Keyword<'_>) -> Result<(), E> {
        Ok(())
    }
    fn rule(&mut self, w: &mut W) -> Result<(), E> {
        Ok(write!(w, "<hr>")?)
    }
    fn cookie(&mut self, w: &mut W, cookie: Cookie) -> Result<(), E> {
        Ok(())
    }
    fn fn_ref(&mut self, w: &mut W, fn_ref: FnRef<'_>) -> Result<(), E> {
        Ok(())
    }
    fn inline_call(&mut self, w: &mut W, call: InlineCall<'_>) -> Result<(), E> {
        Ok(())
    }
    fn inline_src(&mut self, w: &mut W, src: InlineSrc<'_>) -> Result<(), E> {
        write!(w, "<code>")?;
        self.text(w, src.body)?;
        write!(w, "</code>")?;
        Ok(())
    }
    fn link(&mut self, w: &mut W, link: Link<'_>) -> Result<(), E> {
        write!(w, r#"<a href=""#)?;
        self.text(w, link.path)?;
        write!(w, r#"">"#)?;
        self.text(w, link.desc.unwrap_or(link.path))?;
        write!(w, "</a>")?;
        Ok(())
    }
    fn macros(&mut self, w: &mut W, macros: Macros<'_>) -> Result<(), E> {
        Ok(())
    }
    fn radio_target(&mut self, w: &mut W, target: &str) -> Result<(), E> {
        Ok(())
    }
    fn snippet(&mut self, w: &mut W, snippet: Snippet<'_>) -> Result<(), E> {
        if snippet.name.eq_ignore_ascii_case("HTML") {
            Ok(write!(w, "{}", snippet.value)?)
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
        write!(w, "<code>")?;
        self.text(w, cont)?;
        write!(w, "</code>")?;
        Ok(())
    }
    fn code(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        write!(w, "<code>")?;
        self.text(w, cont)?;
        write!(w, "</code>")?;
        Ok(())
    }
    fn text(&mut self, w: &mut W, cont: &str) -> Result<(), E> {
        self.escape(w, cont)?;
        Ok(())
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
