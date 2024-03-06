use nom::{
    bytes::complete::{tag, take_till},
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{
        l_bracket_token, l_parens_token, node, r_bracket_token, r_parens_token, GreenElement,
    },
    input::Input,
    SyntaxKind,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn inline_call_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
        tuple((
            tag("call_"),
            take_till(|c| c == '[' || c == '\n' || c == '(' || c == ')'),
            opt(tuple((
                l_bracket_token,
                take_till(|c| c == ']' || c == '\n'),
                r_bracket_token,
            ))),
            l_parens_token,
            take_till(|c| c == ')' || c == '\n'),
            r_parens_token,
            opt(tuple((
                l_bracket_token,
                take_till(|c| c == ']' || c == '\n'),
                r_bracket_token,
            ))),
        )),
        |(call, name, inside_header, l_paren, arguments, r_paren, end_header)| {
            let mut children = vec![call.text_token()];
            children.push(name.text_token());
            if let Some((l_bracket, header, r_bracket)) = inside_header {
                children.push(l_bracket);
                children.push(header.text_token());
                children.push(r_bracket);
            }
            children.push(l_paren);
            children.push(arguments.text_token());
            children.push(r_paren);
            if let Some((l_bracket, header, r_bracket)) = end_header {
                children.push(l_bracket);
                children.push(header.text_token());
                children.push(r_bracket);
            }
            node(SyntaxKind::INLINE_CALL, children)
        },
    );
    crate::lossless_parser!(parser, input)
}

#[test]
fn parse() {
    use crate::{ast::InlineCall, tests::to_ast};

    let to_inline_call = to_ast::<InlineCall>(inline_call_node);

    let call = to_inline_call("call_square(4)");
    insta::assert_debug_snapshot!(
        call.syntax,
        @r###"
    INLINE_CALL@0..14
      TEXT@0..5 "call_"
      TEXT@5..11 "square"
      L_PARENS@11..12 "("
      TEXT@12..13 "4"
      R_PARENS@13..14 ")"
    "###
    );

    let call = to_inline_call("call_square[:results output](4)");
    insta::assert_debug_snapshot!(
        call.syntax,
        @r###"
    INLINE_CALL@0..31
      TEXT@0..5 "call_"
      TEXT@5..11 "square"
      L_BRACKET@11..12 "["
      TEXT@12..27 ":results output"
      R_BRACKET@27..28 "]"
      L_PARENS@28..29 "("
      TEXT@29..30 "4"
      R_PARENS@30..31 ")"
    "###
    );

    let call = to_inline_call("call_square(4)[:results html]");
    insta::assert_debug_snapshot!(
        call.syntax,
        @r###"
    INLINE_CALL@0..29
      TEXT@0..5 "call_"
      TEXT@5..11 "square"
      L_PARENS@11..12 "("
      TEXT@12..13 "4"
      R_PARENS@13..14 ")"
      L_BRACKET@14..15 "["
      TEXT@15..28 ":results html"
      R_BRACKET@28..29 "]"
    "###
    );

    let call = to_inline_call("call_square[:results output](4)[:results html]");
    insta::assert_debug_snapshot!(
        call.syntax,
        @r###"
    INLINE_CALL@0..46
      TEXT@0..5 "call_"
      TEXT@5..11 "square"
      L_BRACKET@11..12 "["
      TEXT@12..27 ":results output"
      R_BRACKET@27..28 "]"
      L_PARENS@28..29 "("
      TEXT@29..30 "4"
      R_PARENS@30..31 ")"
      L_BRACKET@31..32 "["
      TEXT@32..45 ":results html"
      R_BRACKET@45..46 "]"
    "###
    );
}
