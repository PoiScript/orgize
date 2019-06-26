//! A Rust library for parsing orgmode files.
//!
//! # Using Parser
//!
//! Orgize parser acts like a event-based parser, which means it
//! returns an `Iterator` of [`Event`] s.
//!
//! [`Event`]: enum.Event.html
//!
//! ```rust
//! use orgize::Parser;
//!
//! let parser = Parser::new(r#"* Title 1
//! *Section 1*
//! ** Title 2
//! _Section 2_
//! * Title 3
//! /Section 3/
//! * Title 4
//! =Section 4="#);
//!
//! for event in parser {
//!     // handling the event
//! }
//! ```
//!
//! # Using Render
//!
//! You can use the built-in [`HtmlRender`] to generate html string directly:
//!
//! [`HtmlRender`]: export/struct.HtmlRender.html
//!
//! ```rust
//! use orgize::export::HtmlRender;
//! use std::io::{Cursor, Result};
//!
//! fn main() -> Result<()> {
//!     let contents = r"* Title 1
//! *Section 1*
//! ** Title 2
//! _Section 2_
//! * Title 3
//! /Section 3/
//! * Title 4
//! =Section 4=";
//!
//!     let mut cursor = Cursor::new(Vec::new());
//!     let mut render = HtmlRender::default(&mut cursor, &contents);
//!
//!     render.render()?;
//!
//!     assert_eq!(
//!         String::from_utf8(cursor.into_inner()).unwrap(),
//!         "<h1>Title 1</h1><section><p><b>Section 1</b></p></section>\
//!          <h2>Title 2</h2><section><p><u>Section 2</u></p></section>\
//!          <h1>Title 3</h1><section><p><i>Section 3</i></p></section>\
//!          <h1>Title 4</h1><section><p><code>Section 4</code></p></section>"
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! # Custom HtmlHandler
//!
//! You can create your own handler by implementing [`HtmlHandler`] trait and passing
//! it to the [`HtmlRender`].
//!
//! The following example demonstrates how to add an anchor for every headline and use
//! your own error type.
//!
//! [`HtmlHandler`]: export/trait.HtmlHandler.html
//! [`HtmlRender`]: export/struct.HtmlRender.html
//!
//! ```rust
//! use orgize::{export::*, headline::Headline};
//! use slugify::slugify;
//! use std::io::{Cursor, Error as IOError, Write};
//! use std::string::FromUtf8Error;
//!
//! // custom error type
//! #[derive(Debug)]
//! enum Error {
//!     IO(IOError),
//!     Headline,
//!     Utf8(FromUtf8Error),
//! }
//!
//! // From<std::io::Error> trait is required for custom error type
//! impl From<IOError> for Error {
//!     fn from(err: IOError) -> Error {
//!         Error::IO(err)
//!     }
//! }
//!
//! struct CustomHtmlHandler;
//!
//! impl<W: Write> HtmlHandler<W, Error> for CustomHtmlHandler {
//!     fn headline_beg(&mut self, w: &mut W, hdl: Headline) -> Result<(), Error> {
//!          if hdl.level > 6 {
//!              Err(Error::Headline)
//!          } else {
//!              write!(
//!                  w,
//!                  r##"<h{}><a class="anchor" href="#{}">"##,
//!                  hdl.level,
//!                  slugify!(hdl.title),
//!              )?;
//!              self.escape(w, hdl.title)?;
//!              Ok(write!(w, "</a></h{}>", hdl.level)?)
//!          }
//!     }
//! }
//!
//! fn main() -> Result<(), Error> {
//!     let contents = r"* Title 1
//! *Section 1*
//! ** Title 2
//! _Section 2_
//! * Title 3
//! /Section 3/
//! * Title 4
//! =Section 4=";
//!
//!     let mut cursor = Cursor::new(Vec::new());
//!     let mut render = HtmlRender::new(CustomHtmlHandler, &mut cursor, &contents);
//!
//!     render.render()?;
//!
//!     assert_eq!(
//!         String::from_utf8(cursor.into_inner()).map_err(Error::Utf8)?,
//!         "<h1><a class=\"anchor\" href=\"#title-1\">Title 1</a></h1><section><p><b>Section 1</b></p></section>\
//!          <h2><a class=\"anchor\" href=\"#title-2\">Title 2</a></h2><section><p><u>Section 2</u></p></section>\
//!          <h1><a class=\"anchor\" href=\"#title-3\">Title 3</a></h1><section><p><i>Section 3</i></p></section>\
//!          <h1><a class=\"anchor\" href=\"#title-4\">Title 4</a></h1><section><p><code>Section 4</code></p></section>"
//!     );
//!
//!     Ok(())
//! }
//! ```

pub mod elements;
pub mod export;
pub mod iter;
pub mod org;
#[cfg(feature = "serde")]
mod serde;

pub use iter::{Container, Event};
pub use org::Org;
