use std::fmt;
use std::io::{Error, Write};

use jetscii::{bytes, BytesConst};

use crate::elements::Element;
use crate::export::write_datetime;

/// A wrapper for escaping sensitive characters in html.
///
/// ```rust
/// use orgize::export::HtmlEscape as Escape;
///
/// assert_eq!(format!("{}", Escape("< < <")), "&lt; &lt; &lt;");
/// assert_eq!(
///     format!("{}", Escape("<script>alert('Hello XSS')</script>")),
///     "&lt;script&gt;alert(&apos;Hello XSS&apos;)&lt;/script&gt;"
/// );
/// ```
pub struct HtmlEscape<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> fmt::Display for HtmlEscape<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pos = 0;

        let content = self.0.as_ref();
        let bytes = content.as_bytes();

        lazy_static::lazy_static! {
            static ref ESCAPE_BYTES: BytesConst = bytes!(b'<', b'>', b'&', b'\'', b'"');
        }

        while let Some(off) = ESCAPE_BYTES.find(&bytes[pos..]) {
            write!(f, "{}", &content[pos..pos + off])?;

            pos += off + 1;

            match bytes[pos - 1] {
                b'<' => write!(f, "&lt;")?,
                b'>' => write!(f, "&gt;")?,
                b'&' => write!(f, "&amp;")?,
                b'\'' => write!(f, "&apos;")?,
                b'"' => write!(f, "&quot;")?,
                _ => unreachable!(),
            }
        }

        write!(f, "{}", &content[pos..])
    }
}

pub trait HtmlHandler<E: From<Error>>: Default {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), E> {
        use Element::*;

        match element {
            // container elements
            SpecialBlock(_) => (),
            QuoteBlock(_) => write!(w, "<blockquote>")?,
            CenterBlock(_) => write!(w, "<div class=\"center\">")?,
            VerseBlock(_) => write!(w, "<p class=\"verse\">")?,
            Bold => write!(w, "<b>")?,
            Document { .. } => write!(w, "<main>")?,
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
            Paragraph { .. } => write!(w, "<p>")?,
            Section => write!(w, "<section>")?,
            Strike => write!(w, "<s>")?,
            Underline => write!(w, "<u>")?,
            // non-container elements
            CommentBlock(_) => (),
            ExampleBlock(block) => write!(
                w,
                "<pre class=\"example\">{}</pre>",
                HtmlEscape(&block.contents)
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
                        HtmlEscape(&block.contents)
                    )?;
                } else {
                    write!(
                        w,
                        "<div class=\"org-src-container\"><pre class=\"src src-{}\">{}</pre></div>",
                        block.language,
                        HtmlEscape(&block.contents)
                    )?;
                }
            }
            BabelCall(_) => (),
            InlineSrc(inline_src) => write!(
                w,
                "<code class=\"src src-{}\">{}</code>",
                inline_src.lang,
                HtmlEscape(&inline_src.body)
            )?,
            Code { value } => write!(w, "<code>{}</code>", HtmlEscape(value))?,
            FnRef(_fn_ref) => (),
            InlineCall(_) => (),
            Link(link) => write!(
                w,
                "<a href=\"{}\">{}</a>",
                HtmlEscape(&link.path),
                HtmlEscape(link.desc.as_ref().unwrap_or(&link.path)),
            )?,
            Macros(_macros) => (),
            RadioTarget => (),
            Snippet(snippet) => {
                if snippet.name.eq_ignore_ascii_case("HTML") {
                    write!(w, "{}", snippet.value)?;
                }
            }
            Target(_target) => (),
            Text { value } => write!(w, "{}", HtmlEscape(value))?,
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
                    Timestamp::Diary { value } => {
                        write!(&mut w, "&lt;%%({})&gt;", HtmlEscape(value))?
                    }
                }

                write!(&mut w, "</span></span>")?;
            }
            Verbatim { value } => write!(&mut w, "<code>{}</code>", HtmlEscape(value))?,
            FnDef(_fn_def) => (),
            Clock(_clock) => (),
            Comment(_) => (),
            FixedWidth(fixed_width) => write!(
                w,
                "<pre class=\"example\">{}</pre>",
                HtmlEscape(&fixed_width.value)
            )?,
            Keyword(_keyword) => (),
            Drawer(_drawer) => (),
            Rule(_) => write!(w, "<hr>")?,
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
            Document { .. } => write!(w, "</main>")?,
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
            Paragraph { .. } => write!(w, "</p>")?,
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

