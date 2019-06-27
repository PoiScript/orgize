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
            Block { .. } => write!(w, "<div>")?,
            Bold { .. } => write!(w, "<b>")?,
            Document { .. } => write!(w, "<main>")?,
            DynBlock { .. } => (),
            Headline { headline, .. } => {
                let level = if headline.level <= 6 {
                    headline.level
                } else {
                    6
                };
                write!(w, "<h{0}>{1}</h{0}>", level, Escape(headline.title))?;
            }
            List { list, .. } => {
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
            BabelCall { .. } => (),
            InlineSrc { inline_src, .. } => write!(w, "<code>{}</code>", Escape(inline_src.body))?,
            Code { value, .. } => write!(w, "<code>{}</code>", Escape(value))?,
            FnRef { .. } => (),
            InlineCall { .. } => (),
            Link { link, .. } => write!(
                w,
                "<a href=\"{}\">{}</a>",
                Escape(link.path),
                Escape(link.desc.unwrap_or(link.path)),
            )?,
            Macros { .. } => (),
            Planning { .. } => (),
            RadioTarget { .. } => (),
            Snippet { snippet, .. } => {
                if snippet.name.eq_ignore_ascii_case("HTML") {
                    write!(w, "{}", snippet.value)?;
                }
            }
            Target { .. } => (),
            Text { value, .. } => write!(w, "{}", Escape(value))?,
            Timestamp { .. } => (),
            Verbatim { value, .. } => write!(&mut w, "<code>{}</code>", Escape(value))?,
            FnDef { .. } => (),
            Clock { .. } => (),
            Comment { value, .. } => write!(w, "<!--\n{}\n-->", Escape(value))?,
            FixedWidth { value, .. } => write!(w, "<pre>{}</pre>", Escape(value))?,
            Keyword { .. } => (),
            Drawer { .. } => (),
            Rule { .. } => write!(w, "<hr>")?,
            Cookie { .. } => (),
        }

        Ok(())
    }
    fn end<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), E> {
        use Element::*;

        match element {
            // container elements
            Block { .. } => write!(w, "</div>")?,
            Bold { .. } => write!(w, "</b>")?,
            Document { .. } => write!(w, "</main>")?,
            DynBlock { .. } => (),
            Headline { .. } => (),
            List { list, .. } => {
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
