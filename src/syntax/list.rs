use memchr::{memchr, memchr2};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alphanumeric1, digit1, space0, space1},
    combinator::{cond, map, opt, recognize, verify},
    sequence::{preceded, tuple},
    IResult, InputTake,
};

use super::{
    combinator::{
        at_token, blank_lines, colon2_token, eol_or_eof, l_bracket_token, line_starts_iter, node,
        r_bracket_token, GreenElement,
    },
    element::element_node,
    input::Input,
    keyword::affiliated_keyword_nodes,
    object::standard_object_nodes,
    paragraph::paragraph_nodes,
    SyntaxKind::*,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn list_node(input: Input) -> IResult<Input, GreenElement, ()> {
    crate::lossless_parser!(list_node_base, input)
}

fn list_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, affiliated_keywords) = affiliated_keyword_nodes(input)?;
    let (input, first_indent) = space0(input)?;
    let (input, (ends_with_empty_blank_lines, first_item)) = list_item_node(first_indent, input)?;

    let mut children = vec![];
    children.extend(affiliated_keywords);
    children.push(first_item);

    let mut input = input;
    while !ends_with_empty_blank_lines && !input.is_empty() {
        let (input_, indent) = space0(input)?;

        if indent.len() != first_indent.len() {
            break;
        }

        let Ok((input_, (ends_with_empty_blank_lines, list_item))) = list_item_node(indent, input_)
        else {
            break;
        };

        children.push(list_item);
        debug_assert!(
            input.len() > input_.len(),
            "{} > {}",
            input.len(),
            input_.len(),
        );
        input = input_;

        if ends_with_empty_blank_lines {
            break;
        }
    }

    let (input, post_blank) = blank_lines(input)?;

    children.extend(post_blank);

    Ok((input, node(LIST, children)))
}

