use nom::IResult;

use super::{
    block::block_node,
    clock::clock_node,
    combinator::GreenElement,
    comment::comment_node,
    drawer::drawer_node,
    dyn_block::dyn_block_node,
    fixed_width::fixed_width_node,
    fn_def::fn_def_node,
    input::Input,
    keyword::{affiliated_keyword_nodes, keyword_node},
    list::list_node,
    paragraph::paragraph_node,
    rule::rule_node,
    table::{org_table_node, table_el_node},
};

/// Parses input into multiple element
///
/// input must not contains blank line in the beginning
#[tracing::instrument(level = "debug", skip(input), fields(input = input.s))]
pub fn element_nodes(input: Input) -> Result<Vec<GreenElement>, nom::Err<()>> {
    debug_assert!(!input.is_empty());

    let mut i = input;
    let mut nodes = vec![];

    while !i.is_empty() {
        let result = element_node(i);
        debug_assert!(result.is_ok(), "element_node() always returns Ok()");
        let (input, node) = result?;
        i = input;
        nodes.push(node);
    }

    debug_assert_eq!(
        input.as_str(),
        nodes.iter().fold(String::new(), |s, n| s + &n.to_string()),
        "parser must be lossless"
    );

    Ok(nodes)
}

#[tracing::instrument(level = "debug", skip(input), fields(input = input.s))]
pub fn element_node(input: Input) -> IResult<Input, GreenElement, ()> {
    // skip affiliated keyword first
    let (i, nodes) = affiliated_keyword_nodes(input)?;

    let has_affiliated_keyword = !nodes.is_empty();

    // find first non-whitespace character
    let byte = i
        .as_str()
        .trim_start_matches(|c| c == ' ' || c == '\t')
        .bytes()
        .next();

    debug_assert!(
        !(has_affiliated_keyword && matches!(byte, None | Some(b'\n') | Some(b'\r'))),
        "affiliated_keyword must not followed by blank lines: {:?}",
        input.s
    );

    let result = match byte {
        Some(b'[') => fn_def_node(input),
        Some(b'0'..=b'9') | Some(b'*') => list_node(input),
        // clock doesn't have affiliated keywords
        Some(b'C') if !has_affiliated_keyword => clock_node(input),
        Some(b'-') => rule_node(input).or_else(|_| list_node(input)),
        Some(b':') => drawer_node(input).or_else(|_| fixed_width_node(input)),
        Some(b'|') => org_table_node(input),
        Some(b'+') => table_el_node(input).or_else(|_| list_node(input)),
        Some(b'#') => block_node(input)
            .or_else(|_| keyword_node(input))
            .or_else(|_| dyn_block_node(input))
            .or_else(|_| comment_node(input)),
        _ => Err(nom::Err::Error(())),
    };

    result.or_else(|_| paragraph_node(input))
}

