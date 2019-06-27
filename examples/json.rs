use orgize::Org;
use serde_json::to_string;
use std::env::args;
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <org-file>", args[0]);
    } else {
        let contents = String::from_utf8(fs::read(&args[1])?).unwrap();
        println!("{}", to_string(&Org::parse(&contents)).unwrap());
    }
    Ok(())
}
