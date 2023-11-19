//! ```bash
//! cargo run --example html-slugify '* hello world!'
//! ```

use orgize::{
    ast::Headline,
    export::{HtmlExport, TraversalContext, Traverser},
    forward_handler,
    rowan::WalkEvent,
    Org,
};
use rowan::NodeOrToken;
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
    fn headline(&mut self, event: WalkEvent<&Headline>, ctx: &mut TraversalContext) {
        if let WalkEvent::Enter(headline) = event {
            let level = min(headline.level(), 6);
            let title = headline.title().map(|e| e.to_string()).collect::<String>();
            self.0.push_str(format!(
                "<h{level}><a id=\"{0}\" href=\"#{0}\">",
                slugify!(&title)
            ));
            for elem in headline.title() {
                match elem {
                    NodeOrToken::Node(node) => self.node(node, ctx),
                    NodeOrToken::Token(token) => self.token(token, ctx),
                }
            }
            self.0.push_str(format!("</a></h{level}>"));
        }
    }

    forward_handler! {
        HtmlExport,
        link text document paragraph section rule comment
        inline_src inline_call code bold verbatim italic strike underline list list_item
        special_block quote_block center_block verse_block comment_block example_block export_block
        source_block babel_call clock cookie radio_target drawer dyn_block fn_def fn_ref macros
        snippet timestamp target fixed_width org_table org_table_row org_table_cell latex_fragment
        latex_environment
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