#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "debug", skip(input, indent), fields(input = input.s))
)]
fn list_item_node<'a>(
    indent: Input<'a>,
    input: Input<'a>,
) -> IResult<Input<'a>, (bool, GreenElement), ()> {
    let (input, bullet) = recognize(tuple((
        alt((
            tag("+"),
            tag("*"),
            tag("-"),
            preceded(digit1, tag(".")),
            preceded(digit1, tag(")")),
        )),
        alt((space1, eol_or_eof)),
    )))(input)?;

    // list item cannot have an asterisk at the beginning of line
    if indent.is_empty() && bullet.s.starts_with('*') {
        return Err(nom::Err::Error(()));
    }

    if input.is_empty() {
        return Ok((
            input,
            (
                false,
                node(
                    LIST_ITEM,
                    [
                        indent.token(LIST_ITEM_INDENT),
                        bullet.token(LIST_ITEM_BULLET),
                    ],
                ),
            ),
        ));
    }

    let is_ordered = bullet.s.starts_with(|c: char| c.is_ascii_digit());
    let (input, counter) = opt(list_item_counter)(input)?;
    let (input, checkbox) = opt(list_item_checkbox)(input)?;
    let (input, tag) = cond(!is_ordered, opt(list_item_tag))(input)?;
    let (input, (ends_with_empty_blank_lines, content)) =
        list_item_content_node(input, indent.len())?;
    let (input, post_blank) = cond(!ends_with_empty_blank_lines, blank_lines)(input)?;

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
    if let Some(post_blank) = post_blank {
        children.extend(post_blank);
    }

    Ok((
        input,
        (ends_with_empty_blank_lines, node(LIST_ITEM, children)),
    ))
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
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

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
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

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
fn list_item_tag(input: Input) -> IResult<Input, (GreenElement, Input), ()> {
    let bytes = input.as_bytes();

    let (input, tag) = match memchr2(b'\n', b':', bytes) {
        Some(idx) if idx > 0 && bytes[idx] == b':' => input.take_split(idx),
        _ => return Err(nom::Err::Error(())),
    };
    let (input, ws) = space0(input)?;
    let (input, colon2) = colon2_token(input)?;

    let mut children = standard_object_nodes(tag);
    children.push(colon2);

    Ok((input, (node(LIST_ITEM_TAG, children), ws)))
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
fn list_item_content_node(input: Input, indent: usize) -> IResult<Input, (bool, GreenElement), ()> {
    if memchr(b'\n', input.as_bytes()).is_none() {
        return Ok((
            input.of(""),
            (
                false,
                node(
                    LIST_ITEM_CONTENT,
                    [node(PARAGRAPH, standard_object_nodes(input))],
                ),
            ),
        ));
    };

    let mut skip_one = true;
    let mut i = input;
    let mut children = vec![];
    let mut previous_blank_line: Option<(Input, Input)> = None;
    'l: while !i.is_empty() {
        for (input, head) in line_starts_iter(i.as_str())
            // the first line in list item content will always be a paragraph
            // so we need to skip it in the first iteration
            .skip(if skip_one { 1 } else { 0 })
            .map(|idx| i.take_split(idx))
        {
            match get_line_indent(input.as_str()) {
                Some(next_indent) => {
                    if next_indent <= indent {
                        let (input, head) = previous_blank_line.unwrap_or((input, head));
                        if !head.is_empty() {
                            children.extend(paragraph_nodes(head)?);
                        }
                        return Ok((input, (false, node(LIST_ITEM_CONTENT, children))));
                    }

                    previous_blank_line = None;

                    if let Ok((input, element)) = element_node(input) {
                        if !head.is_empty() {
                            children.extend(paragraph_nodes(head)?);
                        }
                        children.push(element);
                        debug_assert!(input.len() < i.len(), "{} < {}", input.len(), i.len());
                        i = input;
                        skip_one = false;
                        continue 'l;
                    }
                }
                _ => {
                    // list item ends at two consecutive empty lines
                    if let Some((input, head)) = previous_blank_line {
                        if !head.is_empty() {
                            children.extend(paragraph_nodes(head)?);
                        }

                        return Ok((input, (true, node(LIST_ITEM_CONTENT, children))));
                    } else {
                        previous_blank_line = Some((input, head))
                    }
                }
            }
        }
        children.extend(paragraph_nodes(i)?);
        break;
    }

    Ok((input.of(""), (false, node(LIST_ITEM_CONTENT, children))))
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

    insta::assert_debug_snapshot!(
        to_list("1)").syntax,
        @r###"
    LIST@0..2
      LIST_ITEM@0..2
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "1)"
    "###
    );

    insta::assert_debug_snapshot!(
        to_list("+ ").syntax,
        @r###"
    LIST@0..2
      LIST_ITEM@0..2
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
    "###
    );

    insta::assert_debug_snapshot!(
        to_list("-\n").syntax,
        @r###"
    LIST@0..2
      LIST_ITEM@0..2
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "-\n"
    "###
    );

    insta::assert_debug_snapshot!(
        to_list("+ 1").syntax,
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

    insta::assert_debug_snapshot!(
        to_list("+ 1\n").syntax,
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

    // list ends with two consecutive blank lines, and these blank lines
    // will be the post_blank of list node
    insta::assert_debug_snapshot!(
        to_list("+ [@A] 1\n\n\n+ 2").syntax,
        @r###"
    LIST@0..11
      LIST_ITEM@0..9
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
        LIST_ITEM_COUNTER@2..6
          L_BRACKET@2..3 "["
          AT@3..4 "@"
          TEXT@4..5 "A"
          R_BRACKET@5..6 "]"
        WHITESPACE@6..7 " "
        LIST_ITEM_CONTENT@7..9
          PARAGRAPH@7..9
            TEXT@7..9 "1\n"
      BLANK_LINE@9..10 "\n"
      BLANK_LINE@10..11 "\n"
    "###
    );

    // empty line between list item, the empty line will be
    // the post_blank of first item
    insta::assert_debug_snapshot!(
        to_list("+ *TAG* :: item1\n\n+ [X] item2").syntax,
        @r###"
    LIST@0..29
      LIST_ITEM@0..18
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
        BLANK_LINE@17..18 "\n"
      LIST_ITEM@18..29
        LIST_ITEM_INDENT@18..18 ""
        LIST_ITEM_BULLET@18..20 "+ "
        LIST_ITEM_CHECK_BOX@20..23
          L_BRACKET@20..21 "["
          TEXT@21..22 "X"
          R_BRACKET@22..23 "]"
        WHITESPACE@23..24 " "
        LIST_ITEM_CONTENT@24..29
          PARAGRAPH@24..29
            TEXT@24..29 "item2"
    "###
    );

    // nested list
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

    insta::assert_debug_snapshot!(
        to_list("+ item1\nitem2").syntax,
        @r###"
    LIST@0..8
      LIST_ITEM@0..8
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
        LIST_ITEM_CONTENT@2..8
          PARAGRAPH@2..8
            TEXT@2..8 "item1\n"
    "###
    );

    insta::assert_debug_snapshot!(
        to_list("+ item1\n\n  still item 1").syntax,
        @r###"
    LIST@0..23
      LIST_ITEM@0..23
        LIST_ITEM_INDENT@0..0 ""
        LIST_ITEM_BULLET@0..2 "+ "
        LIST_ITEM_CONTENT@2..23
          PARAGRAPH@2..9
            TEXT@2..8 "item1\n"
            BLANK_LINE@8..9 "\n"
          PARAGRAPH@9..23
            TEXT@9..23 "  still item 1"
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
                  TEXT@16..22 "item2\n"
                  BLANK_LINE@22..26 "    "
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
            TEXT@3..9 "item1\n"
            BLANK_LINE@9..10 "\n"
          LIST@10..23
            LIST_ITEM@10..23
              LIST_ITEM_INDENT@10..14 "    "
              LIST_ITEM_BULLET@14..16 "- "
              LIST_ITEM_CONTENT@16..22
                PARAGRAPH@16..22
                  TEXT@16..22 "item2\n"
              BLANK_LINE@22..23 "\n"
      LIST_ITEM@23..32
        LIST_ITEM_INDENT@23..23 ""
        LIST_ITEM_BULLET@23..26 "3. "
        LIST_ITEM_CONTENT@26..32
          PARAGRAPH@26..32
            TEXT@26..32 "item 3"
    "###
    );

    // nested list
    insta::assert_debug_snapshot!(
        to_list("  + item1\n\n  + item2").syntax,
        @r###"
    LIST@0..20
      LIST_ITEM@0..11
        LIST_ITEM_INDENT@0..2 "  "
        LIST_ITEM_BULLET@2..4 "+ "
        LIST_ITEM_CONTENT@4..10
          PARAGRAPH@4..10
            TEXT@4..10 "item1\n"
        BLANK_LINE@10..11 "\n"
      LIST_ITEM@11..20
        LIST_ITEM_INDENT@11..13 "  "
        LIST_ITEM_BULLET@13..15 "+ "
        LIST_ITEM_CONTENT@15..20
          PARAGRAPH@15..20
            TEXT@15..20 "item2"
    "###
    );

    insta::assert_debug_snapshot!(
        to_list("  1. item1\n        2. item2\n      3. item3").syntax,
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

    // Indentation of lines within other greater elements do not count
    insta::assert_debug_snapshot!(
        to_list("  1. item1\n    #+begin_example\nhello\n#+end_example\n").syntax,
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
              NEW_LINE@30..31 "\n"
            BLOCK_CONTENT@31..37
              TEXT@31..37 "hello\n"
            BLOCK_END@37..51
              TEXT@37..43 "#+end_"
              TEXT@43..50 "example"
              NEW_LINE@50..51 "\n"
    "###
    );

    to_list("- ");
    to_list("-\t");
    to_list("-\r");
    to_list("-\t\n");
    to_list("-\r\n");
    to_list("-");

    let config = &ParseConfig::default();

    assert!(list_node(("-a", config).into()).is_err());
    assert!(list_node(("*\r\n", config).into()).is_err());
    assert!(list_node(("* ", config).into()).is_err());
}