/// Default Html Handler
#[derive(Default)]
pub struct DefaultHtmlHandler;

impl HtmlHandler<Error> for DefaultHtmlHandler {}

#[cfg(feature = "syntect")]
mod syntect_handler {
    use super::*;
    use std::marker::PhantomData;

    use syntect::{
        easy::HighlightLines,
        highlighting::ThemeSet,
        html::{styled_line_to_highlighted_html, IncludeBackground},
        parsing::SyntaxSet,
    };

    /// Syntect Html Handler
    ///
    /// Simple Usage:
    ///
    /// ```rust
    /// use orgize::Org;
    /// use orgize::export::html::{DefaultHtmlHandler, SyntectHtmlHandler};
    ///
    /// let mut handler = SyntectHtmlHandler::new(DefaultHtmlHandler);
    /// let org = Org::parse("src_rust{println!(\"Hello\")}");
    ///
    /// let mut vec = vec![];
    ///
    /// org.html_with_handler(&mut vec, &mut handler).unwrap();
    /// ```
    ///
    /// Customize:
    ///
    /// ```rust,no_run
    /// // orgize has re-exported the whole syntect crate
    /// use orgize::syntect::parsing::SyntaxSet;
    /// use orgize::export::html::{DefaultHtmlHandler, SyntectHtmlHandler};
    ///
    /// let mut handler = SyntectHtmlHandler {
    ///     syntax_set: {
    ///         let set = SyntaxSet::load_defaults_newlines();
    ///         let mut builder = set.into_builder();
    ///         // add extra language syntax
    ///         builder.add_from_folder("path/to/syntax/dir", true).unwrap();
    ///         builder.build()
    ///     },
    ///     // specify theme
    ///     theme: String::from("Solarized (dark)"),
    ///     inner: DefaultHtmlHandler,
    ///     ..Default::default()
    /// };
    ///
    /// // Make sure to check if theme presents or it will painc at runtime
    /// if handler.theme_set.themes.contains_key("dont-exists") {
    ///
    /// }
    /// ```
    pub struct SyntectHtmlHandler<E: From<Error>, H: HtmlHandler<E>> {
        /// syntax set, default is `SyntaxSet::load_defaults_newlines()`
        pub syntax_set: SyntaxSet,
        /// theme set, default is `ThemeSet::load_defaults()`
        pub theme_set: ThemeSet,
        /// theme used for highlighting, default is `"InspiredGitHub"`
        pub theme: String,
        /// inner html handler
        pub inner: H,
        /// background color, default is `IncludeBackground::No`
        pub background: IncludeBackground,
        /// handler error type
        pub error_type: PhantomData<E>,
    }

    impl<E: From<Error>, H: HtmlHandler<E>> SyntectHtmlHandler<E, H> {
        pub fn new(inner: H) -> Self {
            SyntectHtmlHandler {
                inner,
                ..Default::default()
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
            styled_line_to_highlighted_html(&regions[..], self.background)
        }
    }

    impl<E: From<Error>, H: HtmlHandler<E>> Default for SyntectHtmlHandler<E, H> {
        fn default() -> Self {
            SyntectHtmlHandler {
                syntax_set: SyntaxSet::load_defaults_newlines(),
                theme_set: ThemeSet::load_defaults(),
                theme: String::from("InspiredGitHub"),
                inner: H::default(),
                background: IncludeBackground::No,
                error_type: PhantomData,
            }
        }
    }

    impl<E: From<Error>, H: HtmlHandler<E>> HtmlHandler<E> for SyntectHtmlHandler<E, H> {
        fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), E> {
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
                        )?;
                    }
                }
                Element::FixedWidth(fixed_width) => write!(
                    w,
                    "<pre class=\"example\">{}</pre>",
                    self.highlight(None, fixed_width.value)
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

        fn end<W: Write>(&mut self, w: W, element: &Element) -> Result<(), E> {
            self.inner.end(w, element)
        }
    }
}

#[cfg(feature = "syntect")]
pub use syntect_handler::SyntectHtmlHandler;
