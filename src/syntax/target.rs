use nom::{
    bytes::complete::take_while,
    combinator::{map, verify},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{l_angle2_token, node, r_angle2_token, GreenElement},
    input::Input,
    SyntaxKind::*,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn target_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
        tuple((
            l_angle2_token,
            verify(
                take_while(|c: char| c != '<' && c != '\n' && c != '>'),
                |s: &Input| {
                    s.as_str().starts_with(|c| c != ' ') && s.as_str().ends_with(|c| c != ' ')
                },
            ),
            r_angle2_token,
        )),
        |(l_angle2, target, r_angle2)| node(TARGET, [l_angle2, target.text_token(), r_angle2]),
    );
    crate::lossless_parser!(parser, input)
}

#[test]
fn parse() {
    use crate::{ast::Target, tests::to_ast, ParseConfig};

    let to_target = to_ast::<Target>(target_node);

    insta::assert_debug_snapshot!(
        to_target("<<target>>").syntax,
        @r###"
    TARGET@0..10
      L_ANGLE2@0..2 "<<"
      TEXT@2..8 "target"
      R_ANGLE2@8..10 ">>"
    "###
    );

    insta::assert_debug_snapshot!(
        to_target("<<tar get>>").syntax,
        @r###"
    TARGET@0..11
      L_ANGLE2@0..2 "<<"
      TEXT@2..9 "tar get"
      R_ANGLE2@9..11 ">>"
    "###
    );

    let config = &ParseConfig::default();

    assert!(target_node(("<<target >>", config).into()).is_err());
    assert!(target_node(("<< target>>", config).into()).is_err());
    assert!(target_node(("<<ta<get>>", config).into()).is_err());
    assert!(target_node(("<<ta>get>>", config).into()).is_err());
    assert!(target_node(("<<ta\nget>>", config).into()).is_err());
    assert!(target_node(("<<target>", config).into()).is_err());
}
