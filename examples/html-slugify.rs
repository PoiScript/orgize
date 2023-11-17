//! ```bash
//! cargo run --example html-slugify '* hello world!'
//! ```

use orgize::{
    ast::HeadlineTitle,
    export::{HtmlExport, TraversalContext, Traverser},
    forward_handler,
    rowan::{ast::AstNode, WalkEvent},
    Org,
};
use slugify::slugify;
use std::cmp::min;
use std::env::args;

#[derive(Default)]
struct MyHtmlHandler(pub HtmlExport);

// AsMut trait is required for using forward_handler macros
impl AsMut<HtmlExport> for MyHtmlHandler {
    fn as_mut(&mut self) -> &mut HtmlExport {
        &mut self.0
    }
}

impl Traverser for MyHtmlHandler {
    fn headline_title(&mut self, event: WalkEvent<&HeadlineTitle>, _ctx: &mut TraversalContext) {
        match event {
            WalkEvent::Enter(title) => {
                let level = title.headline().map(|h| min(h.level(), 6)).unwrap_or(1);
                let raw = title.syntax().to_string();
                self.0.push_str(format!(
                    "<h{level}><a id=\"{0}\" href=\"#{0}\">",
                    slugify!(&raw)
                ));
            }
            WalkEvent::Leave(title) => {
                let level = title.headline().map(|h| min(h.level(), 6)).unwrap_or(1);
                self.0.push_str(format!("</a></h{level}>"));
            }
        }
    }

    forward_handler! {
        HtmlExport,
        link text document headline paragraph section rule comment
        inline_src inline_call code bold verbatim italic strike underline list list_item list_item_tag
        special_block quote_block center_block verse_block comment_block example_block export_block
        source_block babel_call clock cookie radio_target drawer dyn_block fn_def fn_ref macros
        snippet timestamp target fixed_width org_table org_table_row org_table_cell list_item_content
    }
}

fn main() {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <org-mode-string>", args[0]);
    } else {
        let mut handler = MyHtmlHandler::default();
        Org::parse(&args[1]).traverse(&mut handler);

        println!("{}", handler.0.finish());
    }
}
