use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{space0, space1},
    combinator::{iterator, opt},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{blank_lines, eol_or_eof, GreenElement, NodeBuilder},
    input::Input,
    keyword::affiliated_keyword_nodes,
    SyntaxKind,
};

fn fixed_width_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut b = NodeBuilder::new();

    let (input, keywords) = affiliated_keyword_nodes(input)?;
    b.children.extend(keywords);

    let mut iter = iterator(
        input,
        opt(tuple((
            space0,
            tag(":"),
            opt(tuple((space1, take_while(|c| c != '\r' && c != '\n')))),
            eol_or_eof,
        ))),
    );

    for (idx, option) in iter.enumerate() {
        match option {
            Some((ws, common, content, eol)) => {
                b.ws(ws);
                b.token(SyntaxKind::COMMA, common);
                if let Some((ws, text)) = content {
                    b.ws(ws);
                    b.text(text);
                }
                b.text(eol);
            }
            _ if idx == 0 => return Err(nom::Err::Error(())),
            _ => break,
        }
    }

    let (input, _) = iter.finish()?;

    let (input, post_blank) = blank_lines(input)?;

    b.children.extend(post_blank);

    Ok((input, b.finish(SyntaxKind::FIXED_WIDTH)))
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn fixed_width_node(input: Input) -> IResult<Input, GreenElement, ()> {
    crate::lossless_parser!(fixed_width_node_base, input)
}

#[test]
fn parse() {
    use crate::{ast::FixedWidth, tests::to_ast};

    let to_fixed_width = to_ast::<FixedWidth>(fixed_width_node);

    insta::assert_debug_snapshot!(
        to_fixed_width(
            r#": A
:
: B
: C

    "#
        ).syntax,
        @r###"
    FIXED_WIDTH@0..19
      COMMA@0..1 ":"
      WHITESPACE@1..2 " "
      TEXT@2..3 "A"
      TEXT@3..4 "\n"
      COMMA@4..5 ":"
      TEXT@5..6 "\n"
      COMMA@6..7 ":"
      WHITESPACE@7..8 " "
      TEXT@8..9 "B"
      TEXT@9..10 "\n"
      COMMA@10..11 ":"
      WHITESPACE@11..12 " "
      TEXT@12..13 "C"
      TEXT@13..14 "\n"
      BLANK_LINE@14..15 "\n"
      BLANK_LINE@15..19 "    "
    "###
    );
}
