use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{alpha1, line_ending, space0},
    combinator::eof,
    sequence::tuple,
    IResult, InputTake,
};

use super::{
    combinator::{
        blank_lines, debug_assert_lossless, line_starts_iter, node, token, trim_line_end,
        GreenElement, NodeBuilder,
    },
    element::element_nodes,
    input::Input,
    SyntaxKind::*,
};

fn block_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, (block_begin, name)) = block_begin_node(input)?;
    let (input, pre_blank) = blank_lines(input)?;

    let (kind, is_greater_block) = match name {
        s if s.eq_ignore_ascii_case("COMMENT") => (COMMENT_BLOCK, false),
        s if s.eq_ignore_ascii_case("EXAMPLE") => (EXAMPLE_BLOCK, false),
        s if s.eq_ignore_ascii_case("EXPORT") => (EXPORT_BLOCK, false),
        s if s.eq_ignore_ascii_case("SRC") => (SOURCE_BLOCK, false),

        s if s.eq_ignore_ascii_case("CENTER") => (CENTER_BLOCK, true),
        s if s.eq_ignore_ascii_case("QUOTE") => (QUOTE_BLOCK, true),
        s if s.eq_ignore_ascii_case("VERSE") => (VERSE_BLOCK, true),
        _ => (SPECIAL_BLOCK, true),
    };

    for (input, contents) in line_starts_iter(input.as_str()).map(|i| input.take_split(i)) {
        if let Ok((input, block_end)) = block_end_node(input, name) {
            let (input, post_blank) = blank_lines(input)?;

            let mut children = vec![block_begin];
            children.extend(pre_blank);
            if is_greater_block {
                children.push(node(BLOCK_CONTENT, element_nodes(contents)?));
            } else {
                children.push(node(BLOCK_CONTENT, comma_quoted_text_nodes(contents)));
            }
            children.push(block_end);
            children.extend(post_blank);
            return Ok((input, node(kind, children)));
        }
    }

    Err(nom::Err::Error(()))
}

fn block_begin_node(input: Input) -> IResult<Input, (GreenElement, &str), ()> {
    let (input, (ws, start, name, (argument, ws_, nl))) =
        tuple((space0, tag_no_case("#+BEGIN_"), alpha1, trim_line_end))(input)?;

    let mut b = NodeBuilder::new();
    b.ws(ws);
    b.text(start);
    b.text(name);
    b.text(argument);
    b.ws(ws_);
    b.nl(nl);

    Ok((input, (b.finish(BLOCK_BEGIN), name.as_str())))
}

fn block_end_node<'a>(input: Input<'a>, name: &str) -> IResult<Input<'a>, GreenElement, ()> {
    let (input, (ws, end, name, ws_, nl)) = tuple((
        space0,
        tag_no_case("#+END_"),
        tag(name),
        space0,
        alt((line_ending, eof)),
    ))(input)?;

    let mut b = NodeBuilder::new();
    b.ws(ws);
    b.text(end);
    b.text(name);
    b.ws(ws_);
    b.nl(nl);

    Ok((input, b.finish(BLOCK_END)))
}

fn comma_quoted_text_nodes(input: Input) -> Vec<GreenElement> {
    let mut nodes = vec![];

    let s = input.as_str();

    let mut start = 0;
    for i in line_starts_iter(s) {
        // line must start with either ",*" or ",#+"
        if s.get(i..i + 2) != Some(",*") && s.get(i..i + 3) != Some(",#+") {
            continue;
        }

        let text = &s[start..i];
        if !text.is_empty() {
            nodes.push(token(TEXT, text));
        }

        nodes.push(token(COMMA, ","));
        start = i + 1;
    }

    if !s[start..].is_empty() {
        nodes.push(token(TEXT, &s[start..]));
    }

    nodes
}

pub fn block_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert_lossless(block_node_base)(input)
}

#[test]
fn test_parse() {
    use crate::ast::{ExampleBlock, SourceBlock};
    use crate::tests::to_ast;

    let to_src_block = to_ast::<SourceBlock>(block_node);
    let to_example_block = to_ast::<ExampleBlock>(block_node);

    insta::assert_debug_snapshot!(
        to_example_block(
r#"#+BEGIN_EXAMPLE
,* headline
,#+block
text
    #+END_EXAMPLE"#
        ).syntax,
        @r###"
    EXAMPLE_BLOCK@0..59
      BLOCK_BEGIN@0..16
        TEXT@0..8 "#+BEGIN_"
        TEXT@8..15 "EXAMPLE"
        TEXT@15..15 ""
        NEW_LINE@15..16 "\n"
      BLOCK_CONTENT@16..42
        COMMA@16..17 ","
        TEXT@17..28 "* headline\n"
        COMMA@28..29 ","
        TEXT@29..42 "#+block\ntext\n"
      BLOCK_END@42..59
        WHITESPACE@42..46 "    "
        TEXT@46..52 "#+END_"
        TEXT@52..59 "EXAMPLE"
    "###
    );

    insta::assert_debug_snapshot!(
        to_src_block(
r#"#+BEGIN_SRC


    #+END_SRC"#
        ).syntax,
        @r###"
    SOURCE_BLOCK@0..27
      BLOCK_BEGIN@0..12
        TEXT@0..8 "#+BEGIN_"
        TEXT@8..11 "SRC"
        TEXT@11..11 ""
        NEW_LINE@11..12 "\n"
      BLANK_LINE@12..13
        NEW_LINE@12..13 "\n"
      BLANK_LINE@13..14
        NEW_LINE@13..14 "\n"
      BLOCK_CONTENT@14..14
      BLOCK_END@14..27
        WHITESPACE@14..18 "    "
        TEXT@18..24 "#+END_"
        TEXT@24..27 "SRC"
    "###
    );

    insta::assert_debug_snapshot!(
        to_src_block(
r#"#+begin_src
    #+end_src"#
        ).syntax,
        @r###"
    SOURCE_BLOCK@0..25
      BLOCK_BEGIN@0..12
        TEXT@0..8 "#+begin_"
        TEXT@8..11 "src"
        TEXT@11..11 ""
        NEW_LINE@11..12 "\n"
      BLOCK_CONTENT@12..12
      BLOCK_END@12..25
        WHITESPACE@12..16 "    "
        TEXT@16..22 "#+end_"
        TEXT@22..25 "src"
    "###
    );

    insta::assert_debug_snapshot!(
        to_src_block(
r#"#+BEGIN_SRC javascript    
alert('Hello World!');
    #+END_SRC

    "#).syntax,
        @r###"
    SOURCE_BLOCK@0..69
      BLOCK_BEGIN@0..27
        TEXT@0..8 "#+BEGIN_"
        TEXT@8..11 "SRC"
        TEXT@11..22 " javascript"
        WHITESPACE@22..26 "    "
        NEW_LINE@26..27 "\n"
      BLOCK_CONTENT@27..50
        TEXT@27..50 "alert('Hello World!');\n"
      BLOCK_END@50..64
        WHITESPACE@50..54 "    "
        TEXT@54..60 "#+END_"
        TEXT@60..63 "SRC"
        NEW_LINE@63..64 "\n"
      BLANK_LINE@64..65
        NEW_LINE@64..65 "\n"
      BLANK_LINE@65..69
        WHITESPACE@65..69 "    "
    "###
    );

    // TODO: more testing
}
