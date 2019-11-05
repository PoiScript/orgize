use orgize::Org;
use pretty_assertions::assert_eq;

macro_rules! test_suite {
    ($name:ident, $content:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut writer = Vec::new();
            let org = Org::parse($content);
            org.write_html(&mut writer).unwrap();
            let string = String::from_utf8(writer).unwrap();
            assert_eq!(string, $expected);
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
    link,
    "Visit[[http://example.com][link1]]or[[http://example.com][link1]].",
    r#"<main><section><p>Visit<a href="http://example.com">link1</a>or<a href="http://example.com">link1</a>.</p></section></main>"#
);

test_suite!(
    section_and_headline,
    r#"
* title 1
section 1
** title 2
section 2
* title 3
section 3
* title 4
section 4
"#,
    "<main><h1>title 1</h1><section><p>section 1</p></section>\
     <h2>title 2</h2><section><p>section 2</p></section>\
     <h1>title 3</h1><section><p>section 3</p></section>\
     <h1>title 4</h1><section><p>section 4</p></section></main>"
);

test_suite!(
    list,
    r#"
+ 1

+ 2

  - 3

  - 4

+ 5
"#,
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

test_suite!(
    paragraphs,
    r#"
* title

paragraph 1

paragraph 2

paragraph 3

paragraph 4
"#,
    "<main><h1>title</h1><section>\
     <p>paragraph 1</p><p>paragraph 2</p>\
     <p>paragraph 3</p><p>paragraph 4</p>\
     </section></main>"
);

test_suite!(
    table1,
    r#"
|-----+-----+-----|
|   0 |   1 |   2 |
|-----+-----+-----|
|   4 |   5 |   6 |
"#,
    "<main><section><table>\
     <thead><tr><th>0</th><th>1</th><th>2</th></tr></thead>\
     <tbody><tr><td>4</td><td>5</td><td>6</td></tr></tbody>\
     </table></section></main>"
);

test_suite!(
    table2,
    r#"
|-----+-----+-----|
|   0 |   1 |   2 |
|   4 |   5 |   6 |
|-----+-----+-----|
"#,
    "<main><section><table>\
     <tbody><tr><td>0</td><td>1</td><td>2</td></tr>\
     <tr><td>4</td><td>5</td><td>6</td></tr></tbody>\
     </table></section></main>"
);

test_suite!(
    table3,
    r#"
|-----+-----+-----|
|-----+-----+-----|
|   0 |   1 |   2 |
|   4 |   5 |   6 |
"#,
    "<main><section><table><thead></thead>\
     <tbody><tr><td>0</td><td>1</td><td>2</td></tr>\
     <tr><td>4</td><td>5</td><td>6</td></tr></tbody>\
     </table></section></main>"
);

test_suite!(
    table4,
    r#"
|   0 |   1 |   2 |
|   4 |   5 |   6 |
|-----+-----+-----|
|-----+-----+-----|
"#,
    "<main><section><table>\
     <thead><tr><th>0</th><th>1</th><th>2</th></tr>\
     <tr><th>4</th><th>5</th><th>6</th></tr></thead>\
     <tbody></tbody></table></section></main>"
);

test_suite!(
    table5,
    r#"
|-----+-----+-----|
|-----+-----+-----|
"#,
    "<main><section><table><tbody></tbody></table></section></main>"
);
