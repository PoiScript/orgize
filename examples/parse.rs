//! ```bash
//! cargo run --example parse '* hello\n** /world/!'
//! ```

use orgize::Org;
use rowan::ast::AstNode;
use std::env::args;
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    let args: Vec<_> = args().collect();

    tracing_subscriber::fmt()
        .without_time()
        .with_file(true)
        .with_span_events(FmtSpan::NEW)
        .with_line_number(true)
        .with_max_level(tracing::Level::TRACE)
        .with_file(false)
        .with_line_number(false)
        .init();

    if args.len() < 2 {
        eprintln!("Usage: {} <org-mode-string>", args[0]);
    } else {
        let s = &args[1].replace(r"\n", "\n").replace(r"\r", "\r");
        let org = Org::parse(s);
        println!("{:#?}", org.document().syntax());
    }
}
