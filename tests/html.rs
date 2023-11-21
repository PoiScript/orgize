use orgize::Org;

#[test]
fn emphasis() {
    insta::assert_snapshot!(
        Org::parse("*bold*, /italic/,\n_underlined_, =verbatim= and ~code~").to_html(),
        @r###"
    <main><section><p><b>bold</b>, <i>italic</i>,
    <u>underlined</u>, <code>verbatim</code> and <code>code</code></p></section></main>
    "###
    );
}

#[test]
fn link() {
    insta::assert_snapshot!(
        Org::parse("Visit[[http://example.com][link1]]or[[http://example.com][link1]].").to_html(),
        @r###"<main><section><p>Visit<a href="http://example.com">link1</a>or<a href="http://example.com">link1</a>.</p></section></main>"###
    );
}

#[test]
fn section_and_headline() {
    insta::assert_snapshot!(
        Org::parse(r#"
* title 1
section 1
** title 2
section 2
* title 3
section 3
* title 4
section 4
"#).to_html(),
        @r###"
    <main><h1>title 1</h1><section><p>section 1
    </p></section><h2>title 2</h2><section><p>section 2
    </p></section><h1>title 3</h1><section><p>section 3
    </p></section><h1>title 4</h1><section><p>section 4
    </p></section></main>
    "###
    );
}

#[test]
fn list() {
    insta::assert_snapshot!(
        Org::parse(r#"
+ 1

+ 2

  - 3

  - 4

+ 5
"#).to_html(),
        @r###"
    <main><section><ul><li><p>1
    </p></li><li><p>2
    </p><ul><li><p>3
    </p></li><li><p>4
    </p></li></ul></li><li><p>5
    </p></li></ul></section></main>
    "###
    );
}

#[test]
fn snippet() {
    insta::assert_snapshot!(
        Org::parse("@@html:<del>@@delete this@@html:</del>@@").to_html(),
        @"<main><section><p><del>delete this</del></p></section></main>"
    );
}

#[test]
fn paragraphs() {
    insta::assert_snapshot!(
        Org::parse(r#"
* title

paragraph 1

paragraph 2

paragraph 3

paragraph 4
"#).to_html(),
        @r###"
    <main><h1>title</h1><section><p></p><p>paragraph 1
    </p><p>paragraph 2
    </p><p>paragraph 3
    </p><p>paragraph 4
    </p></section></main>
    "###
    );
}

#[test]
fn table() {
    // don't has table header
    insta::assert_snapshot!(
        Org::parse(r#"
|-----+-----+-----|
|   0 |   1 |   2 |
|   4 |   5 |   6 |
|-----+-----+-----|
"#).to_html(),
        @"<main><section><table><tbody><tr><td>0</td><td>1</td><td>2</td></tr><tr><td>4</td><td>5</td><td>6</td></tr></tbody></table></section></main>"
    );

    // has table header
    insta::assert_snapshot!(
        Org::parse(r#"
|   0 |   1 |   2 |
|-----+-----+-----|
|   4 |   5 |   6 |
|-----+-----+-----|
"#).to_html(),
        @"<main><section><table><thead><tr><td>0</td><td>1</td><td>2</td></tr></thead><tbody><tr><td>4</td><td>5</td><td>6</td></tr></tbody></table></section></main>"
    );

    // has two table body
    insta::assert_snapshot!(
        Org::parse(r#"
|   0 |   1 |   2 |
|-----+-----+-----|
|   4 |   5 |   6 |
|-----+-----+-----|
|   7 |   8 |   9 |
"#).to_html(),
        @"<main><section><table><thead><tr><td>0</td><td>1</td><td>2</td></tr></thead><tbody><tr><td>4</td><td>5</td><td>6</td></tr></tbody><tbody><tr><td>7</td><td>8</td><td>9</td></tr></tbody></table></section></main>"
    );

    // multiple row rule
    insta::assert_snapshot!(
        Org::parse(r#"
|   0 |   1 |   2 |
|-----+-----+-----|
|-----+-----+-----|
|   4 |   5 |   6 |
"#).to_html(),
        @"<main><section><table><thead><tr><td>0</td><td>1</td><td>2</td></tr></thead><tbody><tr><td>4</td><td>5</td><td>6</td></tr></tbody></table></section></main>"
    );

    // empty
    insta::assert_snapshot!(
        Org::parse(r#"
|-----+-----+-----|
|-----+-----+-----|
"#).to_html(),
        @"<main><section><table></table></section></main>"
    );

    insta::assert_snapshot!(
        Org::parse(r#"
|
|-
|
|-
|
"#).to_html(),
        @"<main><section><table><thead><tr></tr></thead><tbody><tr></tr></tbody><tbody><tr></tr></tbody></table></section></main>"
    );
}

#[test]
fn line_break() {
    insta::assert_debug_snapshot!(
        Org::parse("aa\\\\\nbb").to_html(),
        @r###""<main><section><p>aa<br/>bb</p></section></main>""###
    );
}
