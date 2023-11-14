use nom::{IResult, InputTake};

use super::{
    combinator::{blank_lines, debug_assert_lossless, line_ends_iter, node, GreenElement},
    input::Input,
    SyntaxKind,
};

fn comment_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut start = 0;
    for i in line_ends_iter(input.as_str()) {
        let line = &input.as_str()[start..i];
        let trimmed = line.trim_start();

        if trimmed == "#" || trimmed == "#\n" || trimmed == "#\r\n" || trimmed.starts_with("# ") {
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

    Ok((input, node(SyntaxKind::COMMENT, children)))
}

pub fn comment_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert_lossless(comment_node_base)(input)
}

#[test]
fn parse() {
    use crate::{
        syntax::{comment::comment_node, input::Input, SyntaxNode},
        ParseConfig,
    };

    let t = |input: &str| {
        SyntaxNode::new_root(
            comment_node(Input {
                s: input,
                c: &ParseConfig::default(),
            })
            .unwrap()
            .1
            .into_node()
            .unwrap(),
        )
    };

    insta::assert_debug_snapshot!(
        t("#"),
        @r###"
    COMMENT@0..1
      TEXT@0..1 "#"
    "###
    );

    insta::assert_debug_snapshot!(
        t("#\n  # a\n #\n\n"),
        @r###"
    COMMENT@0..12
      TEXT@0..11 "#\n  # a\n #\n"
      BLANK_LINE@11..12 "\n"
    "###
    );

    insta::assert_debug_snapshot!(
        t("#\na\n #\n\n"),
        @r###"
    COMMENT@0..2
      TEXT@0..2 "#\n"
    "###
    );
}
