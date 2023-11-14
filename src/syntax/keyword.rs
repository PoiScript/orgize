use nom::{
    bytes::complete::take_till,
    character::complete::space0,
    combinator::{cond, opt},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{
        blank_lines, colon_token, debug_assert_lossless, hash_plus_token, l_bracket_token,
        r_bracket_token, trim_line_end, GreenElement, NodeBuilder,
    },
    input::Input,
    SyntaxKind,
};

pub fn keyword_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert_lossless(keyword_node_base)(input)
}

fn keyword_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, (ws, hash_plus, key)) = tuple((
        space0,
        hash_plus_token,
        take_till(|c: char| c.is_ascii_whitespace() || c == ':' || c == '['),
    ))(input)?;

    let is_babel_call = key.s.eq_ignore_ascii_case("CALL");

    let (input, optional) = cond(
        !is_babel_call,
        opt(tuple((
            l_bracket_token,
            take_till(|c| c == ']' || c == '\n'),
            r_bracket_token,
        ))),
    )(input)?;

    let (input, (colon, (value, ws_, nl), post_blank)) =
        tuple((colon_token, trim_line_end, blank_lines))(input)?;

    let mut b = NodeBuilder::new();

    b.ws(ws);
    b.push(hash_plus);
    b.text(key);
    if let Some(Some((l_bracket, optional, r_bracket))) = optional {
        b.children
            .extend([l_bracket, optional.text_token(), r_bracket]);
    }
    b.push(colon);
    b.ws(ws_);
    b.text(value);
    b.nl(nl);
    b.children.extend(post_blank);

    Ok((
        input,
        b.finish(if is_babel_call {
            SyntaxKind::BABEL_CALL
        } else {
            SyntaxKind::KEYWORD
        }),
    ))
}

pub fn affiliated_keyword_nodes(input: Input) -> IResult<Input, Vec<GreenElement>, ()> {
    use rowan::NodeOrToken;

    let mut children = vec![];
    let mut i = input;

    while !i.is_empty() {
        let Ok((input, keyword)) = keyword_node(i) else {
            break;
        };
        i = input;

        let Some(node) = keyword.as_node() else {
            return Err(nom::Err::Error(()));
        };

        // find the first text token in children
        let Some(NodeOrToken::Token(token)) = node
            .children()
            .find(|t| t.kind() == SyntaxKind::TEXT.into())
        else {
            return Err(nom::Err::Error(()));
        };

        let text = token.text();

        if input.c.affiliated_keywords.iter().all(|w| w != text) && !text.starts_with("ATTR_") {
            return Err(nom::Err::Error(()));
        }

        children.push(keyword);
    }

    Ok((i, children))
}

#[test]
fn parse() {
    use crate::{
        ast::{BabelCall, Keyword},
        tests::to_ast,
        ParseConfig,
    };

    let to_keyword = to_ast::<Keyword>(keyword_node);

    let to_babel_call = to_ast::<BabelCall>(keyword_node);

    insta::assert_debug_snapshot!(
        to_keyword("#+KEY:").syntax,
        @r###"
    KEYWORD@0..6
      HASH_PLUS@0..2 "#+"
      TEXT@2..5 "KEY"
      COLON@5..6 ":"
      TEXT@6..6 ""
    "###
    );

    insta::assert_debug_snapshot!(
        to_keyword("#+KEY: VALUE").syntax,
        @r###"
    KEYWORD@0..12
      HASH_PLUS@0..2 "#+"
      TEXT@2..5 "KEY"
      COLON@5..6 ":"
      TEXT@6..12 " VALUE"
    "###
    );

    insta::assert_debug_snapshot!(
        to_keyword("#+K_E_Y: VALUE").syntax,
        @r###"
    KEYWORD@0..14
      HASH_PLUS@0..2 "#+"
      TEXT@2..7 "K_E_Y"
      COLON@7..8 ":"
      TEXT@8..14 " VALUE"
    "###
    );

    insta::assert_debug_snapshot!(
        to_keyword("#+KEY:VALUE\n").syntax,
        @r###"
    KEYWORD@0..12
      HASH_PLUS@0..2 "#+"
      TEXT@2..5 "KEY"
      COLON@5..6 ":"
      TEXT@6..11 "VALUE"
      NEW_LINE@11..12 "\n"
    "###
    );

    insta::assert_debug_snapshot!(
        to_keyword("#+RESULTS:").syntax,
        @r###"
    KEYWORD@0..10
      HASH_PLUS@0..2 "#+"
      TEXT@2..9 "RESULTS"
      COLON@9..10 ":"
      TEXT@10..10 ""
    "###
    );

    insta::assert_debug_snapshot!(
        to_keyword("#+ATTR_LATEX: :width 5cm\n").syntax,
        @r###"
    KEYWORD@0..25
      HASH_PLUS@0..2 "#+"
      TEXT@2..12 "ATTR_LATEX"
      COLON@12..13 ":"
      TEXT@13..24 " :width 5cm"
      NEW_LINE@24..25 "\n"
    "###
    );

    insta::assert_debug_snapshot!(
        to_babel_call("#+CALL: double(n=4)").syntax,
        @r###"
    BABEL_CALL@0..19
      HASH_PLUS@0..2 "#+"
      TEXT@2..6 "CALL"
      COLON@6..7 ":"
      TEXT@7..19 " double(n=4)"
    "###
    );

    insta::assert_debug_snapshot!(
        to_keyword("#+CAPTION[Short caption]: Longer caption.").syntax,
        @r###"
    KEYWORD@0..41
      HASH_PLUS@0..2 "#+"
      TEXT@2..9 "CAPTION"
      L_BRACKET@9..10 "["
      TEXT@10..23 "Short caption"
      R_BRACKET@23..24 "]"
      COLON@24..25 ":"
      TEXT@25..41 " Longer caption."
    "###
    );

    let config = &ParseConfig::default();

    assert!(keyword_node(("#+KE Y: VALUE", config).into()).is_err());
    assert!(keyword_node(("#+CALL[option]: VALUE", config).into()).is_err());
    assert!(keyword_node(("#+ KEY: VALUE", config).into()).is_err());
}
