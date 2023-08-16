use std::fmt;
use std::io::{Error, Result as IOResult, Write};

use jetscii::{bytes, BytesConst};

use crate::elements::{Element, Table, TableCell, TableRow, Timestamp};
use crate::export::{write_datetime, ExportHandler};

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

/// Default Html Handler
#[derive(Default)]
pub struct DefaultHtmlHandler;

impl ExportHandler<Error> for DefaultHtmlHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element, _ancestors: Vec<&Element>) -> IOResult<()> {
        match element {
            // container elements
            Element::SpecialBlock(_) => (),
            Element::QuoteBlock(_) => write!(w, "<blockquote>")?,
            Element::CenterBlock(_) => write!(w, "<div class=\"center\">")?,
            Element::VerseBlock(_) => write!(w, "<p class=\"verse\">")?,
            Element::Bold => write!(w, "<b>")?,
            Element::Document { .. } => write!(w, "<main>")?,
            Element::DynBlock(_dyn_block) => (),
            Element::Headline { .. } => (),
            Element::List(list) => {
                if list.ordered {
                    write!(w, "<ol>")?;
                } else {
                    write!(w, "<ul>")?;
                }
            }
            Element::Italic => write!(w, "<i>")?,
            Element::ListItem(_) => write!(w, "<li>")?,
            Element::Paragraph { .. } => write!(w, "<p>")?,
            Element::Section => write!(w, "<section>")?,
            Element::Strike => write!(w, "<s>")?,
            Element::Underline => write!(w, "<u>")?,
            // non-container elements
            Element::CommentBlock(_) => (),
            Element::ExampleBlock(block) => write!(
                w,
                "<pre class=\"example\">{}</pre>",
                HtmlEscape(&block.contents)
            )?,
            Element::ExportBlock(block) => {
                if block.data.eq_ignore_ascii_case("HTML") {
                    write!(w, "{}", block.contents)?
                }
            }
            Element::SourceBlock(block) => {
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
            Element::BabelCall(_) => (),
            Element::InlineSrc(inline_src) => write!(
                w,
                "<code class=\"src src-{}\">{}</code>",
                inline_src.lang,
                HtmlEscape(&inline_src.body)
            )?,
            Element::Code { value } => write!(w, "<code>{}</code>", HtmlEscape(value))?,
            Element::FnRef(_fn_ref) => (),
            Element::InlineCall(_) => (),
            Element::Link(link) => write!(
                w,
                "<a href=\"{}\">{}</a>",
                HtmlEscape(&link.path),
                HtmlEscape(link.desc.as_ref().unwrap_or(&link.path)),
            )?,
            Element::Macros(_macros) => (),
            Element::RadioTarget => (),
            Element::Snippet(snippet) => {
                if snippet.name.eq_ignore_ascii_case("HTML") {
                    write!(w, "{}", snippet.value)?;
                }
            }
            Element::Target(_target) => (),
            Element::Text { value } => write!(w, "{}", HtmlEscape(value))?,
            Element::Timestamp(timestamp) => {
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
            Element::Verbatim { value } => write!(&mut w, "<code>{}</code>", HtmlEscape(value))?,
            Element::FnDef(_fn_def) => (),
            Element::Clock(_clock) => (),
            Element::Comment(_) => (),
            Element::FixedWidth(fixed_width) => write!(
                w,
                "<pre class=\"example\">{}</pre>",
                HtmlEscape(&fixed_width.value)
            )?,
            Element::Keyword(_keyword) => (),
            Element::Drawer(_drawer) => (),
            Element::Rule(_) => write!(w, "<hr>")?,
            Element::Cookie(cookie) => write!(w, "<code>{}</code>", cookie.value)?,
            Element::Title(title) => {
                write!(w, "<h{}>", if title.level <= 6 { title.level } else { 6 })?;
            }
            Element::Table(Table::TableEl { .. }) => (),
            Element::Table(Table::Org { has_header, .. }) => {
                write!(w, "<table>")?;
                if *has_header {
                    write!(w, "<thead>")?;
                } else {
                    write!(w, "<tbody>")?;
                }
            }
            Element::TableRow(row) => match row {
                TableRow::Body => write!(w, "<tr>")?,
                TableRow::BodyRule => write!(w, "</tbody><tbody>")?,
                TableRow::Header => write!(w, "<tr>")?,
                TableRow::HeaderRule => write!(w, "</thead><tbody>")?,
            },
            Element::TableCell(cell) => match cell {
                TableCell::Body => write!(w, "<td>")?,
                TableCell::Header => write!(w, "<th>")?,
            },
        }

        Ok(())
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element, _ancestors: Vec<&Element>) -> IOResult<()> {
        match element {
            // container elements
            Element::SpecialBlock(_) => (),
            Element::QuoteBlock(_) => write!(w, "</blockquote>")?,
            Element::CenterBlock(_) => write!(w, "</div>")?,
            Element::VerseBlock(_) => write!(w, "</p>")?,
            Element::Bold => write!(w, "</b>")?,
            Element::Document { .. } => write!(w, "</main>")?,
            Element::DynBlock(_dyn_block) => (),
            Element::Headline { .. } => (),
            Element::List(list) => {
                if list.ordered {
                    write!(w, "</ol>")?;
                } else {
                    write!(w, "</ul>")?;
                }
            }
            Element::Italic => write!(w, "</i>")?,
            Element::ListItem(_) => write!(w, "</li>")?,
            Element::Paragraph { .. } => write!(w, "</p>")?,
            Element::Section => write!(w, "</section>")?,
            Element::Strike => write!(w, "</s>")?,
            Element::Underline => write!(w, "</u>")?,
            Element::Title(title) => {
                write!(w, "</h{}>", if title.level <= 6 { title.level } else { 6 })?
            }
            Element::Table(Table::TableEl { .. }) => (),
            Element::Table(Table::Org { .. }) => {
                write!(w, "</tbody></table>")?;
            }
            Element::TableRow(TableRow::Body) | Element::TableRow(TableRow::Header) => {
                write!(w, "</tr>")?;
            }
            Element::TableCell(cell) => match cell {
                TableCell::Body => write!(w, "</td>")?,
                TableCell::Header => write!(w, "</th>")?,
            },
            // non-container elements
            _ => debug_assert!(!element.is_container()),
        }

        Ok(())
    }
}

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
    /// use orgize::export::{DefaultHtmlHandler, SyntectHtmlHandler};
    ///
    /// let mut handler = SyntectHtmlHandler::new(DefaultHtmlHandler);
    /// let org = Org::parse("src_rust{println!(\"Hello\")}");
    ///
    /// let mut vec = vec![];
    ///
    /// org.write_html_custom(&mut vec, &mut handler).unwrap();
    /// ```
    ///
    /// Customize:
    ///
    /// ```rust,no_run
    /// // orgize has re-exported the whole syntect crate
    /// use orgize::syntect::parsing::SyntaxSet;
    /// use orgize::export::{DefaultHtmlHandler, SyntectHtmlHandler};
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
    /// // Make sure to check if theme presents or it will panic at runtime
    /// if handler.theme_set.themes.contains_key("dont-exists") {
    ///
    /// }
    /// ```
    pub struct SyntectHtmlHandler<E: From<Error>, H: ExportHandler<E>> {
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

    impl<E: From<Error>, H: ExportHandler<E>> SyntectHtmlHandler<E, H> {
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

    impl<E: From<Error>, H: ExportHandler<E>> Default for SyntectHtmlHandler<E, H> {
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

    impl<E: From<Error>, H: ExportHandler<E>> ExportHandler<E> for SyntectHtmlHandler<E, H> {
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
                    self.highlight(None, &fixed_width.value)
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
