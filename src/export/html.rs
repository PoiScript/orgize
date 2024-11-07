use rowan::ast::AstNode;
use rowan::NodeOrToken;
use std::cmp::min;
use std::fmt;
use std::fmt::Write as _;

use super::event::{Container, Event};
use super::TraversalContext;
use super::Traverser;
use crate::ast::token;
use crate::{SyntaxElement, SyntaxKind, SyntaxNode};

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

        while let Some(off) = jetscii::bytes!(b'<', b'>', b'&', b'\'', b'"').find(&bytes[pos..]) {
            write!(f, "{}", &content[pos..pos + off])?;

            pos += off + 1;

            match bytes[pos - 1] {
                b'<' => write!(f, "&lt;")?,
                b'>' => write!(f, "&gt;")?,
                b'&' => write!(f, "&amp;")?,
                b'\'' => write!(f, "&apos;")?,
                b'"' => write!(f, "&quot;")?,
                _ => {}
            }
        }

        write!(f, "{}", &content[pos..])
    }
}

#[derive(Default)]
pub struct HtmlExport {
    output: String,

    ///TODO: track footnotes and citations within the export struct and
    /// construct them after the document is fully parsed?
    //footnotes: HashMap<String, String>,
    in_descriptive_list: Vec<bool>,

    table_row: TableRow,
}

#[derive(Default, PartialEq, Eq)]
enum TableRow {
    #[default]
    HeaderRule,
    Header,
    BodyRule,
    Body,
}

impl HtmlExport {
    pub fn push_str(&mut self, s: impl AsRef<str>) {
        self.output += s.as_ref();
    }

    pub fn finish(self) -> String {
        self.output
    }

    /// Render syntax node to html string
    ///
    /// ```rust
    /// use orgize::{Org, ast::Bold, export::HtmlExport, rowan::ast::AstNode};
    ///
    /// let org = Org::parse("* /hello/ *world*");
    /// let bold = org.first_node::<Bold>().unwrap();
    /// let mut html = HtmlExport::default();
    /// html.render(bold.syntax());
    /// assert_eq!(html.finish(), "<b>world</b>");
    /// ```
    pub fn render(&mut self, node: &SyntaxNode) {
        let mut ctx = TraversalContext::default();
        self.element(SyntaxElement::Node(node.clone()), &mut ctx);
    }
}

impl Traverser for HtmlExport {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        match event {
            Event::Enter(Container::Document(_)) => self.output += "<main>",
            Event::Leave(Container::Document(_)) => self.output += "</main>",

            Event::Enter(Container::Headline(headline)) => {
                let level = min(headline.level(), 6);
                let _ = write!(&mut self.output, "<h{level}>");
                for elem in headline.title() {
                    self.element(elem, ctx);
                }
                let _ = write!(&mut self.output, "</h{level}>");
            }
            Event::Leave(Container::Headline(_)) => {}

            Event::Enter(Container::FnRef(t)) => {
                if let Some(label) = t.label() {
                    let _ = write!(
                        &mut self.output,
                        "<a href=\"#footnote_{}\" class=\"footnote-reference\">[{}]",
                        label.syntax().text(),
                        label.syntax().text()
                    );
                }
                self.output += "</a>";
            }
            Event::Leave(Container::FnRef(_)) => {}

            Event::Enter(Container::FnDef(t)) => {
                self.output += "<aside ";
                self.output += r#"class="footnote-definition" "#;
                self.output += ">";

                if let Some(label) = t.label() {
                    self.output += "<a ";
                    let _ = write!(
                        &mut self.output,
                        "href=\"#footnote_{}\" ",
                        label.syntax().text()
                    );
                    self.output += "class=\"footnote-reference\" ";
                    self.output += ">";
                    let _ = write!(&mut self.output, "[{}]", label.syntax().text());
                    self.output += "</a>";
                }
            }
            Event::Leave(Container::FnDef(_)) => {
                self.output += "</aside>";
            }

