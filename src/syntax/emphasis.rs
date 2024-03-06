use bytecount::count;
use memchr::memchr_iter;
use nom::{combinator::map, IResult, Slice};

use super::{
    combinator::{node, token, GreenElement},
    input::Input,
    object::standard_object_nodes,
    SyntaxKind::*,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn bold_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(emphasis(b'*'), |contents| {
        let mut children = vec![token(STAR, "*")];
        children.extend(standard_object_nodes(contents));
        children.push(token(STAR, "*"));
        node(BOLD, children)
    });
    crate::lossless_parser!(parser, input)
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn code_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(emphasis(b'~'), |contents| {
        node(
            CODE,
            [token(TILDE, "~"), contents.text_token(), token(TILDE, "~")],
        )
    });
    crate::lossless_parser!(parser, input)
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn strike_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(emphasis(b'+'), |contents| {
        let mut children = vec![token(PLUS, "+")];
        children.extend(standard_object_nodes(contents));
        children.push(token(PLUS, "+"));
        node(STRIKE, children)
    });
    crate::lossless_parser!(parser, input)
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn verbatim_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(emphasis(b'='), |contents| {
        node(
            VERBATIM,
            [token(EQUAL, "="), contents.text_token(), token(EQUAL, "=")],
        )
    });
    crate::lossless_parser!(parser, input)
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn underline_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(emphasis(b'_'), |contents| {
        let mut children = vec![token(UNDERSCORE, "_")];
        children.extend(standard_object_nodes(contents));
        children.push(token(UNDERSCORE, "_"));
        node(UNDERLINE, children)
    });
    crate::lossless_parser!(parser, input)
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn italic_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(emphasis(b'/'), |contents| {
        let mut children = vec![token(SLASH, "/")];
        children.extend(standard_object_nodes(contents));
        children.push(token(SLASH, "/"));
        node(ITALIC, children)
    });
    crate::lossless_parser!(parser, input)
}

fn emphasis(marker: u8) -> impl Fn(Input) -> IResult<Input, Input, ()> {
    move |input: Input| {
        let bytes = input.as_bytes();

        if bytes.len() < 3 || bytes[0] != marker || bytes[1].is_ascii_whitespace() {
            return Err(nom::Err::Error(()));
        }

        for idx in memchr_iter(marker, bytes).skip(1) {
            // contains at least one character
            if idx == 1 {
                continue;
            } else if count(&bytes[1..idx], b'\n') >= 2 {
                break;
            } else if validate_marker(idx, input) {
                return Ok((input.slice(idx + 1..), input.slice(1..idx)));
            }
        }

        Err(nom::Err::Error(()))
    }
}

fn validate_marker(pos: usize, text: Input) -> bool {
    if text.as_bytes()[pos - 1].is_ascii_whitespace() {
        false
    } else if let Some(post) = text.as_bytes().get(pos + 1) {
        [
            b' ', b'\t', b'\r', b'\n', b'-', b'.', b',', b';', b':', b'!', b'?', b'\'', b')', b'}',
            b'[',
        ]
        .contains(post)
    } else {
        true
    }
}

pub fn verify_pre(input: &str) -> bool {
    if input.is_empty() {
        return true;
    }
    matches!(
        input.as_bytes()[input.len() - 1],
        b'\t' | b' ' | b'-' | b'(' | b'{' | b'\\' | b'"' | b'\r' | b'\n'
    )
}

#[test]
fn parse() {
    use crate::{ast::Bold, tests::to_ast, ParseConfig};

    let to_bold = to_ast::<Bold>(bold_node);

    insta::assert_debug_snapshot!(
        to_bold("*bold*").syntax,
        @r###"
    BOLD@0..6
      STAR@0..1 "*"
      TEXT@1..5 "bold"
      STAR@5..6 "*"
    "###
    );

    insta::assert_debug_snapshot!(
        to_bold("*bo*ld*").syntax,
        @r###"
    BOLD@0..7
      STAR@0..1 "*"
      TEXT@1..6 "bo*ld"
      STAR@6..7 "*"
    "###
    );

    insta::assert_debug_snapshot!(
        to_bold("*bo\nld*").syntax,
        @r###"
    BOLD@0..7
      STAR@0..1 "*"
      TEXT@1..6 "bo\nld"
      STAR@6..7 "*"
    "###
    );

    let config = &ParseConfig::default();

    assert!(bold_node(("*bold*a", config).into()).is_err());
    assert!(bold_node(("*bold *", config).into()).is_err());
    assert!(bold_node(("* bold*", config).into()).is_err());
    assert!(bold_node(("*b\nol\nd*", config).into()).is_err());
    assert!(italic_node(("*bold*", config).into()).is_err());
}
