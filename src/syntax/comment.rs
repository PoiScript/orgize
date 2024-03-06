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
    SyntaxKind,
};

fn comment_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut b = NodeBuilder::new();

    let mut iter = iterator(
        input,
        opt(tuple((
            space0,
            tag("#"),
            opt(tuple((space1, take_while(|c| c != '\r' && c != '\n')))),
            eol_or_eof,
        ))),
    );

    for (idx, option) in iter.enumerate() {
        match option {
            Some((ws, common, content, eol)) => {
                b.ws(ws);
                b.token(SyntaxKind::HASH, common);
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

    Ok((input, b.finish(SyntaxKind::COMMENT)))
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn comment_node(input: Input) -> IResult<Input, GreenElement, ()> {
    crate::lossless_parser!(comment_node_base, input)
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
      HASH@0..1 "#"
    "###
    );

    insta::assert_debug_snapshot!(
        t("#\n  # a\n #\n\n"),
        @r###"
    COMMENT@0..12
      HASH@0..1 "#"
      TEXT@1..2 "\n"
      WHITESPACE@2..4 "  "
      HASH@4..5 "#"
      WHITESPACE@5..6 " "
      TEXT@6..7 "a"
      TEXT@7..8 "\n"
      WHITESPACE@8..9 " "
      HASH@9..10 "#"
      TEXT@10..11 "\n"
      BLANK_LINE@11..12 "\n"
    "###
    );

    insta::assert_debug_snapshot!(
        t("#\na\n #\n\n"),
        @r###"
    COMMENT@0..2
      HASH@0..1 "#"
      TEXT@1..2 "\n"
    "###
    );
}
