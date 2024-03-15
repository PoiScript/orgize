use nom::{
    bytes::complete::{tag_no_case, take_while1},
    character::complete::{space0, space1},
    combinator::{iterator, map, opt},
    sequence::tuple,
    IResult, InputTake,
};

use super::{
    combinator::{
        blank_lines, colon_token, eol_or_eof, line_starts_iter, node, plus_token, trim_line_end,
        GreenElement, NodeBuilder,
    },
    element::element_nodes,
    input::Input,
    SyntaxKind::*,
};

fn drawer_begin_node(input: Input) -> IResult<Input, (GreenElement, &str), ()> {
    let mut b = NodeBuilder::new();

    let (input, (ws, colon, name, colon_, ws_, nl)) = tuple((
        space0,
        colon_token,
        take_while1(|c: char| c.is_ascii_alphabetic() || c == '-' || c == '_'),
        colon_token,
        space0,
        eol_or_eof,
    ))(input)?;

    b.ws(ws);
    b.push(colon);
    b.text(name);
    b.push(colon_);
    b.ws(ws_);
    b.nl(nl);

    Ok((input, (b.finish(DRAWER_BEGIN), name.as_str())))
}

fn drawer_end_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, (ws, colon, end, colon_, ws_, nl)) = tuple((
        space0,
        colon_token,
        tag_no_case("END"),
        colon_token,
        space0,
        eol_or_eof,
    ))(input)?;

    let mut b = NodeBuilder::new();
    b.ws(ws);
    b.push(colon);
    b.text(end);
    b.push(colon_);
    b.ws(ws_);
    b.nl(nl);

    Ok((input, b.finish(DRAWER_END)))
}

fn drawer_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, (begin, _)) = drawer_begin_node(input)?;

    let (input, pre_blank) = blank_lines(input)?;

    for (input, contents) in line_starts_iter(input.as_str()).map(|i| input.take_split(i)) {
        if let Ok((input, end)) = drawer_end_node(input) {
            let (input, post_blank) = blank_lines(input)?;
            let mut children = vec![begin];
            children.extend(pre_blank);
            if !contents.is_empty() {
                children.push(node(DRAWER_CONTENT, element_nodes(contents)?));
            } else {
                children.push(node(DRAWER_CONTENT, []));
            }
            children.push(end);
            children.extend(post_blank);

            return Ok((input, node(DRAWER, children)));
        }
    }

    Err(nom::Err::Error(()))
}

fn property_drawer_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, (begin, name)) = drawer_begin_node(input)?;

    if name != "PROPERTIES" {
        return Err(nom::Err::Error(()));
    }

    let mut children = vec![begin];

    let mut it = iterator(input, node_property_node);
    children.extend(&mut it);
    let (input, _) = it.finish()?;
    let (input, end) = drawer_end_node(input)?;

    children.push(end);

    Ok((input, node(PROPERTY_DRAWER, children)))
}

fn node_property_node(input: Input) -> IResult<Input, GreenElement, ()> {
    map(
        tuple((
            space0,
            colon_token,
            take_while1(|c| c != ':' && c != '+'),
            opt(plus_token),
            colon_token,
            space1,
            trim_line_end,
        )),
        |(ws, colon, name, plus, colon_, ws_, (value, ws__, nl))| {
            let mut b = NodeBuilder::new();
            b.ws(ws);
            b.push(colon);
            b.text(name);
            b.push_opt(plus);
            b.push(colon_);
            b.ws(ws_);
            b.text(value);
            b.ws(ws__);
            b.nl(nl);
            b.finish(NODE_PROPERTY)
        },
    )(input)
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn property_drawer_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert!(!input.is_empty());
    crate::lossless_parser!(property_drawer_node_base, input)
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn drawer_node(input: Input) -> IResult<Input, GreenElement, ()> {
    crate::lossless_parser!(drawer_node_base, input)
}

#[test]
fn parse() {
    use crate::{ast::Drawer, tests::to_ast, ParseConfig};

    let to_drawer = to_ast::<Drawer>(drawer_node);

    insta::assert_debug_snapshot!(
        to_drawer(
            r#":DRAWER:
  :CUSTOM_ID: id
  :END:"#
        ).syntax,
       @r###"
    DRAWER@0..33
      DRAWER_BEGIN@0..9
        COLON@0..1 ":"
        TEXT@1..7 "DRAWER"
        COLON@7..8 ":"
        NEW_LINE@8..9 "\n"
      DRAWER_CONTENT@9..26
        PARAGRAPH@9..26
          TEXT@9..18 "  :CUSTOM"
          SUBSCRIPT@18..21
            UNDERSCORE@18..19 "_"
            TEXT@19..21 "ID"
          TEXT@21..26 ": id\n"
      DRAWER_END@26..33
        WHITESPACE@26..28 "  "
        COLON@28..29 ":"
        TEXT@29..32 "END"
        COLON@32..33 ":"
    "###
    );

    insta::assert_debug_snapshot!(
        to_drawer(
            r#":DRAWER:

  :END:

"#
        ).syntax,
        @r###"
    DRAWER@0..19
      DRAWER_BEGIN@0..9
        COLON@0..1 ":"
        TEXT@1..7 "DRAWER"
        COLON@7..8 ":"
        NEW_LINE@8..9 "\n"
      BLANK_LINE@9..10 "\n"
      DRAWER_CONTENT@10..10
      DRAWER_END@10..18
        WHITESPACE@10..12 "  "
        COLON@12..13 ":"
        TEXT@13..16 "END"
        COLON@16..17 ":"
        NEW_LINE@17..18 "\n"
      BLANK_LINE@18..19 "\n"
    "###
    );

    let config = &ParseConfig::default();

    // https://github.com/PoiScript/orgize/issues/9
    assert!(drawer_node((":SPAGHETTI:\n", config).into()).is_err());
}
