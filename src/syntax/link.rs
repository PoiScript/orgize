use nom::{
    bytes::complete::take_while,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{
        debug_assert_lossless, l_bracket2_token, l_bracket_token, node, r_bracket2_token,
        r_bracket_token, GreenElement,
    },
    input::Input,
    SyntaxKind::*,
};

pub fn link_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert_lossless(map(
        tuple((
            l_bracket2_token,
            take_while(|c: char| c != '<' && c != '>' && c != '\n' && c != ']'),
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
                children.extend([r_bracket, l_bracket, desc.text_token()]);
            }

            children.push(r_bracket2);

            node(LINK, children)
        },
    ))(input)
}

#[test]
fn parse() {
    use crate::{ast::Link, tests::to_ast, ParseConfig};

    let to_link = to_ast::<Link>(link_node);

    let link = to_link("[[#id]]");
    assert_eq!(link.path().as_ref().map(|x| x.text()), Some("#id"));
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

    let config = &ParseConfig::default();

    assert!(link_node(("[[#id][desc]", config).into()).is_err());
}
