extern crate orgize;

use orgize::export::HtmlRender;
use std::io::Cursor;

macro_rules! parse_assert {
    ($content:expr, $expected:expr) => {{
        let mut cursor = Cursor::new(Vec::new());
        let mut render = HtmlRender::default(&mut cursor, $content);
        render.render().expect("render error");
        let s = String::from_utf8(cursor.into_inner()).expect("invalid utf-8");
        assert_eq!(s, $expected);
    }};
}

#[test]
fn emphasis() {
    parse_assert!(
        r#"* Title 1
*Section 1*
** Title 2
_Section 2_
* Title 3
/Section 3/
* Title 4
=Section 4="#,
        concat!(
            "<h1>Title 1</h1><section><p><b>Section 1</b></p></section>",
            "<h2>Title 2</h2><section><p><u>Section 2</u></p></section>",
            "<h1>Title 3</h1><section><p><i>Section 3</i></p></section>",
            "<h1>Title 4</h1><section><p><code>Section 4</code></p></section>"
        )
    )
}
