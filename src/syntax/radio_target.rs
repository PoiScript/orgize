use nom::{
    bytes::complete::take_while,
    combinator::{map, verify},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{l_angle3_token, node, r_angle3_token, GreenElement},
    input::Input,
    SyntaxKind::*,
};

// TODO: text-markup, entities, latex-fragments, subscript and superscript

pub fn radio_target_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
        tuple((
            l_angle3_token,
            verify(
                take_while(|c: char| c != '<' && c != '\n' && c != '>'),
                |s: &Input| {
                    s.as_str().starts_with(|c| c != ' ') && s.as_str().ends_with(|c| c != ' ')
                },
            ),
            r_angle3_token,
        )),
        |(l_angle3, contents, r_angle3)| {
            node(RADIO_TARGET, [l_angle3, contents.text_token(), r_angle3])
        },
    );
    crate::lossless_parser!(parser, input)
}

#[test]
fn parse() {
    use crate::{ast::RadioTarget, tests::to_ast, ParseConfig};

    let to_radio_target = to_ast::<RadioTarget>(radio_target_node);

    insta::assert_debug_snapshot!(
        to_radio_target("<<<target>>>").syntax,
        @r###"
    RADIO_TARGET@0..12
      L_ANGLE3@0..3 "<<<"
      TEXT@3..9 "target"
      R_ANGLE3@9..12 ">>>"
    "###
    );

    insta::assert_debug_snapshot!(
        to_radio_target("<<<tar get>>>").syntax,
        @r###"
    RADIO_TARGET@0..13
      L_ANGLE3@0..3 "<<<"
      TEXT@3..10 "tar get"
      R_ANGLE3@10..13 ">>>"
    "###
    );

    let config = &ParseConfig::default();

    assert!(radio_target_node(("<<<target >>>", config).into()).is_err());
    assert!(radio_target_node(("<<< target>>>", config).into()).is_err());
    assert!(radio_target_node(("<<<ta<get>>>", config).into()).is_err());
    assert!(radio_target_node(("<<<ta>get>>>", config).into()).is_err());
    assert!(radio_target_node(("<<<ta\nget>>>", config).into()).is_err());
    assert!(radio_target_node(("<<<target>>", config).into()).is_err());
}
