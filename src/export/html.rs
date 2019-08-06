use crate::elements::{Datetime, Element};
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
            SpecialBlock(_) => (),
            QuoteBlock(_) => write!(w, "<blockquote>")?,
            CenterBlock(_) => write!(w, "<div class=\"center\">")?,
            VerseBlock(_) => write!(w, "<p class=\"verse\">")?,
            Bold => write!(w, "<b>")?,
            Document => write!(w, "<main>")?,
            DynBlock(_dyn_block) => (),
            Headline => (),
            List(list) => {
                if list.ordered {
                    write!(w, "<ol>")?;
                } else {
                    write!(w, "<ul>")?;
                }
            }
            Italic => write!(w, "<i>")?,
            ListItem(_) => write!(w, "<li>")?,
            Paragraph => write!(w, "<p>")?,
            Section => write!(w, "<section>")?,
            Strike => write!(w, "<s>")?,
            Underline => write!(w, "<u>")?,
            // non-container elements
            CommentBlock(_) => (),
            ExampleBlock(block) => {
                write!(w, "<pre class=\"example\">{}</pre>", Escape(block.contents))?
            }
            ExportBlock(block) => {
                if block.data.eq_ignore_ascii_case("HTML") {
                    write!(w, "{}", block.contents)?
                }
            }
            SourceBlock(block) => {
                if block.language.is_empty() {
                    write!(w, "<pre class=\"example\">{}</pre>", Escape(block.contents))?;
                } else {
                    write!(
                        w,
                        "<div class=\"org-src-container\"><pre class=\"src src-{}\">{}</pre></div>",
                        block.language,
                        Escape(block.contents)
                    )?;
                }
            }
            BabelCall(_) => (),
            InlineSrc(inline_src) => write!(
                w,
                "<code class=\"src src-{}\">{}</code>",
                inline_src.lang,
                Escape(inline_src.body)
            )?,
            Code { value } => write!(w, "<code>{}</code>", Escape(value))?,
            FnRef(_fn_ref) => (),
            InlineCall(_) => (),
            Link(link) => write!(
                w,
                "<a href=\"{}\">{}</a>",
                Escape(link.path),
                Escape(link.desc.unwrap_or(link.path)),
            )?,
            Macros(_macros) => (),
            RadioTarget(_radio_target) => (),
            Snippet(snippet) => {
                if snippet.name.eq_ignore_ascii_case("HTML") {
                    write!(w, "{}", snippet.value)?;
                }
            }
            Target(_target) => (),
            Text { value } => write!(w, "{}", Escape(value))?,
            Timestamp(timestamp) => {
                use crate::elements::Timestamp;

                write!(
                    &mut w,
                    "<span class=\"timestamp-wrapper\"><span class=\"timestamp\">"
                )?;

                fn write_datetime<W: Write>(
                    mut w: W,
                    start: &str,
                    datetime: &Datetime,
                    end: &str,
                ) -> Result<(), Error> {
                    write!(w, "{}", start)?;
                    write!(
                        w,
                        "{}-{}-{} {}",
                        datetime.year, datetime.month, datetime.day, datetime.dayname
                    )?;
                    if let (Some(hour), Some(minute)) = (datetime.hour, datetime.minute) {
                        write!(w, " {}:{}", hour, minute)?;
                    }
                    write!(w, "{}", end)
                }

                match timestamp {
                    Timestamp::Active { start, .. } => {
                        write_datetime(&mut w, "&lt;", start, "&gt;")?;
                    }
                    Timestamp::Inactive { start, .. } => {
                        write_datetime(&mut w, "[", start, "]")?;
                    }
                    Timestamp::ActiveRange { start, end, .. } => {
                        write_datetime(&mut w, "&lt;", start, "&gt;&#x2013;")?;
                        write_datetime(&mut w, "&lt;", end, "&gt;")?;
                    }
                    Timestamp::InactiveRange { start, end, .. } => {
                        write_datetime(&mut w, "[", start, "]&#x2013;")?;
                        write_datetime(&mut w, "[", end, "]")?;
                    }
                    Timestamp::Diary { value } => write!(&mut w, "&lt;%%({})&gt;", Escape(value))?,
                }

                write!(&mut w, "</span></span>")?;
            }
            Verbatim { value } => write!(&mut w, "<code>{}</code>", Escape(value))?,
            FnDef(_fn_def) => (),
            Clock(_clock) => (),
            Comment { .. } => (),
            FixedWidth { value } => write!(w, "<pre class=\"example\">{}</pre>", Escape(value))?,
            Keyword(_keyword) => (),
            Drawer(_drawer) => (),
            Rule => write!(w, "<hr>")?,
            Cookie(cookie) => write!(w, "<code>{}</code>", cookie.value)?,
            Title(title) => write!(w, "<h{}>", if title.level <= 6 { title.level } else { 6 })?,
            Table(_) => (),
            TableRow(_) => (),
            TableCell => (),
        }

        Ok(())
    }
    fn end<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), E> {
        use Element::*;

        match element {
            // container elements
            SpecialBlock(_) => (),
            QuoteBlock(_) => write!(w, "</blockquote>")?,
            CenterBlock(_) => write!(w, "</div>")?,
            VerseBlock(_) => write!(w, "</p>")?,
            Bold => write!(w, "</b>")?,
            Document => write!(w, "</main>")?,
            DynBlock(_dyn_block) => (),
            Headline => (),
            List(list) => {
                if list.ordered {
                    write!(w, "</ol>")?;
                } else {
                    write!(w, "</ul>")?;
                }
            }
            Italic => write!(w, "</i>")?,
            ListItem(_) => write!(w, "</li>")?,
            Paragraph => write!(w, "</p>")?,
            Section => write!(w, "</section>")?,
            Strike => write!(w, "</s>")?,
            Underline => write!(w, "</u>")?,
            Title(title) => write!(w, "</h{}>", if title.level <= 6 { title.level } else { 6 })?,
            Table(_) => (),
            TableRow(_) => (),
            TableCell => (),
            // non-container elements
            _ => debug_assert!(!element.is_container()),
        }

        Ok(())
    }
}

pub struct DefaultHtmlHandler;

impl HtmlHandler<Error> for DefaultHtmlHandler {}
