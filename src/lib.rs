//! A Rust library for parsing orgmode files.
//!
//! [Live demo](https://orgize.herokuapp.com/)
//!
//! # Parse
//!
//! To parse a orgmode string, simply invoking the [`Org::parse`] function:
//!
//! [`Org::parse`]: struct.Org.html#method.parse
//!
//! ```rust
//! use orgize::Org;
//!
//! Org::parse("* DONE Title :tag:");
//! ```
//!
//! or [`Org::parse_custom`]:
//!
//! [`Org::parse_custom`]: struct.Org.html#method.parse_custom
//!
//! ```rust
//! use orgize::{Org, ParseConfig};
//!
//! Org::parse_custom(
//!     "* TASK Title 1",
//!     &ParseConfig {
//!         // custom todo keywords
//!         todo_keywords: (vec!["TASK".to_string()], vec![]),
//!         ..Default::default()
//!     },
//! );
//! ```
//!
//! # Iter
//!
//! [`Org::iter`] function will returns an iteractor of [`Event`]s, which is
//! a simple wrapper of [`Element`].
//!
//! [`Org::iter`]: struct.Org.html#method.iter
//! [`Event`]: enum.Event.html
//! [`Element`]: elements/enum.Element.html
//!
//! ```rust
//! use orgize::Org;
//!
//! for event in Org::parse("* DONE Title :tag:").iter() {
//!     // handling the event
//! }
//! ```
//!
//! **Note**: whether an element is container or not, it will appears twice in one loop.
//! One as [`Event::Start(element)`], one as [`Event::End(element)`].
//!
//! [`Event::Start(element)`]: enum.Event.html#variant.Start
//! [`Event::End(element)`]: enum.Event.html#variant.End
//!
//! # Render html
//!
//! You can call the [`Org::write_html`] function to generate html directly, which
//! uses the [`DefaultHtmlHandler`] internally:
//!
//! [`Org::write_html`]: struct.Org.html#method.write_html
//! [`DefaultHtmlHandler`]: export/struct.DefaultHtmlHandler.html
//!
//! ```rust
//! use orgize::Org;
//!
//! let mut writer = Vec::new();
//! Org::parse("* title\n*section*").write_html(&mut writer).unwrap();
//!
//! assert_eq!(
//!     String::from_utf8(writer).unwrap(),
//!     "<main><h1>title</h1><section><p><b>section</b></p></section></main>"
//! );
//! ```
//!
//! # Render html with custom `HtmlHandler`
//!
//! To customize html rendering, simply implementing [`HtmlHandler`] trait and passing
//! it to the [`Org::write_html_custom`] function.
//!
//! [`HtmlHandler`]: export/trait.HtmlHandler.html
//! [`Org::write_html_custom`]: struct.Org.html#method.write_html_custom
//!
//! The following code demonstrates how to add a id for every headline and return
//! own error type while rendering.
//!
//! ```rust
//! use std::convert::From;
//! use std::io::{Error as IOError, Write};
//! use std::string::FromUtf8Error;
//!
//! use orgize::export::{DefaultHtmlHandler, HtmlHandler};
//! use orgize::{Element, Org};
//! use slugify::slugify;
//!
//! #[derive(Debug)]
//! enum MyError {
//!     IO(IOError),
//!     Heading,
//!     Utf8(FromUtf8Error),
//! }
//!
//! // From<std::io::Error> trait is required for custom error type
//! impl From<IOError> for MyError {
//!     fn from(err: IOError) -> Self {
//!         MyError::IO(err)
//!     }
//! }
//!
//! impl From<FromUtf8Error> for MyError {
//!     fn from(err: FromUtf8Error) -> Self {
//!         MyError::Utf8(err)
//!     }
//! }
//!
//! #[derive(Default)]
//! struct MyHtmlHandler(DefaultHtmlHandler);
//!
//! impl HtmlHandler<MyError> for MyHtmlHandler {
//!     fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), MyError> {
//!         if let Element::Title(title) = element {
//!             if title.level > 6 {
//!                 return Err(MyError::Heading);
//!             } else {
//!                 write!(
//!                     w,
//!                     "<h{0}><a id=\"{1}\" href=\"#{1}\">",
//!                     title.level,
//!                     slugify!(&title.raw),
//!                 )?;
//!             }
//!         } else {
//!             // fallthrough to default handler
//!             self.0.start(w, element)?;
//!         }
//!         Ok(())
//!     }
//!
//!     fn end<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), MyError> {
//!         if let Element::Title(title) = element {
//!             write!(w, "</a></h{}>", title.level)?;
//!         } else {
//!             self.0.end(w, element)?;
//!         }
//!         Ok(())
//!     }
//! }
//!
//! fn main() -> Result<(), MyError> {
//!     let mut writer = Vec::new();
//!     let mut handler = MyHtmlHandler::default();
//!     Org::parse("* title\n*section*").write_html_custom(&mut writer, &mut handler)?;
//!
//!     assert_eq!(
//!         String::from_utf8(writer)?,
//!         "<main><h1><a id=\"title\" href=\"#title\">title</a></h1>\
//!          <section><p><b>section</b></p></section></main>"
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! **Note**: as I mentioned above, each element will appears two times while iterating.
//! And handler will silently ignores all end events from non-container elements.
//!
//! So if you want to change how a non-container element renders, just redefine the `start`
//! function and leave the `end` function unchanged.
//!
//! # Serde
//!
//! `Org` struct have already implemented serde's `Serialize` trait. It means you can
//! serialize it into any format supported by serde, such as json:
//!
//! ```rust
//! use orgize::Org;
//! use serde_json::{json, to_string};
//!
//! let org = Org::parse("I 'm *bold*.");
//! println!("{}", to_string(&org).unwrap());
//!
//! // {
//! //     "type": "document",
//! //     "children": [{
//! //         "type": "section",
//! //         "children": [{
//! //             "type": "paragraph",
//! //             "children":[{
//! //                 "type": "text",
//! //                 "value":"I 'm "
//! //             }, {
//! //                 "type": "bold",
//! //                 "children":[{
//! //                     "type": "text",
//! //                     "value": "bold"
//! //                 }]
//! //             }, {
//! //                 "type":"text",
//! //                 "value":"."
//! //             }]
//! //         }]
//! //     }]
//! // }
//! ```
//!
//! # Features
//!
//! By now, orgize provides three features:
//!
//! + `ser`: adds the ability to serialize `Org` and other elements using `serde`, enabled by default.
//!
//! + `chrono`: adds the ability to convert `Datetime` into `chrono` structs, disabled by default.
//!
//! + `syntect`: provides [`SyntectHtmlHandler`] for highlighting code block, disabled by default.
//!
//! [`SyntectHtmlHandler`]: export/struct.SyntectHtmlHandler.html
//!
//! # License
//!
//! MIT

mod config;
pub mod elements;
pub mod export;
mod headline;
mod org;
mod parsers;
mod validate;

// Re-export of the indextree crate.
pub use indextree;
#[cfg(feature = "syntect")]
pub use syntect;

pub use config::ParseConfig;
pub use elements::Element;
pub use headline::{Document, Headline};
pub use org::{Event, Org};
pub use validate::ValidationError;
