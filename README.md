# Orgize

[![Build Status](https://travis-ci.org/PoiScript/orgize.svg?branch=master)](https://travis-ci.org/PoiScript/orgize)
[![Crates.io](https://img.shields.io/crates/v/orgize.svg)](https://crates.io/crates/orgize)
[![Document](https://docs.rs/orgize/badge.svg)](https://docs.rs/orgize)

A Rust library for parsing orgmode files.

## Parse

To parse a orgmode string, simply invoking the `Org::parse` function:

```rust
use orgize::Org;

Org::parse("* DONE Title :tag:");
```

or `Org::parse_with_config`:

``` rust
use orgize::{Org, ParseConfig};

let org = Org::parse_with_config(
    "* TASK Title 1",
    ParseConfig {
        // custom todo keywords
        todo_keywords: &["TASK"],
        ..Default::default()
    },
);
```

## Iter

`Org::iter` function will returns an iteractor of `Event`s, which is
a simple wrapper of `Element`.

```rust
use orgize::Org;

for event in Org::parse("* DONE Title :tag:").iter() {
    // handling the event
}
```

**Note**: whether an element is container or not, it will appears twice in one loop.
One as `Event::Start(element)`, one as `Event::End(element)`.

## Render html

You can call the `Org::html` function to generate html directly, which
uses the `DefaultHtmlHandler` internally:

```rust
use orgize::Org;

let mut writer = Vec::new();
Org::parse("* title\n*section*").html(&mut writer).unwrap();

assert_eq!(
    String::from_utf8(writer).unwrap(),
    "<main><h1>title</h1><section><p><b>section</b></p></section></main>"
);
```

## Render html with custom HtmlHandler

To customize html rendering, simply implementing `HtmlHandler` trait and passing
it to the `Org::html_with_handler` function.

The following code demonstrates how to add a id for every headline and return
own error type while rendering.

```rust
use std::convert::From;
use std::io::{Error as IOError, Write};
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
    let mut writer = Vec::new();
    Org::parse("* title\n*section*").html_with_handler(&mut writer, MyHtmlHandler)?;

    assert_eq!(
        String::from_utf8(writer)?,
        "<main><h1><a id=\"title\" href=\"#title\">title</a></h1>\
         <section><p><b>section</b></p></section></main>"
    );

    Ok(())
}
```

**Note**: as I mentioned above, each element will appears two times while iterating.
And handler will silently ignores all end events from non-container elements.

So if you want to change how a non-container element renders, just redefine the start
function and leave the end function untouched.

## Serde

`Org` struct have already implemented serde's `Serialize` trait. It means you can
freely serialize it into any format that serde supports such as json:

```rust
use orgize::Org;
use serde_json::{json, to_string};

let org = Org::parse("I 'm *bold*.");
println!("{}", to_string(&org).unwrap());

// {
//     "type": "document",
//     "children": [{
//         "type": "section",
//         "children": [{
//             "type": "paragraph",
//             "children":[{
//                 "type": "text",
//                 "value":"I 'm "
//             }, {
//                 "type": "bold",
//                 "children":[{
//                     "type": "text",
//                     "value": "bold"
//                 }]
//             }, {
//                 "type":"text",
//                 "value":"."
//             }]
//         }]
//     }]
// }
```

## Features

By now, orgize provides three features:

+ `serde`: adds the ability to serialize `Org` and other elements using `serde`, enabled by default.

+ `chrono`: adds the ability to convert `Datetime` into `chrono` struct, disabled by default.

## License

MIT
