use memchr::memchr2_iter;
use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    sequence::tuple,
    Err, IResult, InputTake,
};

use super::{
    combinator::{colon_token, l_bracket_token, node, r_bracket_token, GreenElement},
    input::Input,
    object::standard_object_nodes,
    SyntaxKind::*,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn fn_ref_node(input: Input) -> IResult<Input, GreenElement, ()> {
    crate::lossless_parser!(fn_ref_node_base, input)
}

fn fn_ref_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, (l_bracket, fn_, colon, label, definition, r_bracket)) = tuple((
        l_bracket_token,
        tag("fn"),
        colon_token,
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
        opt(tuple((colon_token, balanced_brackets))),
        r_bracket_token,
    ))(input)?;

    let mut children = vec![l_bracket, fn_.text_token(), colon, label.text_token()];
    if let Some((colon, definition)) = definition {
        children.push(colon);
        children.extend(standard_object_nodes(definition));
    }
    children.push(r_bracket);

    Ok((input, node(FN_REF, children)))
}

fn balanced_brackets(input: Input) -> IResult<Input, Input, ()> {
    let mut pairs = 1;
    let bytes = input.as_bytes();
    for i in memchr2_iter(b'[', b']', bytes) {
        if bytes[i] == b'[' {
            pairs += 1;
        } else if pairs != 1 {
            pairs -= 1;
        } else {
            return Ok(input.take_split(i));
        }
    }
    Err(Err::Error(()))
}

#[test]
fn parse() {
    use crate::{ast::FnRef, tests::to_ast, ParseConfig};

    let to_fn_ref = to_ast::<FnRef>(fn_ref_node);

    insta::assert_debug_snapshot!(
        to_fn_ref("[fn:1]").syntax,
        @r###"
    FN_REF@0..6
      L_BRACKET@0..1 "["
      TEXT@1..3 "fn"
      COLON@3..4 ":"
      TEXT@4..5 "1"
      R_BRACKET@5..6 "]"
    "###
    );

    insta::assert_debug_snapshot!(
        to_fn_ref("[fn:1:2]").syntax,
        @r###"
    FN_REF@0..8
      L_BRACKET@0..1 "["
      TEXT@1..3 "fn"
      COLON@3..4 ":"
      TEXT@4..5 "1"
      COLON@5..6 ":"
      TEXT@6..7 "2"
      R_BRACKET@7..8 "]"
    "###
    );

    insta::assert_debug_snapshot!(
        to_fn_ref("[fn::2]").syntax,
        @r###"
    FN_REF@0..7
      L_BRACKET@0..1 "["
      TEXT@1..3 "fn"
      COLON@3..4 ":"
      TEXT@4..4 ""
      COLON@4..5 ":"
      TEXT@5..6 "2"
      R_BRACKET@6..7 "]"
    "###
    );

    insta::assert_debug_snapshot!(
        to_fn_ref("[fn::[]]").syntax,
        @r###"
    FN_REF@0..8
      L_BRACKET@0..1 "["
      TEXT@1..3 "fn"
      COLON@3..4 ":"
      TEXT@4..4 ""
      COLON@4..5 ":"
      TEXT@5..7 "[]"
      R_BRACKET@7..8 "]"
    "###
    );

    let config = &ParseConfig::default();

    assert!(fn_ref_node(("[fn::[]", config).into()).is_err());
}
