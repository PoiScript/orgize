use std::convert::From;
use std::env::args;
use std::fs;
use std::io::{Error as IOError, Write};
use std::result::Result;
use std::string::FromUtf8Error;

use orgize::export::{html::Escape, DefaultHtmlHandler, HtmlHandler};
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

struct MyHtmlHandler;

impl HtmlHandler<MyError> for MyHtmlHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), MyError> {
        let mut default_handler = DefaultHtmlHandler;
        match element {
            Element::Headline(headline) => {
                if headline.level > 6 {
                    return Err(MyError::Heading);
                } else {
                    let slugify = slugify!(headline.title);
                    write!(
                        w,
                        "<h{0}><a id=\"{1}\" href=\"#{1}\">{2}</a></h{0}>",
                        headline.level,
                        slugify,
                        Escape(headline.title),
                    )?;
                }
            }
            // fallthrough to default handler
            _ => default_handler.start(w, element)?,
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
        Org::parse(&contents).html_with_handler(&mut writer, MyHtmlHandler)?;

        println!("{}", String::from_utf8(writer)?);
    }

    Ok(())
}