            Event::Enter(Container::FnContent(c)) => {
                self.output += "<span class=\"footnote-content\" ";
                if let Some(parent) = c.syntax().parent() {
                    if parent.kind() == SyntaxKind::FN_REF || parent.kind() == SyntaxKind::FN_DEF {
                        let label = token(&parent, SyntaxKind::FN_LABEL).unwrap();
                        let _ = write!(&mut self.output, "id=\"footnote_{}\" ", label);
                    }
                }
                self.output += ">";
            }
            Event::Leave(Container::FnContent(_)) => {
                self.output += "</span>";
            }

            Event::Enter(Container::Paragraph(_)) => self.output += "<p>",
            Event::Leave(Container::Paragraph(_)) => self.output += "</p>",

            Event::Enter(Container::Section(_)) => self.output += "<section>",
            Event::Leave(Container::Section(_)) => self.output += "</section>",

            Event::Enter(Container::Italic(_)) => self.output += "<i>",
            Event::Leave(Container::Italic(_)) => self.output += "</i>",

            Event::Enter(Container::Bold(_)) => self.output += "<b>",
            Event::Leave(Container::Bold(_)) => self.output += "</b>",

            Event::Enter(Container::Strike(_)) => self.output += "<s>",
            Event::Leave(Container::Strike(_)) => self.output += "</s>",

            Event::Enter(Container::Underline(_)) => self.output += "<u>",
            Event::Leave(Container::Underline(_)) => self.output += "</u>",

            Event::Enter(Container::Verbatim(_)) => self.output += "<code>",
            Event::Leave(Container::Verbatim(_)) => self.output += "</code>",

            Event::Enter(Container::Code(_)) => self.output += "<code>",
            Event::Leave(Container::Code(_)) => self.output += "</code>",

            Event::Enter(Container::SourceBlock(block)) => {
                if let Some(language) = block.language() {
                    let _ = write!(
                        &mut self.output,
                        r#"<pre><code class="language-{}">"#,
                        HtmlEscape(&language)
                    );
                } else {
                    self.output += r#"<pre><code>"#
                }
            }
            Event::Leave(Container::SourceBlock(_)) => self.output += "</code></pre>",

            Event::Enter(Container::QuoteBlock(_)) => self.output += "<blockquote>",
            Event::Leave(Container::QuoteBlock(_)) => self.output += "</blockquote>",

            Event::Enter(Container::VerseBlock(_)) => self.output += "<p class=\"verse\">",
            Event::Leave(Container::VerseBlock(_)) => self.output += "</p>",

            Event::Enter(Container::ExampleBlock(_)) => self.output += "<pre class=\"example\">",
            Event::Leave(Container::ExampleBlock(_)) => self.output += "</pre>",

            Event::Enter(Container::CenterBlock(_)) => self.output += "<div class=\"center\">",
            Event::Leave(Container::CenterBlock(_)) => self.output += "</div>",

            Event::Enter(Container::CommentBlock(_)) => self.output += "<!--",
            Event::Leave(Container::CommentBlock(_)) => self.output += "-->",

            Event::Enter(Container::Comment(_)) => self.output += "<!--",
            Event::Leave(Container::Comment(_)) => self.output += "-->",

            Event::Enter(Container::Subscript(_)) => self.output += "<sub>",
            Event::Leave(Container::Subscript(_)) => self.output += "</sub>",

            Event::Enter(Container::Superscript(_)) => self.output += "<sup>",
            Event::Leave(Container::Superscript(_)) => self.output += "</sup>",

            Event::Enter(Container::List(list)) => {
                self.output += if list.is_ordered() {
                    self.in_descriptive_list.push(false);
                    "<ol>"
                } else if list.is_descriptive() {
                    self.in_descriptive_list.push(true);
                    "<dl>"
                } else {
                    self.in_descriptive_list.push(false);
                    "<ul>"
                };
            }
            Event::Leave(Container::List(list)) => {
                self.output += if list.is_ordered() {
                    "</ol>"
                } else if let Some(true) = self.in_descriptive_list.last() {
                    "</dl>"
                } else {
                    "</ul>"
                };
                self.in_descriptive_list.pop();
            }
            Event::Enter(Container::ListItem(list_item)) => {
                if let Some(&true) = self.in_descriptive_list.last() {
                    self.output += "<dt>";
                    for elem in list_item.tag() {
                        self.element(elem, ctx);
                    }
                    self.output += "</dt><dd>";
                } else {
                    self.output += "<li>";
                }
            }
            Event::Leave(Container::ListItem(_)) => {
                if let Some(&true) = self.in_descriptive_list.last() {
                    self.output += "</dd>";
                } else {
                    self.output += "</li>";
                }
            }

