use nom::{AsBytes, IResult, InputTake};

use super::{
    combinator::{blank_lines, line_ends_iter, node, GreenElement},
    input::Input,
    SyntaxKind,
};

fn fixed_width_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut start = 0;
    for i in line_ends_iter(input.as_str()) {
        let mut iter = input.as_bytes()[start..]
            .iter()
            .skip_while(|&&b| b == b' ' || b == b'\t');

        if matches!(iter.next(), Some(b':'))
            && matches!(iter.next(), None | Some(b'\n') | Some(b'\r') | Some(b' '))
        {
            start = i;
        } else {
            break;
        }
    }

    if start == 0 {
        return Err(nom::Err::Error(()));
    }

    let (input, contents) = input.take_split(start);
    let (input, post_blank) = blank_lines(input)?;

    let mut children = vec![];
    children.push(contents.text_token());
    children.extend(post_blank);

    Ok((input, node(SyntaxKind::FIXED_WIDTH, children)))
}

#[tracing::instrument(level = "debug", skip(input), fields(input = input.s))]
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
      TEXT@0..14 ": A\n:\n: B\n: C\n"
      BLANK_LINE@14..15 "\n"
      BLANK_LINE@15..19 "    "
    "###
    );
}
