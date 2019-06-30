use crate::elements::Element;
use jetscii::bytes;
use std::fmt;
use std::io::{Error, Write};

pub struct Escape<'a>(pub &'a str);

impl fmt::Display for Escape<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pos = 0;
        let bytes = self.0.as_bytes();
        while let Some(off) = bytes!(b'<', b'>', b'&', b'\'', b'"').find(&bytes[pos..]) {
            write!(f, "{}", &self.0[pos..pos + off])?;

            pos += off + 1;

            match bytes[pos - 1] {
                b'<' => write!(f, "&lt;")?,
                b'>' => write!(f, "&gt;")?,
                b'&' => write!(f, "&amp;")?,
                b'\'' => write!(f, "&#39;")?,
                b'"' => write!(f, "&quot;")?,
                _ => unreachable!(),
            }
        }

        write!(f, "{}", &self.0[pos..])
    }
}

pub trait HtmlHandler<E: From<Error>> {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), E> {
        use Element::*;

        match element {
            // container elements
            Block(_block) => write!(w, "<div>")?,
            Bold { .. } => write!(w, "<b>")?,
            Document { .. } => write!(w, "<main>")?,
            DynBlock(_dyn_block) => (),
            Headline(headline) => {
                let level = if headline.level <= 6 {
                    headline.level
                } else {
                    6
                };
                write!(w, "<h{0}>{1}</h{0}>", level, Escape(headline.title))?;
            }
            List(list) => {
                if list.ordered {
                    write!(w, "<ol>")?;
                } else {
                    write!(w, "<ul>")?;
                }
            }
            Italic { .. } => write!(w, "<i>")?,
            ListItem { .. } => write!(w, "<li>")?,
            Paragraph { .. } => write!(w, "<p>")?,
            Section { .. } => write!(w, "<section>")?,
            Strike { .. } => write!(w, "<s>")?,
            Underline { .. } => write!(w, "<u>")?,
            // non-container elements
            BabelCall(_babel_call) => (),
            InlineSrc(inline_src) => write!(w, "<code>{}</code>", Escape(inline_src.body))?,
            Code { value } => write!(w, "<code>{}</code>", Escape(value))?,
            FnRef(_fn_ref) => (),
            InlineCall(_inline_call) => (),
            Link(link) => write!(
                w,
                "<a href=\"{}\">{}</a>",
                Escape(link.path),
                Escape(link.desc.unwrap_or(link.path)),
            )?,
            Macros(_macros) => (),
            Planning(_planning) => (),
            RadioTarget(_radio_target) => (),
            Snippet(snippet) => {
                if snippet.name.eq_ignore_ascii_case("HTML") {
                    write!(w, "{}", snippet.value)?;
                }
            }
            Target(_target) => (),
            Text { value } => write!(w, "{}", Escape(value))?,
            Timestamp(_timestamp) => (),
            Verbatim { value } => write!(&mut w, "<code>{}</code>", Escape(value))?,
            FnDef(_fn_def) => (),
            Clock(_clock) => (),
            Comment { value } => write!(w, "<!--\n{}\n-->", Escape(value))?,
            FixedWidth { value } => write!(w, "<pre>{}</pre>", Escape(value))?,
            Keyword(_keyword) => (),
            Drawer(_drawer) => (),
            Rule => write!(w, "<hr>")?,
            Cookie(_cookie) => (),
        }

        Ok(())
    }
    fn end<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), E> {
        use Element::*;

        match element {
            // container elements
            Block(_block) => write!(w, "</div>")?,
            Bold { .. } => write!(w, "</b>")?,
            Document { .. } => write!(w, "</main>")?,
            DynBlock(_dyn_block) => (),
            Headline(_headline) => (),
            List(list) => {
                if list.ordered {
                    write!(w, "</ol>")?;
                } else {
                    write!(w, "</ul>")?;
                }
            }
            Italic { .. } => write!(w, "</i>")?,
            ListItem { .. } => write!(w, "</li>")?,
            Paragraph { .. } => write!(w, "</p>")?,
            Section { .. } => write!(w, "</section>")?,
            Strike { .. } => write!(w, "</s>")?,
            Underline { .. } => write!(w, "</u>")?,
            // non-container elements
            _ => (),
        }

        Ok(())
    }
}

pub struct DefaultHtmlHandler;

impl HtmlHandler<Error> for DefaultHtmlHandler {}
