//! A Rust library for parsing orgmode files.
//!
//! ## Example
//!
//! ```rust
//! use orgize::Parser;
//!
//! fn main() {
//!     let parser = Parser::new(
//!         r#"* Title 1
//! *Section 1*
//! ** Title 2
//! _Section 2_
//! * Title 3
//! /Section 3/
//! * Title 4
//! =Section 4="#,
//!     );
//!
//!     for event in parser {
//!         // handling the event
//!     }
//! }
//! ```
//!
//! Alternatively, you can use the built-in render.
//!
//! ```rust
//! use orgize::export::{HtmlHandler, Render};
//! use std::io::Cursor;
//!
//! fn main() {
//!     let contents = r#"* Title 1
//! *Section 1*
//! ** Title 2
//! _Section 2_
//! * Title 3
//! /Section 3/
//! * Title 4
//! =Section 4="#;
//!
//!     let cursor = Cursor::new(Vec::new());
//!     let mut render = Render::new(HtmlHandler, cursor, &contents);
//!
//!     render
//!         .render()
//!         .expect("something went wrong rendering the file");
//!
//!     println!(
//!         "{}",
//!         String::from_utf8(render.into_wirter().into_inner()).expect("invalid utf-8")
//!     );
//! }
//! ```

#[macro_use]
mod utils;

pub mod elements;
pub mod export;
pub mod headline;
mod lines;
pub mod objects;
mod parser;
pub mod tools;

pub use parser::{Event, Parser};
