//! ```bash
//! cargo run --example html-slugify '* hello world!'
//! ```

use orgize::{
    export::{Container, Event, HtmlExport, TraversalContext, Traverser},
    Org,
};
use slugify::slugify;
use std::cmp::min;
use std::env::args;

#[derive(Default)]
struct MyHtmlHandler(pub HtmlExport);

impl Traverser for MyHtmlHandler {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        if let Event::Enter(Container::Headline(headline)) = event {
            let level = min(headline.level(), 6);
            let title = headline.title().map(|e| e.to_string()).collect::<String>();
            self.0.push_str(format!(
                "<h{level}><a id=\"{0}\" href=\"#{0}\">",
                slugify!(&title)
            ));
            for elem in headline.title() {
                self.element(elem, ctx);
            }
            self.0.push_str(format!("</a></h{level}>"));
        } else {
            // forwrad to default html export
            self.0.event(event, ctx);
        }
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
