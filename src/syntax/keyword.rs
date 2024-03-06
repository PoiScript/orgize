#![allow(clippy::type_complexity)]

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::complete::space0,
    combinator::{recognize, verify},
    sequence::tuple,
    IResult, InputTake,
};

use super::{
    combinator::{blank_lines, hash_plus_token, node, trim_line_end, GreenElement},
    input::Input,
    SyntaxKind,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn keyword_node(input: Input) -> IResult<Input, GreenElement, ()> {
    fn f(input: Input) -> IResult<Input, GreenElement, ()> {
        let (input, (key, mut nodes)) = keyword_node_base(input)?;
        let (input, post_blank) = blank_lines(input)?;
        nodes.extend(post_blank);
        Ok((
            input,
            node(
                if key == "CALL" {
                    SyntaxKind::BABEL_CALL
                } else {
                    SyntaxKind::KEYWORD
                },
                nodes,
            ),
        ))
    }
    crate::lossless_parser!(f, input)
}

/// Return empty vector if input doesn't contain affiliated keyword, or affiliated keyword is
/// followed by blank lines.
#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn affiliated_keyword_nodes(input: Input) -> IResult<Input, Vec<GreenElement>, ()> {
    let mut children = vec![];
    let mut i = input;

    while !i.is_empty() {
        let Ok((input_, (key, nodes))) = keyword_node_base(i) else {
            break;
        };

        let (input_, post_blank) = blank_lines(input_)?;

        // affiliated keyword can not followed by blank lines or eof
        if !post_blank.is_empty() || input_.is_empty() {
            return Ok((input, vec![]));
        }

        if input_.c.affiliated_keywords.iter().all(|w| w != key) && !key.starts_with("ATTR_") {
            break;
        }

        debug_assert!(i.len() > input_.len(), "{} > {}", i.len(), input_.len());
        i = input_;
        children.push(node(SyntaxKind::AFFILIATED_KEYWORD, nodes));
    }

    Ok((i, children))
}

pub fn tblfm_keyword_nodes(input: Input) -> IResult<Input, Vec<GreenElement>, ()> {
    let mut children = vec![];
    let mut i = input;

    while !i.is_empty() {
        let Ok((input, (key, nodes))) = keyword_node_base(i) else {
            break;
        };

        if !key.eq_ignore_ascii_case("TBLFM") {
            break;
        }

        debug_assert!(i.len() > input.len(), "{} > {}", i.len(), input.len());
        i = input;
        children.push(node(SyntaxKind::KEYWORD, nodes));
    }

    Ok((i, children))
}

fn keyword_node_base(input: Input) -> IResult<Input, (&str, Vec<GreenElement>), ()> {
    let (input, (ws, hash_plus)) = tuple((space0, hash_plus_token))(input)?;

    let (input, (key, optional, colon)) = alt((key_with_optional, key))(input)?;

    let (input, (value, ws_, nl)) = trim_line_end(input)?;

    let mut children = vec![];
    if !ws.is_empty() {
        children.push(ws.ws_token());
    }
    children.push(hash_plus);
    children.push(key.text_token());
    if let Some((l_bracket, optional, r_bracket)) = optional {
        children.push(l_bracket.token(SyntaxKind::L_BRACKET));
        children.push(optional.text_token());
        children.push(r_bracket.token(SyntaxKind::R_BRACKET));
    }
    children.push(colon.token(SyntaxKind::COLON));
    children.push(value.text_token());
    if !ws_.is_empty() {
        children.push(ws_.ws_token());
    }
    if !nl.is_empty() {
        children.push(nl.nl_token());
    }

    Ok((input, (key.s, children)))
}

fn key(input: Input) -> IResult<Input, (Input, Option<(Input, Input, Input)>, Input), ()> {
    let (input, output) = verify(
        recognize(tuple((
            take_till(|c: char| c.is_ascii_whitespace() || c == ':'),
            take_while1(|c: char| c == ':'),
        ))),
        |i: &Input| i.len() >= 2,
    )(input)?;
    let (colon, key) = output.take_split(output.len() - 1);
    Ok((input, (key, None, colon)))
}

fn key_with_optional(
    input: Input,
) -> IResult<Input, (Input, Option<(Input, Input, Input)>, Input), ()> {
    let (input, (key, r_backer, optional, l_backer, colon)) = tuple((
        alt((tag("CAPTION"), tag("RESULTS"))),
        tag("["),
        take_till(|c| c == '\r' || c == '\n' || c == ']'),
        tag("]"),
        tag(":"),
    ))(input)?;
    Ok((input, (key, Some((r_backer, optional, l_backer)), colon)))
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

    to_keyword("#+KEY:");
    to_keyword("#+::");
    to_keyword("#+::");
    to_keyword("#+:: ");
    to_keyword("#+:: \n");
    to_keyword("#+::\n");

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
        to_keyword("#+ABC[OPTIONAL]: Longer value.").syntax,
        @r###"
    KEYWORD@0..30
      HASH_PLUS@0..2 "#+"
      TEXT@2..15 "ABC[OPTIONAL]"
      COLON@15..16 ":"
      TEXT@16..30 " Longer value."
    "###
    );

    insta::assert_debug_snapshot!(
        to_keyword("#+CAPTION: value").syntax,
        @r###"
    KEYWORD@0..16
      HASH_PLUS@0..2 "#+"
      TEXT@2..9 "CAPTION"
      COLON@9..10 ":"
      TEXT@10..16 " value"
    "###
    );

    insta::assert_debug_snapshot!(
        to_keyword("#+CAPTION[caption optional]: value").syntax,
        @r###"
    KEYWORD@0..34
      HASH_PLUS@0..2 "#+"
      TEXT@2..9 "CAPTION"
      L_BRACKET@9..10 "["
      TEXT@10..26 "caption optional"
      R_BRACKET@26..27 "]"
      COLON@27..28 ":"
      TEXT@28..34 " value"
    "###
    );

    let config = &ParseConfig::default();

    assert!(keyword_node(("#+KE Y: VALUE", config).into()).is_err());
    assert!(keyword_node(("#+ KEY: VALUE", config).into()).is_err());
}
