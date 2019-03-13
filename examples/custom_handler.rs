use orgize::export::*;
use orgize::headline::Headline;
use slugify::slugify;
use std::convert::From;
use std::env::args;
use std::fs::File;
use std::io::{Cursor, Error as IOError, Read, Write};
use std::string::FromUtf8Error;

struct CustomHtmlHandler;

#[derive(Debug)]
enum Error {
    IO(IOError),
    Heading,
    Utf8(FromUtf8Error),
}

// From<std::io::Error> trait is required
impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::IO(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8(err)
    }
}

type Result = std::result::Result<(), Error>;

impl<W: Write> HtmlHandler<W, Error> for CustomHtmlHandler {
    fn handle_headline_beg(&mut self, w: &mut W, hdl: Headline) -> Result {
        if hdl.level > 6 {
            Err(Error::Heading)
        } else {
            Ok(write!(
                w,
                r##"<h{0}><a class="anchor" href="#{1}">{2}</a></h{0}>"##,
                hdl.level,
                slugify!(hdl.title),
                hdl.title,
            )?)
        }
    }
}

fn main() -> Result {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        println!("Usage: {} <org-file>", args[0]);
    } else {
        let mut file = File::open(&args[1])?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut cursor = Cursor::new(Vec::new());

        //let mut render = DefaultHtmlRender::new(cursor, &contents);
        // comment the following line and uncomment the line above to use the default handler
        let mut render = HtmlRender::new(CustomHtmlHandler, &mut cursor, &contents);

        render.render()?;

        println!("{}", String::from_utf8(cursor.into_inner())?);
    }

    Ok(())
}
