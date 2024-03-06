use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit0,
    combinator::map,
    sequence::{pair, separated_pair, tuple},
    IResult,
};

use super::{
    combinator::{l_bracket_token, node, r_bracket_token, token, GreenElement},
    input::Input,
    SyntaxKind::*,
};

#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn cookie_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
        tuple((
            l_bracket_token,
            alt((
                separated_pair(digit0, tag("/"), digit0),
                pair(digit0, tag("%")),
            )),
            r_bracket_token,
        )),
        |(l_bracket, value, r_bracket)| {
            let mut children = vec![l_bracket];

            children.push(token(TEXT, value.0.as_str()));
            match value.1.as_str() {
                "%" => {
                    children.push(token(PERCENT, value.1.as_str()));
                }
                _ => {
                    children.push(token(SLASH, "/"));
                    children.push(token(TEXT, value.1.as_str()));
                }
            }
            children.push(r_bracket);

            node(COOKIE, children)
        },
    );
    crate::lossless_parser!(parser, input)
}

#[test]
fn parse() {
    use crate::ast::Cookie;
    use crate::tests::to_ast;
    use crate::ParseConfig;

    let to_cookie = to_ast::<Cookie>(cookie_node);

    insta::assert_debug_snapshot!(
      to_cookie("[1/10]").syntax,
       @r###"
    COOKIE@0..6
      L_BRACKET@0..1 "["
      TEXT@1..2 "1"
      SLASH@2..3 "/"
      TEXT@3..5 "10"
      R_BRACKET@5..6 "]"
    "###
    );

    insta::assert_debug_snapshot!(
       to_cookie("[1/1000]").syntax,
       @r###"
    COOKIE@0..8
      L_BRACKET@0..1 "["
      TEXT@1..2 "1"
      SLASH@2..3 "/"
      TEXT@3..7 "1000"
      R_BRACKET@7..8 "]"
    "###
    );

    insta::assert_debug_snapshot!(
       to_cookie("[10%]").syntax,
       @r###"
    COOKIE@0..5
      L_BRACKET@0..1 "["
      TEXT@1..3 "10"
      PERCENT@3..4 "%"
      R_BRACKET@4..5 "]"
    "###
    );

    insta::assert_debug_snapshot!(
       to_cookie("[%]").syntax,
       @r###"
    COOKIE@0..3
      L_BRACKET@0..1 "["
      TEXT@1..1 ""
      PERCENT@1..2 "%"
      R_BRACKET@2..3 "]"
    "###
    );

    insta::assert_debug_snapshot!(
       to_cookie("[/]").syntax,
       @r###"
    COOKIE@0..3
      L_BRACKET@0..1 "["
      TEXT@1..1 ""
      SLASH@1..2 "/"
      TEXT@2..2 ""
      R_BRACKET@2..3 "]"
    "###
    );

    insta::assert_debug_snapshot!(
       to_cookie("[100/]").syntax,
       @r###"
    COOKIE@0..6
      L_BRACKET@0..1 "["
      TEXT@1..4 "100"
      SLASH@4..5 "/"
      TEXT@5..5 ""
      R_BRACKET@5..6 "]"
    "###
    );

    insta::assert_debug_snapshot!(
       to_cookie("[/100]").syntax,
       @r###"
    COOKIE@0..6
      L_BRACKET@0..1 "["
      TEXT@1..1 ""
      SLASH@1..2 "/"
      TEXT@2..5 "100"
      R_BRACKET@5..6 "]"
    "###
    );

    let config = &ParseConfig::default();

    assert!(cookie_node(("[10% ]", config).into()).is_err());
    assert!(cookie_node(("[1//100]", config).into()).is_err());
    assert!(cookie_node(("[1\\100]", config).into()).is_err());
    assert!(cookie_node(("[10%%]", config).into()).is_err());
}
