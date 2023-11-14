use memchr::{memchr, memchr2};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alphanumeric1, digit1, space0},
    combinator::{cond, map, opt, recognize, verify},
    sequence::{preceded, tuple},
    AsBytes, IResult, InputLength, InputTake,
};

use super::{
    combinator::{
        at_token, blank_lines, colon2_token, debug_assert_lossless, l_bracket_token,
        line_starts_iter, node, r_bracket_token, GreenElement,
    },
    element::element_node,
    input::Input,
    object::object_nodes,
    SyntaxKind::*,
};

pub fn list_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert_lossless(list_node_base)(input)
}

fn list_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, first_indent) = space0(input)?;
    let (input, first_item) = list_item_node(first_indent, input)?;

    let mut children = vec![first_item];

    let mut input = input;
    while !input.is_empty() {
        let (input_, indent) = space0(input)?;

        if indent.input_len() != first_indent.input_len() {
            break;
        }

        if let Ok((input_, list_item)) = list_item_node(indent, input_) {
            children.push(list_item);
            input = input_;
        } else {
            break;
        }
    }

    let (input, post_blank) = blank_lines(input)?;

    children.extend(post_blank);

    Ok((input, node(LIST, children)))
}

fn list_item_node<'a>(indent: Input<'a>, input: Input<'a>) -> IResult<Input<'a>, GreenElement, ()> {
    let (input, bullet) = recognize(tuple((
        alt((
            tag("+"),
            tag("*"),
            tag("-"),
            preceded(digit1, tag(".")),
            preceded(digit1, tag(")")),
        )),
        space0,
    )))(input)?;

    // bullet must ends with whitespace,
    if !(bullet
        .s
        .bytes()
        .last()
        .map(|b| b == b' ' || b == b'\t')
        .unwrap_or(true)
        // or input should be a line end
        || input
            .s
            .bytes()
            .next()
            .map(|b| b == b'\r' || b == b'\n')
            .unwrap_or(true))
    {
        return Err(nom::Err::Error(()));
    }

    let is_ordered = bullet.s.starts_with(|c: char| c.is_ascii_digit());
    let (input, counter) = opt(list_item_counter)(input)?;
    let (input, checkbox) = opt(list_item_checkbox)(input)?;
    let (input, tag) = cond(!is_ordered, opt(list_item_tag))(input)?;
    let (input, content) = list_item_content_node(input, indent.input_len())?;

    let mut children = vec![
        indent.token(LIST_ITEM_INDENT),
        bullet.token(LIST_ITEM_BULLET),
    ];

    if let Some((counter, ws)) = counter {
        children.extend([counter, ws.ws_token()]);
    }
    if let Some((checkbox, ws)) = checkbox {
        children.extend([checkbox, ws.ws_token()]);
    }
    if let Some(Some((tag, ws))) = tag {
        children.extend([tag, ws.ws_token()]);
    }

    children.push(content);

    Ok((input, node(LIST_ITEM, children)))
}

fn list_item_counter(input: Input) -> IResult<Input, (GreenElement, Input), ()> {
    let (input, node) = map(
        tuple((l_bracket_token, at_token, alphanumeric1, r_bracket_token)),
        |(l_bracket, at, char, r_bracket)| {
            node(
                LIST_ITEM_COUNTER,
                [l_bracket, at, char.text_token(), r_bracket],
            )
        },
    )(input)?;

    let (input, ws) = space0(input)?;

    Ok((input, (node, ws)))
}

fn list_item_checkbox(input: Input) -> IResult<Input, (GreenElement, Input), ()> {
    let (input, node) = map(
        tuple((
            l_bracket_token,
            verify(take(1usize), |input: &Input| {
                input.s == " " || input.s == "X" || input.s == "-"
            }),
            r_bracket_token,
        )),
        |(l_bracket, char, r_bracket)| {
            node(
                LIST_ITEM_CHECK_BOX,
                [l_bracket, char.text_token(), r_bracket],
            )
        },
    )(input)?;

    let (input, ws) = space0(input)?;

    Ok((input, (node, ws)))
}