#[test]
fn parse() {
    use crate::syntax::{SyntaxKind, SyntaxNode};
    use crate::{syntax::combinator::node, ParseConfig};

    let t = |input: &str| {
        let config = &ParseConfig::default();
        let children = element_nodes((input, config).into()).unwrap();
        SyntaxNode::new_root(node(SyntaxKind::SECTION, children).into_node().unwrap())
    };

    insta::assert_debug_snapshot!(
        t(r#"a

b"#),
        @r###"
    SECTION@0..4
      PARAGRAPH@0..3
        TEXT@0..2 "a\n"
        BLANK_LINE@2..3 "\n"
      PARAGRAPH@3..4
        TEXT@3..4 "b"
    "###
    );

    insta::assert_debug_snapshot!(
        t("#+ATTR_HTML: :width 300px\n[[./img/a.jpg]]"),
        @r###"
    SECTION@0..41
      PARAGRAPH@0..41
        AFFILIATED_KEYWORD@0..26
          HASH_PLUS@0..2 "#+"
          TEXT@2..11 "ATTR_HTML"
          COLON@11..12 ":"
          TEXT@12..25 " :width 300px"
          NEW_LINE@25..26 "\n"
        LINK@26..41
          L_BRACKET2@26..28 "[["
          LINK_PATH@28..39 "./img/a.jpg"
          R_BRACKET2@39..41 "]]"
    "###
    );

    insta::assert_debug_snapshot!(
        t("#+ATTR_HTML: :width 300px\n[[./img/a.jpg]]"),
        @r###"
    SECTION@0..41
      PARAGRAPH@0..41
        AFFILIATED_KEYWORD@0..26
          HASH_PLUS@0..2 "#+"
          TEXT@2..11 "ATTR_HTML"
          COLON@11..12 ":"
          TEXT@12..25 " :width 300px"
          NEW_LINE@25..26 "\n"
        LINK@26..41
          L_BRACKET2@26..28 "[["
          LINK_PATH@28..39 "./img/a.jpg"
          R_BRACKET2@39..41 "]]"
    "###
    );
}

#[test]
fn affiliated_keywords() {
    use crate::syntax::{SyntaxKind, SyntaxNode};
    use crate::{syntax::combinator::node, ParseConfig};

    let t = |input: &str| {
        let config = &ParseConfig::default();
        let children = element_nodes((input, config).into()).unwrap();
        SyntaxNode::new_root(node(SyntaxKind::SECTION, children).into_node().unwrap())
    };

    // affiliated keywords + paragraph
    insta::assert_debug_snapshot!(
        t("#+ATTR_HTML: :width 300px\n[[./img/a.jpg]]"),
        @r###"
    SECTION@0..41
      PARAGRAPH@0..41
        AFFILIATED_KEYWORD@0..26
          HASH_PLUS@0..2 "#+"
          TEXT@2..11 "ATTR_HTML"
          COLON@11..12 ":"
          TEXT@12..25 " :width 300px"
          NEW_LINE@25..26 "\n"
        LINK@26..41
          L_BRACKET2@26..28 "[["
          LINK_PATH@28..39 "./img/a.jpg"
          R_BRACKET2@39..41 "]]"
    "###
    );

    // affiliated keywords + blank lines, fallback to normal keyword
    insta::assert_debug_snapshot!(
        t("#+ATTR_HTML: :width 300px\n#+CAPTION: abc\n\n[[./img/a.jpg]]"),
        @r###"
    SECTION@0..57
      KEYWORD@0..26
        HASH_PLUS@0..2 "#+"
        TEXT@2..11 "ATTR_HTML"
        COLON@11..12 ":"
        TEXT@12..25 " :width 300px"
        NEW_LINE@25..26 "\n"
      KEYWORD@26..42
        HASH_PLUS@26..28 "#+"
        TEXT@28..35 "CAPTION"
        COLON@35..36 ":"
        TEXT@36..40 " abc"
        NEW_LINE@40..41 "\n"
        BLANK_LINE@41..42 "\n"
      PARAGRAPH@42..57
        LINK@42..57
          L_BRACKET2@42..44 "[["
          LINK_PATH@44..55 "./img/a.jpg"
          R_BRACKET2@55..57 "]]"
    "###
    );

    // affiliated keywords + special element
    insta::assert_debug_snapshot!(
        t("#+CAPTION: a footnote def\n[fn:WORD] https://orgmode.org"),
        @r###"
    SECTION@0..55
      FN_DEF@0..55
        AFFILIATED_KEYWORD@0..26
          HASH_PLUS@0..2 "#+"
          TEXT@2..9 "CAPTION"
          COLON@9..10 ":"
          TEXT@10..25 " a footnote def"
          NEW_LINE@25..26 "\n"
        L_BRACKET@26..27 "["
        TEXT@27..29 "fn"
        COLON@29..30 ":"
        TEXT@30..34 "WORD"
        R_BRACKET@34..35 "]"
        TEXT@35..55 " https://orgmode.org"
    "###
    );

    // affiliated keywords + clock
    insta::assert_debug_snapshot!(
        t("#+CAPTION: a footnote def\nCLOCK: [2003-09-16 Tue 09:39]"),
        @r###"
    SECTION@0..55
      PARAGRAPH@0..55
        AFFILIATED_KEYWORD@0..26
          HASH_PLUS@0..2 "#+"
          TEXT@2..9 "CAPTION"
          COLON@9..10 ":"
          TEXT@10..25 " a footnote def"
          NEW_LINE@25..26 "\n"
        TEXT@26..33 "CLOCK: "
        TIMESTAMP_INACTIVE@33..55
          L_BRACKET@33..34 "["
          TIMESTAMP_YEAR@34..38 "2003"
          MINUS@38..39 "-"
          TIMESTAMP_MONTH@39..41 "09"
          MINUS@41..42 "-"
          TIMESTAMP_DAY@42..44 "16"
          WHITESPACE@44..45 " "
          TIMESTAMP_DAYNAME@45..48 "Tue"
          WHITESPACE@48..49 " "
          TIMESTAMP_HOUR@49..51 "09"
          COLON@51..52 ":"
          TIMESTAMP_MINUTE@52..54 "39"
          R_BRACKET@54..55 "]"
    "###
    );

    // affiliated keywords + eof
    insta::assert_debug_snapshot!(
        t("#+CAPTION: Longer caption."),
        @r###"
    SECTION@0..26
      KEYWORD@0..26
        HASH_PLUS@0..2 "#+"
        TEXT@2..9 "CAPTION"
        COLON@9..10 ":"
        TEXT@10..26 " Longer caption."
    "###
    );
}
