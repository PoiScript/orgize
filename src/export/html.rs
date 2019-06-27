#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::elements::*;
use crate::iter::Container;
use jetscii::bytes;
use std::io::{Error, Write};

pub trait HtmlHandler<E: From<Error>> {
    fn escape<W: Write>(&mut self, mut w: W, text: &str) -> Result<(), E> {
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
    fn start<W: Write>(&mut self, mut w: W, container: Container) -> Result<(), E> {
        match container {
            Container::Block(block) => write!(w, "<div>")?,
            Container::Bold => write!(w, "<b>")?,
            Container::Document => write!(w, "<main>")?,
            Container::DynBlock(_) => (),
            Container::Headline(hdl) => {
                let level = if hdl.level <= 6 { hdl.level } else { 6 };
                write!(&mut w, "<h{}>", level)?;
                self.text(&mut w, hdl.title)?;
                write!(&mut w, "</h{}>", level)?;
            }
            Container::Italic => write!(w, "<i>")?,
            Container::List(list) => {
                if list.ordered {
                    write!(w, "<ol>")?;
                } else {
                    write!(w, "<ul>")?;
                }
            }
            Container::ListItem(_) => write!(w, "<li>")?,
            Container::Paragraph => write!(w, "<p>")?,
            Container::Section => write!(w, "<section>")?,
            Container::Strike => write!(w, "<s>")?,
            Container::Underline => write!(w, "<u>")?,
        }
        Ok(())
    }
    fn end<W: Write>(&mut self, mut w: W, container: Container) -> Result<(), E> {
        match container {
            Container::Block(block) => write!(w, "</div>")?,
            Container::Bold => write!(w, "</b>")?,
            Container::Document => write!(w, "</main>")?,
            Container::DynBlock(_) => (),
            Container::Headline(_) => (),
            Container::Italic => write!(w, "</i>")?,
            Container::List(list) => {
                if list.ordered {
                    write!(w, "</ol>")?;
                } else {
                    write!(w, "</ul>")?;
                }
            }
            Container::ListItem(_) => write!(w, "</li>")?,
            Container::Paragraph => write!(w, "</p>")?,
            Container::Section => write!(w, "</section>")?,
            Container::Strike => write!(w, "</s>")?,
            Container::Underline => write!(w, "</u>")?,
        }
        Ok(())
    }
    fn keyword<W: Write>(&mut self, mut w: W, keyword: &Keyword<'_>) -> Result<(), E> {
        Ok(())
    }
    fn drawer<W: Write>(&mut self, mut w: W, drawer: &Drawer<'_>) -> Result<(), E> {
        Ok(())
    }
    fn rule<W: Write>(&mut self, mut w: W) -> Result<(), E> {
        Ok(write!(w, "<hr>")?)
    }
    fn cookie<W: Write>(&mut self, mut w: W, cookie: &Cookie) -> Result<(), E> {
        Ok(())
    }
    fn fn_ref<W: Write>(&mut self, mut w: W, fn_ref: &FnRef<'_>) -> Result<(), E> {
        Ok(())
    }
    fn babel_call<W: Write>(&mut self, mut w: W, call: &BabelCall<'_>) -> Result<(), E> {
        Ok(())
    }
    fn inline_call<W: Write>(&mut self, mut w: W, call: &InlineCall<'_>) -> Result<(), E> {
        Ok(())
    }
    fn inline_src<W: Write>(&mut self, mut w: W, src: &InlineSrc<'_>) -> Result<(), E> {
        write!(&mut w, "<code>")?;
        self.text(&mut w, src.body)?;
        write!(&mut w, "</code>")?;
        Ok(())
    }
    fn link<W: Write>(&mut self, mut w: W, link: &Link<'_>) -> Result<(), E> {
        write!(&mut w, r#"<a href=""#)?;
        self.text(&mut w, link.path)?;
        write!(&mut w, r#"">"#)?;
        self.text(&mut w, link.desc.unwrap_or(link.path))?;
        write!(&mut w, "</a>")?;
        Ok(())
    }
    fn macros<W: Write>(&mut self, mut w: W, macros: &Macros<'_>) -> Result<(), E> {
        Ok(())
    }
    fn radio_target<W: Write>(&mut self, mut w: W, target: &RadioTarget<'_>) -> Result<(), E> {
        Ok(())
    }
    fn snippet<W: Write>(&mut self, mut w: W, snippet: &Snippet<'_>) -> Result<(), E> {
        if snippet.name.eq_ignore_ascii_case("HTML") {
            write!(w, "{}", snippet.value)?;
        }
        Ok(())
    }
    fn target<W: Write>(&mut self, mut w: W, target: &Target<'_>) -> Result<(), E> {
        Ok(())
    }
    fn timestamp<W: Write>(&mut self, mut w: W, timestamp: &Timestamp) -> Result<(), E> {
        Ok(())
    }
    fn verbatim<W: Write>(&mut self, mut w: W, cont: &str) -> Result<(), E> {
        write!(&mut w, "<code>")?;
        self.text(&mut w, cont)?;
        write!(&mut w, "</code>")?;
        Ok(())
    }
    fn code<W: Write>(&mut self, mut w: W, cont: &str) -> Result<(), E> {
        write!(&mut w, "<code>")?;
        self.text(&mut w, cont)?;
        write!(&mut w, "</code>")?;
        Ok(())
    }
    fn text<W: Write>(&mut self, mut w: W, cont: &str) -> Result<(), E> {
        self.escape(w, cont)?;
        Ok(())
    }
    fn planning<W: Write>(&mut self, mut w: W, planning: &Planning) -> Result<(), E> {
        Ok(())
    }
    fn clock<W: Write>(&mut self, mut w: W, clock: &Clock<'_>) -> Result<(), E> {
        Ok(())
    }
    fn fn_def<W: Write>(&mut self, mut w: W, fn_def: &FnDef<'_>) -> Result<(), E> {
        Ok(())
    }
    fn comment<W: Write>(&mut self, mut w: W, value: &str) -> Result<(), E> {
        write!(&mut w, "<!--\n")?;
        self.text(&mut w, value)?;
        write!(&mut w, "\n-->")?;
        Ok(())
    }
    fn fixed_width<W: Write>(&mut self, mut w: W, value: &str) -> Result<(), E> {
        write!(&mut w, "<pre>")?;
        self.text(&mut w, value)?;
        write!(&mut w, "</pre>")?;
        Ok(())
    }
}

pub struct DefaultHtmlHandler;

impl HtmlHandler<Error> for DefaultHtmlHandler {}
