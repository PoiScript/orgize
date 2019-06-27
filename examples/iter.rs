use orgize::Org;
use std::env::args;
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <org-file>", args[0]);
    } else {
        let contents = String::from_utf8(fs::read(&args[1])?).unwrap();

        for event in Org::parse(&contents).iter() {
            println!("{:?}", event);
        }
    }
    Ok(())
}
