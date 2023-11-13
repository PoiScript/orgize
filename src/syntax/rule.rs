use nom::{
    branch::alt,
    bytes::complete::take_while_m_n,
    character::complete::{line_ending, space0},
    combinator::{eof, map},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{blank_lines, debug_assert_lossless, GreenElement, NodeBuilder},
    input::Input,
    SyntaxKind::*,
};

pub fn rule_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert_lossless(map(
        tuple((
            space0,
            take_while_m_n(5, usize::max_value(), |c| c == '-'),
            space0,
            alt((line_ending, eof)),
            blank_lines,
        )),
        |(ws, dashes, ws_, nl, post_blank)| {
            let mut b = NodeBuilder::new();
            b.ws(ws);
            b.text(dashes);
            b.ws(ws_);
            b.nl(nl);
            b.children.extend(post_blank);
            b.finish(RULE)
        },
    ))(input)
}

#[test]
fn parse() {
    use crate::{ast::Rule, tests::to_ast, ParseConfig};

    let to_rule = to_ast::<Rule>(rule_node);

    insta::assert_debug_snapshot!(
        to_rule("-----").syntax,
        @r###"
    RULE@0..5
      TEXT@0..5 "-----"
    "###
    );

    insta::assert_debug_snapshot!(
        to_rule("--------").syntax,
        @r###"
    RULE@0..8
      TEXT@0..8 "--------"
    "###
    );

    insta::assert_debug_snapshot!(
        to_rule("-----\n\n\n").syntax,
        @r###"
    RULE@0..8
      TEXT@0..5 "-----"
      NEW_LINE@5..6 "\n"
      BLANK_LINE@6..7
        NEW_LINE@6..7 "\n"
      BLANK_LINE@7..8
        NEW_LINE@7..8 "\n"
    "###
    );

    insta::assert_debug_snapshot!(
        to_rule("-----  \n").syntax,
        @r###"
    RULE@0..8
      TEXT@0..5 "-----"
      WHITESPACE@5..7 "  "
      NEW_LINE@7..8 "\n"
    "###
    );

    let config = &ParseConfig::default();

    assert!(rule_node(("", config).into()).is_err());
    assert!(rule_node(("----", config).into()).is_err());
    assert!(rule_node(("None----", config).into()).is_err());
    assert!(rule_node(("None  ----", config).into()).is_err());
    assert!(rule_node(("None------", config).into()).is_err());
    assert!(rule_node(("----None----", config).into()).is_err());
    assert!(rule_node(("\t\t----", config).into()).is_err());
    assert!(rule_node(("------None", config).into()).is_err());
    assert!(rule_node(("----- None", config).into()).is_err());
}