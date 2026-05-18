use nom::{
    bytes::complete::take_while,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{
        l_bracket2_token, l_bracket_token, node, r_bracket2_token, r_bracket_token, GreenElement,
    },
    input::Input,
    object::link_description_object_nodes,
    SyntaxKind::*,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn link_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
        tuple((
            l_bracket2_token,
            take_while(|c: char| c != '<' && c != '>' && c != ']'),
            opt(tuple((
                r_bracket_token,
                l_bracket_token,
                take_while(|c: char| c != '[' && c != ']'),
            ))),
            r_bracket2_token,
        )),
        |(l_bracket2, path, desc, r_bracket2)| {
            let mut children = vec![l_bracket2, path.token(LINK_PATH)];

            if let Some((r_bracket, l_bracket, desc)) = desc {
                children.extend([r_bracket, l_bracket]);
                children.extend(link_description_object_nodes(desc));
            }

            children.push(r_bracket2);

            node(LINK, children)
        },
    );
    crate::lossless_parser!(parser, input)
}

#[test]
fn parse() {
    use crate::{ast::Link, tests::to_ast, ParseConfig};

    let to_link = to_ast::<Link>(link_node);

    let link = to_link("[[#id]]");
    insta::assert_debug_snapshot!(
        link.syntax,
        @r###"
    LINK@0..7
      L_BRACKET2@0..2 "[["
      LINK_PATH@2..5 "#id"
      R_BRACKET2@5..7 "]]"
    "###
    );

    let link = to_link("[[#id][desc]]");
    insta::assert_debug_snapshot!(
        link.syntax,
        @r###"
    LINK@0..13
      L_BRACKET2@0..2 "[["
      LINK_PATH@2..5 "#id"
      R_BRACKET@5..6 "]"
      L_BRACKET@6..7 "["
      TEXT@7..11 "desc"
      R_BRACKET2@11..13 "]]"
    "###
    );

    let link = to_link("[[file:/home/dominik/images/jupiter.jpg]]");
    insta::assert_debug_snapshot!(
        link.syntax,
        @r###"
    LINK@0..41
      L_BRACKET2@0..2 "[["
      LINK_PATH@2..39 "file:/home/dominik/im ..."
      R_BRACKET2@39..41 "]]"
    "###
    );

    let link = to_link("[[https://orgmode.org][*bold* description]]");
    insta::assert_debug_snapshot!(
        link.syntax,
        @r###"
    LINK@0..43
      L_BRACKET2@0..2 "[["
      LINK_PATH@2..21 "https://orgmode.org"
      R_BRACKET@21..22 "]"
      L_BRACKET@22..23 "["
      BOLD@23..29
        STAR@23..24 "*"
        TEXT@24..28 "bold"
        STAR@28..29 "*"
      TEXT@29..41 " description"
      R_BRACKET2@41..43 "]]"
    "###
    );

    let config = &ParseConfig::default();

    assert!(link_node(("[[#id][desc]", config).into()).is_err());
}

/// Emacs org-mode accepts a single newline inside a bracket link — both inside
/// the path and inside the description.  These tests document the discrepancy
/// against orgize and serve as the reproducer for the upstream issue/PR.
#[test]
fn parse_multiline_link() {
    use crate::{ast::Link, tests::to_ast};

    let to_link = to_ast::<Link>(link_node);

    // Path-only link with a newline inside the path.
    let link = to_link("[[really really long link\nthat just keeps going]]");
    insta::assert_debug_snapshot!(
        link.syntax,
        @r###"
    LINK@0..49
      L_BRACKET2@0..2 "[["
      LINK_PATH@2..47 "really really long li ..."
      R_BRACKET2@47..49 "]]"
    "###
    );

    // Link with separate description that contains a newline.
    let link = to_link("[[https://example.com][some\ndescription]]");
    insta::assert_debug_snapshot!(
        link.syntax,
        @r###"
    LINK@0..41
      L_BRACKET2@0..2 "[["
      LINK_PATH@2..21 "https://example.com"
      R_BRACKET@21..22 "]"
      L_BRACKET@22..23 "["
      TEXT@23..39 "some\ndescription"
      R_BRACKET2@39..41 "]]"
    "###
    );
}

/// `Org::parse` should also pick up multi-line links inside paragraphs.  This
/// is the end-to-end case relevant to downstream formatters (e.g. org-fmt).
#[test]
fn parse_multiline_link_in_paragraph() {
    use crate::{ast::Link, Org};
    use rowan::ast::AstNode;

    let org = Org::parse(
        "Here is a [[really really long link\nthat just keeps going]] in prose.\n",
    );

    let links: Vec<_> = org
        .document()
        .syntax()
        .descendants()
        .filter_map(Link::cast)
        .collect();

    assert_eq!(
        links.len(),
        1,
        "expected one link, found {}",
        links.len(),
    );
}
