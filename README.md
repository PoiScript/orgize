# Orgize

Orgize is a Emacs Org-mode parser written by pure Rust. It behaves like a pull
parser (returning an iterator of events) but not exactly.

Besides, orgize also provides some mechanism for exporting org-mode files to
various formats, e.g. HTML.

## Usage

```toml
[dependencies]
orgize = "0.1.0"
```

```rust
// Rust 2015 only
extern crate orgize;
```

## Example

```rust
use orgize::Parser;

let parser = Parser::new(
    r"* Title 1
*Section 1*
** Title 2
_Section 2_
* Title 3
/Section 3/
* Title 4
=Section 4=",
);

for event in parser {
    // handling the event
}
```

Alternatively, you can use the built-in render.

```rust
use orgize::export::DefaultHtmlRender;
use std::io::Cursor;

let contents = r"* Title 1
*Section 1*
** Title 2
_Section 2_
* Title 3
/Section 3/
* Title 4
=Section 4=";

let cursor = Cursor::new(Vec::new());
let mut render = DefaultHtmlRender::new(cursor, &contents);

render
    .render()
    .expect("something went wrong rendering the file");
    
let result = String::from_utf8(render.into_wirter().into_inner()).expect("invalid utf-8");
```

## License

MIT
