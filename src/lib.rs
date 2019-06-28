//! A Rust library for parsing orgmode files.
//!
//! # Parse
//!
//! To parse a orgmode string, simply invoking the [`Org::parse`] function:
//!
//! [`Org::parse`]: org/struct.Org.html#method.parse
//!
//! ```rust
//! use orgize::Org;
//!
//! let org = Org::parse(r#"* Title 1
//! *Section 1*
//! ** Title 2
//! _Section 2_
//! * Title 3
//! /Section 3/
//! * Title 4
//! =Section 4="#);
//! ```
//!
//! # Iter
//!
//! [`Org::iter`] function will return a iteractor of [`Event`]s, which is
//! a simple wrapper of [`Element`].
//!
//! [`Org::iter`]: org/struct.Org.html#method.iter
//! [`Event`]: iter/enum.Event.html
//! [`Element`]: elements/enum.Element.html
//!
//! ```rust
//! # use orgize::Org;
//! #
//! # let org = Org::parse(r#"* Title 1
//! # *Section 1*
//! # ** Title 2
//! # _Section 2_
//! # * Title 3
//! # /Section 3/
//! # * Title 4
//! # =Section 4="#);
//! #
//! for event in org.iter() {
//!     // handling the event
//! }
//! ```
//!
//! **Note**: whether an element is container or not, it will appears two times in a loop.
//! One as [`Event::Start(element)`], one as [`Event::End(element)`].
//!
//! [`Event::Start(element)`]: iter/enum.Event.html#variant.Start
//! [`Event::End(element)`]: iter/enum.Event.html#variant.End
//!
//! # Render html
//!
//! You can call the [`Org::html_default`] function to generate html directly, which
//! uses the [`DefaultHtmlHandler`] internally:
//!
//! [`Org::html_default`]: org/struct.Org.html#method.html_default
//! [`DefaultHtmlHandler`]: export/html/struct.DefaultHtmlHandler.html
//!
//! ```rust
//! # use orgize::Org;
//! #
//! # let org = Org::parse(r#"* Title 1
//! # *Section 1*
//! # ** Title 2
//! # _Section 2_
//! # * Title 3
//! # /Section 3/
//! # * Title 4
//! # =Section 4="#);
//! #
//! let mut writer = Vec::new();
//! org.html_default(&mut writer).unwrap();
//!
//! assert_eq!(
//!     String::from_utf8(writer).unwrap(),
//!     "<main><h1>Title 1</h1><section><p><b>Section 1</b></p></section>\
//!     <h2>Title 2</h2><section><p><u>Section 2</u></p></section>\
//!     <h1>Title 3</h1><section><p><i>Section 3</i></p></section>\
//!     <h1>Title 4</h1><section><p><code>Section 4</code></p></section></main>"
//! );
//! ```
//!
//! # Render html with custom HtmlHandler
//!
//! To customize html rending, simply implementing [`HtmlHandler`] trait and passing
//! it to the [`Org::html`] function.
//!
//! [`HtmlHandler`]: export/html/trait.HtmlHandler.html
//! [`Org::html`]: org/struct.Org.html#method.html
//!
//! The following code demonstrates how to add a id for every headline and return
//! own error type while rendering.
//!
//! ```rust
//! # use std::convert::From;
//! # use std::io::{Error as IOError, Write};
//! # use std::string::FromUtf8Error;
//! #
//! # use orgize::export::{html::Escape, DefaultHtmlHandler, HtmlHandler};
//! # use orgize::{Element, Org};
//! # use slugify::slugify;
//! #
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
//! struct MyHtmlHandler;
//!
//! impl HtmlHandler<MyError> for MyHtmlHandler {
//!     fn start<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), MyError> {
//!         let mut default_handler = DefaultHtmlHandler;
//!         match element {
//!             Element::Headline { headline, .. } => {
//!                 if headline.level > 6 {
//!                     return Err(MyError::Heading);
//!                 } else {
//!                     let slugify = slugify!(headline.title);
//!                     write!(
//!                         w,
//!                         "<h{0}><a id=\"{1}\" href=\"#{1}\">{2}</a></h{0}>",
//!                         headline.level,
//!                         slugify,
//!                         Escape(headline.title),
//!                     )?;
//!                 }
//!             }
//!             // fallthrough to default handler
//!             _ => default_handler.start(w, element)?,
//!         }
//!         Ok(())
//!     }
//! }
//!
//! fn main() -> Result<(), MyError> {
//!     let contents = r"* Title 1
//! *Section 1*
//! ** Title 2
//! _Section 2_
//! * Title 3
//! /Section 3/
//! * Title 4
//! =Section 4=";
//!
//!     let mut writer = Vec::new();
//!     Org::parse(&contents).html(&mut writer, MyHtmlHandler)?;
//!     assert_eq!(
//!         String::from_utf8(writer)?,
//!         "<main><h1><a id=\"title-1\" href=\"#title-1\">Title 1</a></h1><section><p><b>Section 1</b></p></section>\
//!          <h2><a id=\"title-2\" href=\"#title-2\">Title 2</a></h2><section><p><u>Section 2</u></p></section>\
//!          <h1><a id=\"title-3\" href=\"#title-3\">Title 3</a></h1><section><p><i>Section 3</i></p></section>\
//!          <h1><a id=\"title-4\" href=\"#title-4\">Title 4</a></h1><section><p><code>Section 4</code></p></section></main>"
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! **Note**: as I mentioned above, each element will appears two times while iterating.
//! And handler will silently ignores all end events from non-container elements.
//!
//! So if you want to change how a non-container element renders, just redefine the start
//! function and leave the end function untouched.
//!
//! # Serde
//!
//! `Org` struct have already implemented serde's `Serialize` trait. It means you can
//! freely serialize it into any format that serde supports such as json:
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
//! + `serde`: adds the ability to serialize `Org` and other elements using `serde`, enabled by default.
//!
//! + `extra-serde-info`: includes the position information while serializing, disabled by default.
//!
//! + `chrono`: adds the ability to convert `Datetime` into `chrono` struct, disabled by default.
//!
//! # License
//!
//! MIT

pub mod elements;
pub mod export;
pub mod iter;
pub mod org;
#[cfg(feature = "serde")]
mod serde;

pub use elements::Element;
pub use iter::{Event, Iter};
pub use org::Org;
