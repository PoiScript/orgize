//! ```bash
//! cargo run --example html-slugify '* hello world!'
//! ```

use orgize::{
    export::HtmlExport,
    export::{from_fn_with_ctx, Container, Event, Traverser},
    Org,
};
use slugify::slugify;
use std::cmp::min;
use std::env::args;

fn main() {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <org-mode-string>", args[0]);
    } else {
        let mut html_export = HtmlExport::default();

        let mut handler = from_fn_with_ctx(|event, ctx| {
            if let Event::Enter(Container::Headline(headline)) = event {
                let level = min(headline.level(), 6);
                let title = headline.title().map(|e| e.to_string()).collect::<String>();
                html_export.push_str(format!(
                    "<h{level}><a id=\"{0}\" href=\"#{0}\">",
                    slugify!(&title)
                ));
                for elem in headline.title() {
                    html_export.element(elem, ctx);
                }
                html_export.push_str(format!("</a></h{level}>"));
            } else {
                // forward to default html export
                html_export.event(event, ctx);
            }
        });

        Org::parse(&args[1]).traverse(&mut handler);

        println!("{}", html_export.finish());
    }
}
