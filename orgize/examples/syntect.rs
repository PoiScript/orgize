use std::env::args;
use std::fs;
use std::io::{Error, Result, Write};
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    html::{styled_line_to_highlighted_html, IncludeBackground},
    parsing::SyntaxSet,
};

use orgize::export::{DefaultHtmlHandler, HtmlHandler};
use orgize::{Element, Org};

pub struct SyntectHtmlHandler {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    default_hanlder: DefaultHtmlHandler,
}

impl Default for SyntectHtmlHandler {
    fn default() -> Self {
        SyntectHtmlHandler {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            default_hanlder: DefaultHtmlHandler,
        }
    }
}

impl SyntectHtmlHandler {
    fn highlight(&self, language: Option<&str>, content: &str) -> String {
        let mut highlighter = HighlightLines::new(
            language
                .and_then(|lang| self.syntax_set.find_syntax_by_token(lang))
                .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text()),
            &self.theme_set.themes["InspiredGitHub"],
        );
        let regions = highlighter.highlight(content, &self.syntax_set);
        styled_line_to_highlighted_html(&regions[..], IncludeBackground::No)
    }
}

impl HtmlHandler<Error> for SyntectHtmlHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<()> {
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
            _ => self.default_hanlder.start(w, element)?,
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <org-file>", args[0]);
    } else {
        let contents = String::from_utf8(fs::read(&args[1]).unwrap()).unwrap();

        let mut writer = Vec::new();
        Org::parse(&contents)
            .html_with_handler(&mut writer, SyntectHtmlHandler::default())
            .unwrap();

        println!("{}", String::from_utf8(writer).unwrap());
    }
}
