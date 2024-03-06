use nom::{
    bytes::complete::{take_until, take_while1},
    combinator::map,
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{at2_token, colon_token, node, GreenElement},
    input::Input,
    SyntaxKind::*,
};

pub fn snippet_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
        tuple((
            at2_token,
            take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-'),
            colon_token,
            take_until("@@"),
            at2_token,
        )),
        |(at2, name, colon, value, at2_)| {
            node(
                SNIPPET,
                [at2, name.text_token(), colon, value.text_token(), at2_],
            )
        },
    );
    crate::lossless_parser!(parser, input)
}

#[test]
fn parse() {
    use crate::{ast::Snippet, tests::to_ast, ParseConfig};

    let to_snippet = to_ast::<Snippet>(snippet_node);

    insta::assert_debug_snapshot!(
        to_snippet("@@html:<b>@@").syntax,
        @r###"
    SNIPPET@0..12
      AT2@0..2 "@@"
      TEXT@2..6 "html"
      COLON@6..7 ":"
      TEXT@7..10 "<b>"
      AT2@10..12 "@@"
    "###
    );

    insta::assert_debug_snapshot!(
        to_snippet("@@latex:any arbitrary LaTeX code@@").syntax,
        @r###"
    SNIPPET@0..34
      AT2@0..2 "@@"
      TEXT@2..7 "latex"
      COLON@7..8 ":"
      TEXT@8..32 "any arbitrary LaTeX code"
      AT2@32..34 "@@"
    "###
    );

    insta::assert_debug_snapshot!(
        to_snippet("@@html:@@").syntax,
        @r###"
    SNIPPET@0..9
      AT2@0..2 "@@"
      TEXT@2..6 "html"
      COLON@6..7 ":"
      TEXT@7..7 ""
      AT2@7..9 "@@"
    "###
    );

    insta::assert_debug_snapshot!(
        to_snippet("@@html:<p>@</p>@@").syntax,
        @r###"
    SNIPPET@0..17
      AT2@0..2 "@@"
      TEXT@2..6 "html"
      COLON@6..7 ":"
      TEXT@7..15 "<p>@</p>"
      AT2@15..17 "@@"
    "###
    );

    let config = &ParseConfig::default();

    assert!(snippet_node(("@@html:<b>@", config).into()).is_err());
    assert!(snippet_node(("@@html<b>@@", config).into()).is_err());
    assert!(snippet_node(("@@:<b>@@", config).into()).is_err());
}
