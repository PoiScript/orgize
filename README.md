A Rust library for parsing org-mode files.

[![Crates.io](https://img.shields.io/crates/v/orgize.svg)](https://crates.io/crates/orgize)
[![Documentation](https://docs.rs/orgize/badge.svg)](https://docs.rs/orgize)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

# Parse

To parse a org-mode string, simply invoking the `Org::parse` function:

```rust
use orgize::{Org, rowan::ast::AstNode};

let org = Org::parse("* DONE Title :tag:");
assert_eq!(
    format!("{:#?}", org.document().syntax()),
    r#"DOCUMENT@0..18
  HEADLINE@0..18
    HEADLINE_STARS@0..1 "*"
    WHITESPACE@1..2 " "
    HEADLINE_KEYWORD@2..6 "DONE"
    WHITESPACE@6..7 " "
    HEADLINE_TITLE@7..13
      TEXT@7..13 "Title "
    HEADLINE_TAGS@13..18
      COLON@13..14 ":"
      TEXT@14..17 "tag"
      COLON@17..18 ":"
"#);
```

use `ParseConfig::parse` to specific a custom parse config

```rust
use orgize::{Org, ParseConfig, ast::Headline};

let config = ParseConfig {
    // custom todo keywords
    todo_keywords: (vec!["TASK".to_string()], vec![]),
    ..Default::default()
};
let org = config.parse("* TASK Title 1");
let hdl = org.first_node::<Headline>().unwrap();
assert_eq!(hdl.keyword().unwrap().text(), "TASK");
```

# Render to html

Call the `Org::to_html` function to export org element tree to html:

```rust
use orgize::Org;

assert_eq!(
    Org::parse("* title\n*section*").to_html(),
    "<main><h1>title</h1><section><p><b>section</b></p></section></main>"
);
```

Checkout `examples/html-slugify.rs` on how to customizing html export process.

# Features

- `chrono`: adds the ability to convert `Timestamp` into `chrono::NaiveDateTime`, disabled by default.

- `indexmap`: adds the ability to convert `PropertyDrawer` properties into `IndexMap`, disabled by default.
