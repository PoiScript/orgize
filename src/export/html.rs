use rowan::{NodeOrToken, WalkEvent};
use std::cmp::min;
use std::fmt;
use std::fmt::Write as _;

use super::TraversalContext;
use super::Traverser;
use crate::ast::*;
use crate::syntax::SyntaxToken;
use crate::SyntaxKind;

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
}

impl Traverser for HtmlExport {
    #[tracing::instrument(skip(self, _ctx))]
    fn text(&mut self, token: SyntaxToken, _ctx: &mut TraversalContext) {
        self.output += &HtmlEscape(token.text()).to_string();
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn document(&mut self, event: WalkEvent<&Document>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<main>",
            WalkEvent::Leave(_) => "</main>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn list(&mut self, event: WalkEvent<&List>, _ctx: &mut TraversalContext) {
        match event {
            WalkEvent::Enter(list) => {
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
            WalkEvent::Leave(list) => {
                self.output += if list.is_ordered() {
                    "</ol>"
                } else if let Some(true) = self.in_descriptive_list.last() {
                    "</dl>"
                } else {
                    "</ul>"
                };
                self.in_descriptive_list.pop();
            }
        };
    }

    #[tracing::instrument(skip(self, ctx))]
    fn list_item(&mut self, event: WalkEvent<&ListItem>, ctx: &mut TraversalContext) {
        if self.in_descriptive_list.last().copied().unwrap_or_default() {
            match event {
                WalkEvent::Enter(item) => {
                    self.output += "<dt>";
                    for elem in item.tag() {
                        match elem {
                            NodeOrToken::Node(n) => self.node(n, ctx),
                            NodeOrToken::Token(t) => self.token(t, ctx),
                        }
                    }
                    self.output += "</dt><dd>";
                }
                WalkEvent::Leave(_) => self.output += "</dd>",
            };
        } else {
            match event {
                WalkEvent::Enter(_) => self.output += "<li>",
                WalkEvent::Leave(_) => self.output += "</li>",
            };
        }
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn paragraph(&mut self, event: WalkEvent<&Paragraph>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<p>",
            WalkEvent::Leave(_) => "</p>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn section(&mut self, event: WalkEvent<&Section>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<section>",
            WalkEvent::Leave(_) => "</section>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn fixed_width(&mut self, event: WalkEvent<&FixedWidth>, _ctx: &mut TraversalContext) {
        if let WalkEvent::Enter(_f) = event {
            // self.output += f.text();
        };
    }

    #[tracing::instrument(skip(self, ctx))]
    fn snippet(&mut self, event: WalkEvent<&Snippet>, ctx: &mut TraversalContext) {
        if let WalkEvent::Enter(snippet) = event {
            if matches!(snippet.name(), Some(name) if name.text().eq_ignore_ascii_case("html")) {
                if let Some(value) = snippet.value() {
                    self.output += value.text()
                }
            }
            return ctx.skip();
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn italic(&mut self, event: WalkEvent<&Italic>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<i>",
            WalkEvent::Leave(_) => "</i>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn bold(&mut self, event: WalkEvent<&Bold>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<b>",
            WalkEvent::Leave(_) => "</b>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn strike(&mut self, event: WalkEvent<&Strike>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<s>",
            WalkEvent::Leave(_) => "</s>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn underline(&mut self, event: WalkEvent<&Underline>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<u>",
            WalkEvent::Leave(_) => "</u>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn verbatim(&mut self, event: WalkEvent<&Verbatim>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<code>",
            WalkEvent::Leave(_) => "</code>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn code(&mut self, event: WalkEvent<&Code>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<code>",
            WalkEvent::Leave(_) => "</code>",
        };
    }

    #[tracing::instrument(skip(self, ctx))]
    fn rule(&mut self, event: WalkEvent<&Rule>, ctx: &mut TraversalContext) {
        if let WalkEvent::Enter(_) = event {
            self.output += "<hr/>"
        };
        ctx.skip()
    }

    #[tracing::instrument(skip(self, ctx))]
    fn link(&mut self, event: WalkEvent<&Link>, ctx: &mut TraversalContext) {
        match event {
            WalkEvent::Enter(link) => {
                let path = link.path();
                let path = path.as_ref().map(|path| path.text()).unwrap_or_default();

                if link.is_image() {
                    let _ = write!(&mut self.output, r#"<img src="{}">"#, HtmlEscape(path));
                    return ctx.skip();
                }

                let _ = write!(&mut self.output, r#"<a href="{}">"#, HtmlEscape(path));

                if !link.has_description() {
                    let _ = write!(&mut self.output, "{}</a>", HtmlEscape(path));
                    return ctx.skip();
                }
            }
            WalkEvent::Leave(_) => {
                self.output += "</a>";
            }
        }
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn quote_block(&mut self, event: WalkEvent<&QuoteBlock>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<blockquote>",
            WalkEvent::Leave(_) => "</blockquote>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn verse_block(&mut self, event: WalkEvent<&VerseBlock>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<p class=\"verse\">",
            WalkEvent::Leave(_) => "</p>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn example_block(&mut self, event: WalkEvent<&ExampleBlock>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<pre class=\"example\">",
            WalkEvent::Leave(_) => "</pre>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn center_block(&mut self, event: WalkEvent<&CenterBlock>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<div class=\"center\">",
            WalkEvent::Leave(_) => "</div>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn org_table(&mut self, event: WalkEvent<&OrgTable>, _ctx: &mut TraversalContext) {
        match event {
            WalkEvent::Enter(table) => {
                self.output += "<table>";
                self.table_row = if table.has_header() {
                    TableRow::HeaderRule
                } else {
                    TableRow::BodyRule
                }
            }
            WalkEvent::Leave(_) => {
                match self.table_row {
                    TableRow::Body => self.output += "</tbody>",
                    TableRow::Header => self.output += "</thead>",
                    _ => {}
                }
                self.output += "</table>";
            }
        }
    }

    #[tracing::instrument(skip(self, ctx))]
    fn org_table_row(&mut self, event: WalkEvent<&OrgTableRow>, ctx: &mut TraversalContext) {
        if match event {
            WalkEvent::Enter(n) | WalkEvent::Leave(n) => n.is_rule(),
        } {
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
            return ctx.skip();
        }

        match event {
            WalkEvent::Enter(_) => {
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
            WalkEvent::Leave(_) => {
                self.output += "</tr>";
            }
        }
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn org_table_cell(&mut self, event: WalkEvent<&OrgTableCell>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<td>",
            WalkEvent::Leave(_) => "</td>",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn comment(&mut self, event: WalkEvent<&Comment>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<!--",
            WalkEvent::Leave(_) => "-->",
        };
    }

    #[tracing::instrument(skip(self, _ctx))]
    fn comment_block(&mut self, event: WalkEvent<&CommentBlock>, _ctx: &mut TraversalContext) {
        self.output += match event {
            WalkEvent::Enter(_) => "<!--",
            WalkEvent::Leave(_) => "-->",
        };
    }

    #[tracing::instrument(skip(self, ctx))]
    fn headline(&mut self, event: WalkEvent<&Headline>, ctx: &mut TraversalContext) {
        if let WalkEvent::Enter(headline) = event {
            let level = min(headline.level(), 6);
            let _ = write!(&mut self.output, "<h{level}>");
            for elem in headline.title() {
                match elem {
                    NodeOrToken::Node(node) => self.node(node, ctx),
                    NodeOrToken::Token(token) => self.token(token, ctx),
                }
            }
            let _ = write!(&mut self.output, "</h{level}>");
        }
    }

    #[tracing::instrument(skip(self, ctx))]
    fn inline_src(&mut self, _event: WalkEvent<&InlineSrc>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn inline_call(&mut self, _event: WalkEvent<&InlineCall>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn special_block(&mut self, _event: WalkEvent<&SpecialBlock>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn export_block(&mut self, _event: WalkEvent<&ExportBlock>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn source_block(&mut self, _event: WalkEvent<&SourceBlock>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn babel_call(&mut self, _event: WalkEvent<&BabelCall>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn clock(&mut self, _event: WalkEvent<&Clock>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn cookie(&mut self, _event: WalkEvent<&Cookie>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn radio_target(&mut self, _event: WalkEvent<&RadioTarget>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn drawer(&mut self, _event: WalkEvent<&Drawer>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn dyn_block(&mut self, _event: WalkEvent<&DynBlock>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn fn_def(&mut self, _event: WalkEvent<&FnDef>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn fn_ref(&mut self, _event: WalkEvent<&FnRef>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn macros(&mut self, _event: WalkEvent<&Macros>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn timestamp(&mut self, event: WalkEvent<&Timestamp>, ctx: &mut TraversalContext) {
        if let WalkEvent::Enter(t) = event {
            self.output += r#"<span class="timestamp-wrapper"><span class="timestamp">"#;
            for e in t.syntax.children_with_tokens() {
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
            ctx.skip();
        }
    }

    #[tracing::instrument(skip(self, ctx))]
    fn target(&mut self, _event: WalkEvent<&Target>, ctx: &mut TraversalContext) {
        ctx.skip();
    }

    #[tracing::instrument(skip(self, ctx))]
    fn latex_fragment(&mut self, event: WalkEvent<&LatexFragment>, ctx: &mut TraversalContext) {
        if let WalkEvent::Enter(l) = event {
            self.output += &l.syntax.to_string();
            ctx.skip();
        }
    }

    #[tracing::instrument(skip(self, ctx))]
    fn latex_environment(
        &mut self,
        event: WalkEvent<&LatexEnvironment>,
        ctx: &mut TraversalContext,
    ) {
        if let WalkEvent::Enter(l) = event {
            self.output += &l.syntax.to_string();
            ctx.skip();
        }
    }

    #[tracing::instrument(skip(self, ctx))]
    fn entity(&mut self, event: WalkEvent<&Entity>, ctx: &mut TraversalContext) {
        if let WalkEvent::Enter(e) = event {
            self.output += e.html();
            ctx.skip();
        }
    }
}
