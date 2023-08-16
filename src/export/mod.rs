//! Export `Org` struct to various formats.

mod html;
mod org;

#[cfg(feature = "syntect")]
pub use html::SyntectHtmlHandler;
pub use html::{DefaultHtmlHandler, HtmlEscape};
pub use org::{DefaultOrgHandler};

use std::io::{Error, Write};

use crate::elements::{Datetime, Element};

pub trait ExportHandler<E: From<Error>> {
    fn start<W: Write>(&mut self, writer: W, element: &Element, ancestors: Vec<&Element>) -> Result<(), E>;
    fn end<W: Write>(&mut self, writer: W, element: &Element, ancestors: Vec<&Element>) -> Result<(), E>;
}


pub(crate) fn write_datetime<W: Write>(
    mut w: W,
    start: &str,
    datetime: &Datetime,
    end: &str,
) -> Result<(), Error> {
    write!(w, "{}", start)?;
    write!(
        w,
        "{}-{:02}-{:02} {}",
        datetime.year, datetime.month, datetime.day, datetime.dayname
    )?;
    if let (Some(hour), Some(minute)) = (datetime.hour, datetime.minute) {
        write!(w, " {:02}:{:02}", hour, minute)?;
    }
    write!(w, "{}", end)
}
