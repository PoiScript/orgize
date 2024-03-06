use nom::{
    branch::alt,
    bytes::complete::{take_until1, take_while1},
    character::complete::alpha1,
    sequence::tuple,
    IResult, InputTake,
};

use crate::SyntaxKind;

use super::{
    combinator::{
        backslash_token, dollar2_token, dollar_token, l_bracket_token, l_curly_token,
        l_parens_token, node, r_bracket_token, r_curly_token, r_parens_token, GreenElement,
    },
    input::Input,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn latex_fragment_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert!(input.s.starts_with(['\\', '$']));
    let mut parser = alt((template1, template2, template3, template4, template5));
    crate::lossless_parser!(parser, input)
}

// \NAME[CONTENTS1] \NAME{CONTENTS1}
fn template1(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, (backslash, name)) = tuple((backslash_token, alpha1))(input)?;
    let (input, (l, content, r)) = alt((
        tuple((
            l_bracket_token,
            take_while1(|c| c != '{' && c != '}' && c != '[' && c != ']' && c != '\r' && c != '\n'),
            r_bracket_token,
        )),
        tuple((
            l_curly_token,
            take_while1(|c| c != '{' && c != '}' && c != '\r' && c != '\n'),
            r_curly_token,
        )),
    ))(input)?;
    Ok((
        input,
        node(
            SyntaxKind::LATEX_FRAGMENT,
            [backslash, name.text_token(), l, content.text_token(), r],
        ),
    ))
}

// \(CONTENTS\)
fn template2(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, (backslash1, l)) = tuple((backslash_token, l_parens_token))(input)?;
    if let Some(i) = jetscii::Substring::new("\\)").find(input.s) {
        let (input, content) = input.take_split(i);
        let (input, (backslash2, r)) = tuple((backslash_token, r_parens_token))(input)?;
        Ok((
            input,
            node(
                SyntaxKind::LATEX_FRAGMENT,
                [backslash1, l, content.text_token(), backslash2, r],
            ),
        ))
    } else {
        Err(nom::Err::Error(()))
    }
}

// \[CONTENTS\]
fn template3(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, (backslash1, l)) = tuple((backslash_token, l_bracket_token))(input)?;
    if let Some(i) = jetscii::Substring::new("\\]").find(input.s) {
        let (input, content) = input.take_split(i);
        let (input, (backslash2, r)) = tuple((backslash_token, r_bracket_token))(input)?;
        Ok((
            input,
            node(
                SyntaxKind::LATEX_FRAGMENT,
                [backslash1, l, content.text_token(), backslash2, r],
            ),
        ))
    } else {
        Err(nom::Err::Error(()))
    }
}

// $$CONTENTS$$
fn template4(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, l) = dollar2_token(input)?;
    let (input, content) = take_until1("$$")(input)?;
    let (input, r) = dollar2_token(input)?;
    Ok((
        input,
        node(SyntaxKind::LATEX_FRAGMENT, [l, content.text_token(), r]),
    ))
}

// $CONTENTS$
fn template5(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, l) = dollar_token(input)?;
    let (input, content) = take_until1("$")(input)?;
    let (input, r) = dollar_token(input)?;

    let b = content.as_bytes()[0];
    if matches!(b, b'\r' | b'\n' | b' ' | b'\t' | b'.' | b',' | b';' | b'$') {
        return Err(nom::Err::Error(()));
    }

    let b = content.as_bytes()[content.s.len() - 1];
    if matches!(b, b'\r' | b'\n' | b' ' | b'\t' | b'.' | b',' | b'$') {
        return Err(nom::Err::Error(()));
    }

    let p = input.bytes().next();
    if let Some(p) = p {
        if !matches!(p, b')' | b'}' | b']' | b'\'' | b'"' | b' ' | b'\r' | b'\n') {
            return Err(nom::Err::Error(()));
        }
    }

    Ok((
        input,
        node(SyntaxKind::LATEX_FRAGMENT, [l, content.text_token(), r]),
    ))
}

#[test]
fn parse() {
    use crate::{ast::LatexFragment, tests::to_ast, ParseConfig};

    let to_fragment = to_ast::<LatexFragment>(latex_fragment_node);

    insta::assert_debug_snapshot!(
        to_fragment("\\enlargethispage{2\\baselineskip}").syntax,
        @r###"
    LATEX_FRAGMENT@0..32
      BACKSLASH@0..1 "\\"
      TEXT@1..16 "enlargethispage"
      L_CURLY@16..17 "{"
      TEXT@17..31 "2\\baselineskip"
      R_CURLY@31..32 "}"
    "###
    );

    insta::assert_debug_snapshot!(
        to_fragment("\\[a\\]").syntax,
        @r###"
    LATEX_FRAGMENT@0..5
      BACKSLASH@0..1 "\\"
      L_BRACKET@1..2 "["
      TEXT@2..3 "a"
      BACKSLASH@3..4 "\\"
      R_BRACKET@4..5 "]"
    "###
    );

    insta::assert_debug_snapshot!(
        to_fragment("\\(e^{i \\pi}\\)").syntax,
        @r###"
    LATEX_FRAGMENT@0..13
      BACKSLASH@0..1 "\\"
      L_PARENS@1..2 "("
      TEXT@2..11 "e^{i \\pi}"
      BACKSLASH@11..12 "\\"
      R_PARENS@12..13 ")"
    "###
    );

    insta::assert_debug_snapshot!(
        to_fragment("$\\frac{1}{3}$").syntax,
        @r###"
    LATEX_FRAGMENT@0..13
      DOLLAR@0..1 "$"
      TEXT@1..12 "\\frac{1}{3}"
      DOLLAR@12..13 "$"
    "###
    );

    insta::assert_debug_snapshot!(
        to_fragment("$a\nb$").syntax,
        @r###"
    LATEX_FRAGMENT@0..5
      DOLLAR@0..1 "$"
      TEXT@1..4 "a\nb"
      DOLLAR@4..5 "$"
    "###
    );

    let c = ParseConfig::default();

    assert!(latex_fragment_node(("$ LaTeXxxx$", &c).into()).is_err());
    assert!(latex_fragment_node(("$LaTeXxxx $", &c).into()).is_err());
    assert!(latex_fragment_node(("$a.$", &c).into()).is_err());
    assert!(latex_fragment_node(("$a$a", &c).into()).is_err());
    assert!(latex_fragment_node(("$$b\nol\nd*", &c).into()).is_err());
    assert!(latex_fragment_node(("$b\nol\nd*", &c).into()).is_err());
}
