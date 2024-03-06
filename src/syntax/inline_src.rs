use nom::{
    bytes::complete::{tag, take_till, take_while1},
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{
        l_bracket_token, l_curly_token, node, r_bracket_token, r_curly_token, GreenElement,
    },
    input::Input,
    SyntaxKind,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn inline_src_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
        tuple((
            tag("src_"),
            take_while1(|c: char| !c.is_ascii_whitespace() && c != '[' && c != '{'),
            opt(tuple((
                l_bracket_token,
                take_till(|c| c == '\n' || c == ']'),
                r_bracket_token,
            ))),
            l_curly_token,
            take_till(|c| c == '\n' || c == '}'),
            r_curly_token,
        )),
        |(src, lang, options, l_curly, body, r_curly)| {
            let mut children = vec![src.text_token(), lang.text_token()];
            if let Some((l_bracket, options, r_bracket)) = options {
                children.push(l_bracket);
                children.push(options.text_token());
                children.push(r_bracket);
            }
            children.push(l_curly);
            children.push(body.text_token());
            children.push(r_curly);
            node(SyntaxKind::INLINE_SRC, children)
        },
    );
    crate::lossless_parser!(parser, input)
}

#[test]
fn parse() {
    use crate::{ast::InlineSrc, tests::to_ast, ParseConfig};

    let to_inline_src = to_ast::<InlineSrc>(inline_src_node);

    insta::assert_debug_snapshot!(
        to_inline_src("src_C{int a = 0;}").syntax,
        @r###"
    INLINE_SRC@0..17
      TEXT@0..4 "src_"
      TEXT@4..5 "C"
      L_CURLY@5..6 "{"
      TEXT@6..16 "int a = 0;"
      R_CURLY@16..17 "}"
    "###
    );

    insta::assert_debug_snapshot!(
        to_inline_src("src_xml[:exports code]{<tag>text</tag>}").syntax,
        @r###"
    INLINE_SRC@0..39
      TEXT@0..4 "src_"
      TEXT@4..7 "xml"
      L_BRACKET@7..8 "["
      TEXT@8..21 ":exports code"
      R_BRACKET@21..22 "]"
      L_CURLY@22..23 "{"
      TEXT@23..38 "<tag>text</tag>"
      R_CURLY@38..39 "}"
    "###
    );

    let config = &ParseConfig::default();

    assert!(inline_src_node(("src_xml[:exports code]{<tag>text</tag>", config).into()).is_err());
    assert!(inline_src_node(("src_[:exports code]{<tag>text</tag>}", config).into()).is_err());
    assert!(inline_src_node(("src_xml[:exports code]", config).into()).is_err());
}
