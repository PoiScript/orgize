use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    sequence::tuple,
    IResult, InputTake,
};

use crate::SyntaxKind;

use super::{
    combinator::{eol_or_eof, l_curly_token, line_starts_iter, node, r_curly_token, GreenElement},
    input::Input,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn latex_environment_node(input: Input) -> IResult<Input, GreenElement, ()> {
    crate::lossless_parser!(latex_environment_node_base, input)
}

fn latex_environment_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, (ws1, begin, l1, name1, r1)) = tuple((
        space0,
        tag("\\begin"),
        l_curly_token,
        take_while1(|c: char| c.is_ascii_alphanumeric() || c == '*'),
        r_curly_token,
    ))(input)?;

    for (input, contents) in line_starts_iter(input.s).map(|i| input.take_split(i)) {
        if let Ok((input, (ws2, end, l2, name2, r2, ws3, nl))) = tuple((
            space0,
            tag("\\end"),
            l_curly_token,
            tag(name1.s),
            r_curly_token,
            space0,
            eol_or_eof,
        ))(input)
        {
            return Ok((
                input,
                node(
                    SyntaxKind::LATEX_ENVIRONMENT,
                    [
                        ws1.ws_token(),
                        begin.text_token(),
                        l1,
                        name1.text_token(),
                        r1,
                        contents.text_token(),
                        ws2.ws_token(),
                        end.text_token(),
                        l2,
                        name2.text_token(),
                        r2,
                        ws3.ws_token(),
                        nl.nl_token(),
                    ],
                ),
            ));
        }
    }

    Err(nom::Err::Error(()))
}

#[test]
fn parse() {
    use crate::ast::LatexEnvironment;
    use crate::config::ParseConfig;
    use crate::tests::to_ast;

    let to_latex = to_ast::<LatexEnvironment>(latex_environment_node);

    insta::assert_debug_snapshot!(
        to_latex(r"\begin{NAME}\end{NAME}").syntax,
        @r###"
    LATEX_ENVIRONMENT@0..22
      WHITESPACE@0..0 ""
      TEXT@0..6 "\\begin"
      L_CURLY@6..7 "{"
      TEXT@7..11 "NAME"
      R_CURLY@11..12 "}"
      TEXT@12..12 ""
      WHITESPACE@12..12 ""
      TEXT@12..16 "\\end"
      L_CURLY@16..17 "{"
      TEXT@17..21 "NAME"
      R_CURLY@21..22 "}"
      WHITESPACE@22..22 ""
      NEW_LINE@22..22 ""
    "###
    );

    insta::assert_debug_snapshot!(
        to_latex(
        r"\begin{align*}
    2x - 5y &= 8 \\
    3x + 9y &= -12
    \end{align*}"
        ).syntax,
        @r###"
    LATEX_ENVIRONMENT@0..70
      WHITESPACE@0..0 ""
      TEXT@0..6 "\\begin"
      L_CURLY@6..7 "{"
      TEXT@7..13 "align*"
      R_CURLY@13..14 "}"
      TEXT@14..54 "\n    2x - 5y &= 8 \\\\\n ..."
      WHITESPACE@54..58 "    "
      TEXT@58..62 "\\end"
      L_CURLY@62..63 "{"
      TEXT@63..69 "align*"
      R_CURLY@69..70 "}"
      WHITESPACE@70..70 ""
      NEW_LINE@70..70 ""
    "###
    );

    let c = ParseConfig::default();

    assert!(latex_environment_node((r"\begin{equation}\end{align}", &c).into()).is_err());
    assert!(latex_environment_node((r"\begin{_}\end{_}", &c).into()).is_err());
}
