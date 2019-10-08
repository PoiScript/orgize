use std::fmt;
use std::io::{Error, Write};

use jetscii::{bytes, BytesConst};

use crate::elements::Element;
use crate::export::write_datetime;

pub struct Escape<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> fmt::Display for Escape<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pos = 0;
        let bytes = self.0.as_ref().as_bytes();

        lazy_static::lazy_static! {
            static ref ESCAPE_BYTES: BytesConst = bytes!(b'<', b'>', b'&', b'\'', b'"');
        }

        while let Some(off) = ESCAPE_BYTES.find(&bytes[pos..]) {
            write!(f, "{}", &self.0.as_ref()[pos..pos + off])?;

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

        write!(f, "{}", &self.0.as_ref()[pos..])
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
            Headline { .. } => (),
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
            ExampleBlock(block) => write!(
                w,
                "<pre class=\"example\">{}</pre>",
                Escape(&block.contents)
            )?,
            ExportBlock(block) => {
                if block.data.eq_ignore_ascii_case("HTML") {
                    write!(w, "{}", block.contents)?
                }
            }
            SourceBlock(block) => {
                if block.language.is_empty() {
                    write!(
                        w,
                        "<pre class=\"example\">{}</pre>",
                        Escape(&block.contents)
                    )?;
                } else {
                    write!(
                        w,
                        "<div class=\"org-src-container\"><pre class=\"src src-{}\">{}</pre></div>",
                        block.language,
                        Escape(&block.contents)
                    )?;
                }
            }
            BabelCall(_) => (),
            InlineSrc(inline_src) => write!(
                w,
                "<code class=\"src src-{}\">{}</code>",
                inline_src.lang,
                Escape(&inline_src.body)
            )?,
            Code { value } => write!(w, "<code>{}</code>", Escape(value))?,
            FnRef(_fn_ref) => (),
            InlineCall(_) => (),
            Link(link) => write!(
                w,
                "<a href=\"{}\">{}</a>",
                Escape(&link.path),
                Escape(link.desc.as_ref().unwrap_or(&link.path)),
            )?,
            Macros(_macros) => (),
            RadioTarget => (),
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
            Headline { .. } => (),
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

#[cfg(feature = "syntect")]
pub mod syntect_feature {
    use super::*;
    use std::marker::PhantomData;

    use syntect::{
        easy::HighlightLines,
        highlighting::ThemeSet,
        html::{styled_line_to_highlighted_html, IncludeBackground},
        parsing::SyntaxSet,
    };

    pub struct SyntectHtmlHandler<E: From<Error>, H: HtmlHandler<E>> {
        pub syntax_set: SyntaxSet,
        pub theme_set: ThemeSet,
        pub inner: H,
        error_type: PhantomData<E>,
        theme: String,
    }

    impl Default for SyntectHtmlHandler<Error, DefaultHtmlHandler> {
        fn default() -> Self {
            SyntectHtmlHandler {
                syntax_set: SyntaxSet::load_defaults_newlines(),
                theme_set: ThemeSet::load_defaults(),
                inner: DefaultHtmlHandler,
                error_type: PhantomData,
                theme: "InspiredGitHub".into(),
            }
        }
    }

    impl<E: From<Error>, H: HtmlHandler<E>> SyntectHtmlHandler<E, H> {
        pub fn new(inner: H, theme: impl Into<String>) -> Self {
            SyntectHtmlHandler {
                syntax_set: SyntaxSet::load_defaults_newlines(),
                theme_set: ThemeSet::load_defaults(),
                inner,
                error_type: PhantomData,
                theme: theme.into(),
            }
        }

        fn highlight(&self, language: Option<&str>, content: &str) -> String {
            let mut highlighter = HighlightLines::new(
                language
                    .and_then(|lang| self.syntax_set.find_syntax_by_token(lang))
                    .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text()),
                &self.theme_set.themes[&self.theme],
            );
            let regions = highlighter.highlight(content, &self.syntax_set);
            styled_line_to_highlighted_html(&regions[..], IncludeBackground::No)
        }
    }

    impl<E: From<Error>, H: HtmlHandler<E>> HtmlHandler<E> for SyntectHtmlHandler<E, H> {
        fn start<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), E> {
            match element {
                Element::InlineSrc(inline_src) => write!(
                    w,
                    "<code>{}</code>",
                    self.highlight(Some(&inline_src.lang), &inline_src.body)
                )?,
                Element::SourceBlock(block) => {
                    if block.language.is_empty() {
                        write!(w, "<pre class=\"example\">{}</pre>", block.contents)?;
                    } else {
                        write!(
                        w,
                        "<div class=\"org-src-container\"><pre class=\"src src-{}\">{}</pre></div>",
                        block.language,
                        self.highlight(Some(&block.language), &block.contents)
                    )?
                    }
                }
                Element::FixedWidth { value } => write!(
                    w,
                    "<pre class=\"example\">{}</pre>",
                    self.highlight(None, value)
                )?,
                Element::ExampleBlock(block) => write!(
                    w,
                    "<pre class=\"example\">{}</pre>",
                    self.highlight(None, &block.contents)
                )?,
                _ => self.inner.start(w, element)?,
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_highlight_code() {
            let handler = SyntectHtmlHandler::new(DefaultHtmlHandler, "Solarized (light)");
            let code = r#"let name = "James";"#;
            let result = handler.highlight(Some("rust"), code);

            assert!(result.len() > 0);
            // Contains Solarized colors
            assert!(result.contains("#657b83"));
            assert!(result.contains("#268bd2"));
        }
    }
}

#[cfg(feature = "syntect")]
pub use syntect_feature::*;
