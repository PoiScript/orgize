use nom::{IResult, InputTake};

use super::{
    combinator::{blank_lines, line_ends_iter, node, GreenElement},
    input::Input,
    keyword::affiliated_keyword_nodes,
    object::object_nodes,
    SyntaxKind,
};

pub fn paragraph_node(input: Input) -> IResult<Input, GreenElement, ()> {
    crate::lossless_parser!(paragraph_node_base, input)
}

fn paragraph_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert!(!input.is_empty());

    let (input, keywords) = affiliated_keyword_nodes(input)?;

    let mut start = 0;
    for idx in line_ends_iter(input.as_str()) {
        // stops at blank line
        if input.s[start..idx].bytes().all(|c| c.is_ascii_whitespace()) {
            break;
        }

        start = idx;
    }

    let (input, contents) = input.take_split(start);
    let (input, post_blank) = blank_lines(input)?;

    let mut children = vec![];
    children.extend(keywords);
    children.extend(object_nodes(contents));
    children.extend(post_blank);

    Ok((input, node(SyntaxKind::PARAGRAPH, children)))
}

#[test]
fn parse() {
    use crate::{ast::Paragraph, tests::to_ast};

    let to_paragraph = to_ast::<Paragraph>(paragraph_node);

    insta::assert_debug_snapshot!(
        to_paragraph(r#"a"#).syntax,
        @r###"
    PARAGRAPH@0..1
      TEXT@0..1 "a"
    "###
    );

    insta::assert_debug_snapshot!(
        to_paragraph(r#"a
    "#).syntax,
        @r###"
    PARAGRAPH@0..6
      TEXT@0..2 "a\n"
      BLANK_LINE@2..6 "    "
    "###
    );

    insta::assert_debug_snapshot!(
        to_paragraph(r#"a
b
c
"#).syntax,
        @r###"
    PARAGRAPH@0..6
      TEXT@0..6 "a\nb\nc\n"
    "###
    );

    insta::assert_debug_snapshot!(
        to_paragraph(r#"a

c
"#).syntax,
        @r###"
    PARAGRAPH@0..3
      TEXT@0..2 "a\n"
      BLANK_LINE@2..3 "\n"
    "###
    );
}