fn list_item_tag(input: Input) -> IResult<Input, (GreenElement, Input), ()> {
    let bytes = input.as_bytes();

    let (input, tag) = match memchr2(b'\n', b':', bytes) {
        Some(idx) if idx > 0 && bytes[idx] == b':' => input.take_split(idx),
        _ => return Err(nom::Err::Error(())),
    };
    let (input, ws) = space0(input)?;
    let (input, colon2) = colon2_token(input)?;

    let mut children = object_nodes(tag);
    children.push(colon2);

    Ok((input, (node(LIST_ITEM_TAG, children), ws)))
}

fn list_item_content_node(input: Input, indent: usize) -> IResult<Input, GreenElement, ()> {
    if memchr(b'\n', input.as_bytes()).is_none() {
        return Ok((
            input.of(""),
            node(LIST_ITEM_CONTENT, [node(PARAGRAPH, object_nodes(input))]),
        ));
    };

    let mut skip_one = true;
    let mut i = input;
    let mut children = vec![];
    let mut previous_line_is_blank = false;
    'l: loop {
        for (input, head) in line_starts_iter(i.as_str())
            // the first line in list item content will always be a paragraph
            // so we need to skip it in the first iteration
            .skip(if skip_one { 1 } else { 0 })
            .map(|idx| i.take_split(idx))
        {
            match get_line_indent(input.as_str()) {
                Some(next_indent) => {
                    previous_line_is_blank = false;

                    if next_indent <= indent {
                        if !head.is_empty() {
                            children.push(node(PARAGRAPH, object_nodes(head)));
                        }
                        return Ok((input, node(LIST_ITEM_CONTENT, children)));
                    }

                    if let Ok((input, element)) = element_node(input) {
                        if !head.is_empty() {
                            children.push(node(PARAGRAPH, object_nodes(head)));
                        }
                        children.push(element);
                        i = input;
                        skip_one = false;
                        continue 'l;
                    }
                }
                _ if previous_line_is_blank => {
                    // list item ends at two consecutive empty lines
                    if !head.is_empty() {
                        children.push(node(PARAGRAPH, object_nodes(head)));
                    }
                    let (input, post_blank) = blank_lines(input)?;

                    children.extend(post_blank);

                    return Ok((input, node(LIST_ITEM_CONTENT, children)));
                }
                _ => {
                    previous_line_is_blank = true;
                }
            }
        }

        break;
    }

    if !i.is_empty() {
        children.push(node(PARAGRAPH, object_nodes(i)));
    }

    Ok((input.of(""), node(LIST_ITEM_CONTENT, children)))
}

fn get_line_indent(input: &str) -> Option<usize> {
    input
        .bytes()
        .take_while(|b| *b != b'\n')
        .position(|b| !b.is_ascii_whitespace())
}

