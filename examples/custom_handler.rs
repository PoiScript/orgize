use std::env::args;
use std::fs::File;
use std::io::{Cursor, Read, Result, Write};

use orgize::export::*;
use orgize::headline::Headline;
use slugify::slugify;

struct CustomHtmlHandler;

impl<W: Write> HtmlHandler<W> for CustomHtmlHandler {
    fn handle_headline_beg(&mut self, w: &mut W, hdl: Headline) -> Result<()> {
        write!(
            w,
            r##"<h{0}><a class="anchor" href="#{1}">{2}</a></h{0}>"##,
            if hdl.level <= 6 { hdl.level } else { 6 },
            slugify!(hdl.title),
            hdl.title,
        )
    }
}

fn main() -> Result<()> {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        println!("Usage: {} <org-file>", args[0]);
    } else {
        let mut file = File::open(&args[1])?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let cursor = Cursor::new(Vec::new());

        //let mut render = DefaultHtmlRender::new(cursor, &contents);
        // comment the following line and uncomment the line above to use the default handler
        let mut render = HtmlRender::new(CustomHtmlHandler, cursor, &contents);

        render.render()?;

        println!(
            "{}",
            String::from_utf8(render.into_wirter().into_inner()).expect("invalid utf-8")
        );
    }

    Ok(())
}
