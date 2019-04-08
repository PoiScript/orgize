//! A Rust library for parsing orgmode files.
//!
//! ## Example
//!
//! ```rust
//! use orgize::Parser;
//!
//! let parser = Parser::new(
//!     r"* Title 1
//! *Section 1*
//! ** Title 2
//! _Section 2_
//! * Title 3
//! /Section 3/
//! * Title 4
//! =Section 4=",
//!     );
//!
//! for event in parser {
//!     // handling the event
//! }
//! ```
//!
//! Alternatively, you can use the built-in render directly:
//!
//! ```rust
//! use orgize::export::HtmlRender;
//! use std::io::Cursor;
//!
//! let contents = r"* Title 1
//! *Section 1*
//! ** Title 2
//! _Section 2_
//! * Title 3
//! /Section 3/
//! * Title 4
//! =Section 4=";
//!
//! let mut cursor = Cursor::new(Vec::new());
//! let mut render = HtmlRender::default(&mut cursor, &contents);
//!
//! render
//!     .render()
//!     .expect("something went wrong rendering the file");
//!
//! let result = String::from_utf8(cursor.into_inner()).expect("invalid utf-8");
//! ```
//!
//! or `impl HtmlHandler` to create your own render. The following example
//! add an anchor to every headline.
//!
//! ```rust
//! use std::io::{Cursor, Error, Result, Write};
//!
//! use orgize::export::*;
//! use orgize::headline::Headline;
//! use slugify::slugify;
//!
//! struct CustomHtmlHandler;
//!
//! impl<W: Write> HtmlHandler<W, Error> for CustomHtmlHandler {
//!     fn headline_beg(&mut self, w: &mut W, hdl: Headline) -> Result<()> {
//!         write!(
//!             w,
//!             r##"<h{0}><a class="anchor" href="#{1}">{2}</a></h{0}>"##,
//!             if hdl.level <= 6 { hdl.level } else { 6 },
//!             slugify!(hdl.title),
//!             hdl.title,
//!         )
//!     }
//! }
//!
//! let contents = r"* Title 1
//! *Section 1*
//! ** Title 2
//! _Section 2_
//! * Title 3
//! /Section 3/
//! * Title 4
//! =Section 4=";
//!
//! let mut cursor = Cursor::new(Vec::new());
//!
//! let mut render = HtmlRender::new(CustomHtmlHandler, &mut cursor, &contents);
//!
//! render
//!     .render()
//!     .expect("something went wrong rendering the file");
//!
//! let result = String::from_utf8(cursor.into_inner()).expect("invalid utf-8");
//! ```

pub mod elements;
pub mod export;
pub mod headline;
pub mod objects;
mod parser;
pub mod tools;

pub use parser::{Event, Parser};