#[test]
fn parse() {
    use crate::{ast::List, tests::to_ast, ParseConfig};

    let to_list = to_ast::<List>(list_node);

    let list = to_list("1)");
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..2
      LIST_ITEM@0..2
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "1)"
        LIST_ITEM_CONTENT@2..2
          PARAGRAPH@2..2
    "###
    );

    let list = to_list("+ ");
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..2
      LIST_ITEM@0..2
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
        LIST_ITEM_CONTENT@2..2
          PARAGRAPH@2..2
    "###
    );

    let list = to_list("-\n");
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..2
      LIST_ITEM@0..2
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..1 "-"
        LIST_ITEM_CONTENT@1..2
          PARAGRAPH@1..2
            TEXT@1..2 "\n"
    "###
    );

    let list = to_list("+ 1");
    assert!(!list.is_ordered());
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..3
      LIST_ITEM@0..3
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
        LIST_ITEM_CONTENT@2..3
          PARAGRAPH@2..3
            TEXT@2..3 "1"
    "###
    );

    let list = to_list("+ 1\n");
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..4
      LIST_ITEM@0..4
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
        LIST_ITEM_CONTENT@2..4
          PARAGRAPH@2..4
            TEXT@2..4 "1\n"
    "###
    );

    let list = to_list("+ [@A] 1\n\n\n+ 2");
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..14
      LIST_ITEM@0..11
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
        LIST_ITEM_COUNTER@2..6
          L_BRACKET@2..3 "["
          AT@3..4 "@"
          TEXT@4..5 "A"
          R_BRACKET@5..6 "]"
        WHITESPACE@6..7 " "
        LIST_ITEM_CONTENT@7..11
          PARAGRAPH@7..10
            TEXT@7..10 "1\n\n"
          BLANK_LINE@10..11 "\n"
      LIST_ITEM@11..14
        LIST_ITEM_INDENT@11..11 ""
        LIST_ITEM_BULLET@11..13 "+ "
        LIST_ITEM_CONTENT@13..14
          PARAGRAPH@13..14
            TEXT@13..14 "2"
    "###
    );

    let list = to_list("+ *TAG* :: item1\n+ [X] item2");
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..28
      LIST_ITEM@0..17
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
        LIST_ITEM_TAG@2..10
          BOLD@2..7
            STAR@2..3 "*"
            TEXT@3..6 "TAG"
            STAR@6..7 "*"
          TEXT@7..8 " "
          COLON2@8..10 "::"
        WHITESPACE@10..10 ""
        LIST_ITEM_CONTENT@10..17
          PARAGRAPH@10..17
            TEXT@10..17 " item1\n"
      LIST_ITEM@17..28
        LIST_ITEM_INDENT@17..17 ""
        LIST_ITEM_BULLET@17..19 "+ "
        LIST_ITEM_CHECK_BOX@19..22
          L_BRACKET@19..20 "["
          TEXT@20..21 "X"
          R_BRACKET@21..22 "]"
        WHITESPACE@22..23 " "
        LIST_ITEM_CONTENT@23..28
          PARAGRAPH@23..28
            TEXT@23..28 "item2"
    "###
    );

    let list = to_list(
        r#"+ item1
  + item2"#,
    );
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..17
      LIST_ITEM@0..17
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
        LIST_ITEM_CONTENT@2..17
          PARAGRAPH@2..8
            TEXT@2..8 "item1\n"
          LIST@8..17
            LIST_ITEM@8..17
              LIST_ITEM_INDENT@8..10 "  "
              LIST_ITEM_BULLET@10..12 "+ "
              LIST_ITEM_CONTENT@12..17
                PARAGRAPH@12..17
                  TEXT@12..17 "item2"
    "###
    );

    let list = to_list("* item1\nitem2");
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..8
      LIST_ITEM@0..8
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "* "
        LIST_ITEM_CONTENT@2..8
          PARAGRAPH@2..8
            TEXT@2..8 "item1\n"
    "###
    );

    let list = to_list(
        r#"* item1

  still item 1"#,
    );
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..23
      LIST_ITEM@0..23
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "* "
        LIST_ITEM_CONTENT@2..23
          PARAGRAPH@2..23
            TEXT@2..23 "item1\n\n  still item 1"
    "###
    );

    let list = to_list(
        r#"+ item1
      + item2
    "#,
    );
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..26
      LIST_ITEM@0..26
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
        LIST_ITEM_CONTENT@2..26
          PARAGRAPH@2..8
            TEXT@2..8 "item1\n"
          LIST@8..26
            LIST_ITEM@8..26
              LIST_ITEM_INDENT@8..14 "      "
              LIST_ITEM_BULLET@14..16 "+ "
              LIST_ITEM_CONTENT@16..26
                PARAGRAPH@16..26
                  TEXT@16..26 "item2\n    "
    "###
    );

    let list = to_list(
        r#"1. item1

    - item2

3. item 3"#,
    );
    assert!(list.is_ordered());
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..32
      LIST_ITEM@0..23
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..3 "1. "
        LIST_ITEM_CONTENT@3..23
          PARAGRAPH@3..10
            TEXT@3..10 "item1\n\n"
          LIST@10..23
            LIST_ITEM@10..23
              LIST_ITEM_INDENT@10..14 "    "
              LIST_ITEM_BULLET@14..16 "- "
              LIST_ITEM_CONTENT@16..23
                PARAGRAPH@16..23
                  TEXT@16..23 "item2\n\n"
      LIST_ITEM@23..32
        LIST_ITEM_INDENT@23..23 ""
        LIST_ITEM_BULLET@23..26 "3. "
        LIST_ITEM_CONTENT@26..32
          PARAGRAPH@26..32
            TEXT@26..32 "item 3"
    "###
    );

    let list = to_list(
        r#"  + item1

  + item2"#,
    );
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..20
      LIST_ITEM@0..11
        LIST_ITEM_INDENT@0..2 "  "
        LIST_ITEM_BULLET@2..4 "+ "
        LIST_ITEM_CONTENT@4..11
          PARAGRAPH@4..11
            TEXT@4..11 "item1\n\n"
      LIST_ITEM@11..20
        LIST_ITEM_INDENT@11..13 "  "
        LIST_ITEM_BULLET@13..15 "+ "
        LIST_ITEM_CONTENT@15..20
          PARAGRAPH@15..20
            TEXT@15..20 "item2"
    "###
    );

    let list = to_list(
        r#"  1. item1
        2. item2
      3. item3"#,
    );
    assert!(list.is_ordered());
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..42
      LIST_ITEM@0..42
        LIST_ITEM_INDENT@0..2 "  "
        LIST_ITEM_BULLET@2..5 "1. "
        LIST_ITEM_CONTENT@5..42
          PARAGRAPH@5..11
            TEXT@5..11 "item1\n"
          LIST@11..28
            LIST_ITEM@11..28
              LIST_ITEM_INDENT@11..19 "        "
              LIST_ITEM_BULLET@19..22 "2. "
              LIST_ITEM_CONTENT@22..28
                PARAGRAPH@22..28
                  TEXT@22..28 "item2\n"
          LIST@28..42
            LIST_ITEM@28..42
              LIST_ITEM_INDENT@28..34 "      "
              LIST_ITEM_BULLET@34..37 "3. "
              LIST_ITEM_CONTENT@37..42
                PARAGRAPH@37..42
                  TEXT@37..42 "item3"
    "###
    );

    let list = to_list(
        r#"  1. item1
    #+begin_example
hello
#+end_example
"#,
    );
    insta::assert_debug_snapshot!(
        list.syntax,
        @r###"
    LIST@0..51
      LIST_ITEM@0..51
        LIST_ITEM_INDENT@0..2 "  "
        LIST_ITEM_BULLET@2..5 "1. "
        LIST_ITEM_CONTENT@5..51
          PARAGRAPH@5..11
            TEXT@5..11 "item1\n"
          EXAMPLE_BLOCK@11..51
            BLOCK_BEGIN@11..31
              WHITESPACE@11..15 "    "
              TEXT@15..23 "#+begin_"
              TEXT@23..30 "example"
              TEXT@30..30 ""
              NEW_LINE@30..31 "\n"
            BLOCK_CONTENT@31..37
              TEXT@31..37 "hello\n"
            BLOCK_END@37..51
              TEXT@37..43 "#+end_"
              TEXT@43..50 "example"
              NEW_LINE@50..51 "\n"
    "###
    );

    let config = &ParseConfig::default();

    assert!(list_node(("-a", config).into()).is_err());
}
