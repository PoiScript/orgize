use nom::{character::complete::space0, combinator::map, sequence::tuple, IResult};

use crate::{
    syntax::combinator::{backslash_token, eol_or_eof, node},
    SyntaxKind,
};

use super::{combinator::GreenElement, input::Input};

pub fn line_break_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert!(input.s.starts_with('\\'));
    let mut parser = map(
        tuple((backslash_token, backslash_token, space0, eol_or_eof)),
        |(b1, b2, ws, nl)| {
            node(
                SyntaxKind::LINE_BREAK,
                [b1, b2, ws.ws_token(), nl.nl_token()],
            )
        },
    );
    crate::lossless_parser!(parser, input)
}

#[test]
fn parse() {
    use crate::ast::LineBreak;
    use crate::tests::to_ast;

    let to_line_break = to_ast::<LineBreak>(line_break_node);

    insta::assert_debug_snapshot!(
        to_line_break("\\\\\n").syntax,
        @r###"
    LINE_BREAK@0..3
      BACKSLASH@0..1 "\\"
      BACKSLASH@1..2 "\\"
      WHITESPACE@2..2 ""
      NEW_LINE@2..3 "\n"
    "###
    );
    insta::assert_debug_snapshot!(
        to_line_break("\\\\   \n").syntax,
        @r###"
    LINE_BREAK@0..6
      BACKSLASH@0..1 "\\"
      BACKSLASH@1..2 "\\"
      WHITESPACE@2..5 "   "
      NEW_LINE@5..6 "\n"
    "###
    );
    insta::assert_debug_snapshot!(
        to_line_break("\\\\\r\n").syntax,
        @r###"
    LINE_BREAK@0..4
      BACKSLASH@0..1 "\\"
      BACKSLASH@1..2 "\\"
      WHITESPACE@2..2 ""
      NEW_LINE@2..4 "\r\n"
    "###
    );
    insta::assert_debug_snapshot!(
        to_line_break("\\\\    ").syntax,
        @r###"
    LINE_BREAK@0..6
      BACKSLASH@0..1 "\\"
      BACKSLASH@1..2 "\\"
      WHITESPACE@2..6 "    "
      NEW_LINE@6..6 ""
    "###
    );
}
