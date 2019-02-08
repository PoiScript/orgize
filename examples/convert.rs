use std::env;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;

use orgize::export::{HtmlHandler, Render};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <org-file>", args[0]);
        return;
    }

    let mut file = File::open(&args[1]).expect(&format!("file {} not found", &args[1]));

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let cursor = Cursor::new(Vec::new());
    let handler = HtmlHandler;
    let mut render = Render::new(handler, cursor, &contents);

    render
        .render()
        .expect("something went wrong rendering the file");
    println!(
        "{}",
        String::from_utf8(render.into_wirter().into_inner()).expect("invalid utf-8")
    );
}
