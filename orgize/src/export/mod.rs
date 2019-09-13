//! Export `Org` struct to various formats.

pub mod html;
pub mod org;

pub use html::*;
pub use org::*;

use std::io::{Error, Write};

use crate::elements::Datetime;

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
