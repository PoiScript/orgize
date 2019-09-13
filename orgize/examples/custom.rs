use std::convert::From;
use std::env::args;
use std::fs;
use std::io::{Error as IOError, Write};
use std::result::Result;
use std::string::FromUtf8Error;

use orgize::export::{DefaultHtmlHandler, HtmlHandler};
use orgize::{Element, Org};
use slugify::slugify;

#[derive(Debug)]
enum MyError {
    IO(IOError),
    Heading,
    Utf8(FromUtf8Error),
}

// From<std::io::Error> trait is required for custom error type
impl From<IOError> for MyError {
    fn from(err: IOError) -> Self {
        MyError::IO(err)
    }
}

impl From<FromUtf8Error> for MyError {
    fn from(err: FromUtf8Error) -> Self {
        MyError::Utf8(err)
    }
}

struct MyHtmlHandler(DefaultHtmlHandler);

impl HtmlHandler<MyError> for MyHtmlHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), MyError> {
        match element {
            Element::Title(title) => {
                if title.level > 6 {
                    return Err(MyError::Heading);
                } else {
                    write!(
                        w,
                        "<h{0}><a id=\"{1}\" href=\"#{1}\">",
                        title.level,
                        slugify!(&title.raw),
                    )?;
                }
            }
            // fallthrough to default handler
            _ => self.0.start(w, element)?,
        }
        Ok(())
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), MyError> {
        match element {
            Element::Title(title) => {
                write!(w, "</a></h{}>", title.level,)?;
            }
            _ => self.0.end(w, element)?,
        }
        Ok(())
    }
}

fn main() -> Result<(), MyError> {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <org-file>", args[0]);
    } else {
        let contents = String::from_utf8(fs::read(&args[1])?)?;

        let mut writer = Vec::new();
        let mut handler = MyHtmlHandler(DefaultHtmlHandler);
        Org::parse(&contents).html_with_handler(&mut writer, &mut handler)?;

        println!("{}", String::from_utf8(writer)?);
    }

    Ok(())
}