            Event::Enter(Container::OrgTable(table)) => {
                self.output += "<table>";
                self.table_row = if table.has_header() {
                    TableRow::HeaderRule
                } else {
                    TableRow::BodyRule
                }
            }
            Event::Leave(Container::OrgTable(_)) => {
                match self.table_row {
                    TableRow::Body => self.output += "</tbody>",
                    TableRow::Header => self.output += "</thead>",
                    _ => {}
                }
                self.output += "</table>";
            }
            Event::Enter(Container::OrgTableRow(row)) => {
                if row.is_rule() {
                    match self.table_row {
                        TableRow::Body => {
                            self.output += "</tbody>";
                            self.table_row = TableRow::BodyRule;
                        }
                        TableRow::Header => {
                            self.output += "</thead>";
                            self.table_row = TableRow::BodyRule;
                        }
                        _ => {}
                    }
                    ctx.skip();
                } else {
                    match self.table_row {
                        TableRow::HeaderRule => {
                            self.table_row = TableRow::Header;
                            self.output += "<thead>";
                        }
                        TableRow::BodyRule => {
                            self.table_row = TableRow::Body;
                            self.output += "<tbody>";
                        }
                        _ => {}
                    }
                    self.output += "<tr>";
                }
            }
            Event::Leave(Container::OrgTableRow(row)) => {
                if row.is_rule() {
                    match self.table_row {
                        TableRow::Body => {
                            self.output += "</tbody>";
                            self.table_row = TableRow::BodyRule;
                        }
                        TableRow::Header => {
                            self.output += "</thead>";
                            self.table_row = TableRow::BodyRule;
                        }
                        _ => {}
                    }
                    ctx.skip();
                } else {
                    self.output += "</tr>";
                }
            }
            Event::Enter(Container::OrgTableCell(_)) => self.output += "<td>",
            Event::Leave(Container::OrgTableCell(_)) => self.output += "</td>",

            Event::Enter(Container::Link(link)) => {
                let path = link.path();
                let path = path.trim_start_matches("file:");

                if link.is_image() {
                    let _ = write!(&mut self.output, r#"<img src="{}">"#, HtmlEscape(&path));
                    return ctx.skip();
                }

                let _ = write!(&mut self.output, r#"<a href="{}">"#, HtmlEscape(&path));

                if !link.has_description() {
                    let _ = write!(&mut self.output, "{}</a>", HtmlEscape(&path));
                    ctx.skip();
                }
            }
            Event::Leave(Container::Link(_)) => self.output += "</a>",

            Event::Text(text) => {
                let _ = write!(&mut self.output, "{}", HtmlEscape(text));
            }

            Event::FnLabel(_) => {}

            Event::LineBreak(_) => self.output += "<br/>",

            Event::Snippet(snippet) => {
                if snippet.backend().eq_ignore_ascii_case("html") {
                    self.output += &snippet.value();
                }
            }

            Event::Rule(_) => self.output += "<hr/>",

            Event::Timestamp(timestamp) => {
                self.output += r#"<span class="timestamp-wrapper"><span class="timestamp">"#;
                for e in timestamp.syntax.children_with_tokens() {
                    match e {
                        NodeOrToken::Token(t) if t.kind() == SyntaxKind::MINUS2 => {
                            self.output += "&#x2013;";
                        }
                        NodeOrToken::Token(t) => {
                            self.output += t.text();
                        }
                        _ => {}
                    }
                }
                self.output += r#"</span></span>"#;
            }

            Event::LatexFragment(latex) => {
                let _ = write!(&mut self.output, "{}", &latex.syntax);
            }
            Event::LatexEnvironment(latex) => {
                let _ = write!(&mut self.output, "{}", &latex.syntax);
            }

            // ignores keyword
            Event::Enter(Container::Keyword(_)) => ctx.skip(),

            Event::Entity(entity) => self.output += entity.html(),

            _ => {}
        }
    }
}
