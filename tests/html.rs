use orgize::Org;
use pretty_assertions::assert_eq;
use std::io::Result;

macro_rules! test_suite {
    ($name:ident, $content:expr, $expected:expr) => {
        #[test]
        fn $name() -> Result<()> {
            let mut writer = Vec::new();
            let org = Org::parse($content);
            org.html(&mut writer).unwrap();
            let string = String::from_utf8(writer).unwrap();
            assert_eq!(string, $expected);
            Ok(())
        }
    };
}

test_suite!(
    emphasis,
    "*bold*, /italic/,\n_underlined_, =verbatim= and ~code~",
    "<main><section><p><b>bold</b>, <i>italic</i>,\n<u>underlined</u>, \
     <code>verbatim</code> and <code>code</code></p></section></main>"
);

test_suite!(
    section_and_headline,
    r#"* Title 1
*Section 1*
** Title 2
_Section 2_
* Title 3
/Section 3/
* Title 4
=Section 4="#,
    "<main><h1>Title 1</h1>\
     <section><p><b>Section 1</b></p></section>\
     <h2>Title 2</h2>\
     <section><p><u>Section 2</u></p></section>\
     <h1>Title 3</h1>\
     <section><p><i>Section 3</i></p></section>\
     <h1>Title 4</h1>\
     <section><p><code>Section 4</code></p></section></main>"
);

test_suite!(
    list,
    r#"+ 1

+ 2

  - 3

  - 4

+ 5"#,
    "<main><section><ul>\
     <li><p>1</p></li>\
     <li><p>2</p><ul><li><p>3</p></li><li><p>4</p></li></ul></li>\
     <li><p>5</p></li>\
     </ul></section></main>"
);

test_suite!(
    snippet,
    "@@html:<del>@@delete this@@html:</del>@@",
    "<main><section><p><del>delete this</del></p></section></main>"
);
