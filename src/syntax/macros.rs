use nom::{
    bytes::complete::{take_until, take_while1},
    combinator::{map, opt, verify},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{
        l_curly3_token, l_parens_token, node, r_curly3_token, r_parens_token, GreenElement,
    },
    input::Input,
    SyntaxKind::*,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn macros_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
        tuple((
            l_curly3_token,
            verify(
                take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
                |s: &Input| s.as_bytes()[0].is_ascii_alphabetic(),
            ),
            opt(tuple((l_parens_token, take_until(")}}}"), r_parens_token))),
            r_curly3_token,
        )),
        |(l_curly3, name, argument, r_curly3)| {
            let mut children = vec![];
            children.push(l_curly3);
            children.push(name.text_token());
            if let Some((l_parens, argument, r_parens)) = argument {
                children.extend([l_parens, argument.text_token(), r_parens]);
            }
            children.push(r_curly3);
            node(MACROS, children)
        },
    );
    crate::lossless_parser!(parser, input)
}

#[test]
fn test() {
    use crate::{ast::Macros, tests::to_ast, ParseConfig};

    let to_macros = to_ast::<Macros>(macros_node);

    insta::assert_debug_snapshot!(
        to_macros("{{{title}}}").syntax,
        @r###"
    MACROS@0..11
      L_CURLY3@0..3 "{{{"
      TEXT@3..8 "title"
      R_CURLY3@8..11 "}}}"
    "###
    );

    insta::assert_debug_snapshot!(
        to_macros("{{{one_arg_macro(1)}}}").syntax,
        @r###"
    MACROS@0..22
      L_CURLY3@0..3 "{{{"
      TEXT@3..16 "one_arg_macro"
      L_PARENS@16..17 "("
      TEXT@17..18 "1"
      R_PARENS@18..19 ")"
      R_CURLY3@19..22 "}}}"
    "###
    );

    insta::assert_debug_snapshot!(
        to_macros("{{{two_arg_macro(1, 2)}}}").syntax,
        @r###"
    MACROS@0..25
      L_CURLY3@0..3 "{{{"
      TEXT@3..16 "two_arg_macro"
      L_PARENS@16..17 "("
      TEXT@17..21 "1, 2"
      R_PARENS@21..22 ")"
      R_CURLY3@22..25 "}}}"
    "###
    );

    insta::assert_debug_snapshot!(
        to_macros("{{{two_arg_macro(1\\,a, 2)}}}").syntax,
        @r###"
    MACROS@0..28
      L_CURLY3@0..3 "{{{"
      TEXT@3..16 "two_arg_macro"
      L_PARENS@16..17 "("
      TEXT@17..24 "1\\,a, 2"
      R_PARENS@24..25 ")"
      R_CURLY3@25..28 "}}}"
    "###
    );

    let config = &ParseConfig::default();

    assert!(macros_node(("{{{0uthor}}}", config).into()).is_err());
    assert!(macros_node(("{{{author}}", config).into()).is_err());
    assert!(macros_node(("{{{poem(}}}", config).into()).is_err());
    assert!(macros_node(("{{{poem)}}}", config).into()).is_err());
}
