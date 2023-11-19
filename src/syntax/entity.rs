use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::alphanumeric1,
    combinator::opt,
    IResult,
};

use crate::{
    entities::ENTITIES,
    syntax::combinator::{backslash_token, node},
    SyntaxKind,
};

use super::{combinator::GreenElement, input::Input};

pub fn entity_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert!(input.s.starts_with('\\'));
    let mut parser = alt((template1, template2));
    crate::lossless_parser!(parser, input)
}

// \NAME POST or // \NAME{}
fn template1(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, backslash) = backslash_token(input)?;
    let (input, name) = alphanumeric1(input)?;

    if ENTITIES.iter().all(|i| i.0 != name.s) {
        return Err(nom::Err::Error(()));
    }
    let (input, brackets) = opt(tag("{}"))(input)?;

    if let Some(brackets) = brackets {
        return Ok((
            input,
            node(
                SyntaxKind::ENTITY,
                [backslash, name.text_token(), brackets.text_token()],
            ),
        ));
    }

    if let Some(post) = input.bytes().next() {
        if post.is_ascii_alphabetic() {
            return Err(nom::Err::Error(()));
        }
    }

    Ok((
        input,
        node(SyntaxKind::ENTITY, [backslash, name.text_token()]),
    ))
}

// \_SPACES
fn template2(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, backslash) = backslash_token(input)?;
    let (input, underscore) = tag("_")(input)?;
    let (input, spaces) = take_while_m_n(1, 20, |c| c == ' ')(input)?;
    Ok((
        input,
        node(
            SyntaxKind::ENTITY,
            [
                backslash,
                underscore.token(SyntaxKind::UNDERSCORE),
                spaces.text_token(),
            ],
        ),
    ))
}

#[test]
fn parse() {
    use crate::{ast::Entity, tests::to_ast, ParseConfig};

    let to_entity = to_ast::<Entity>(entity_node);

    insta::assert_debug_snapshot!(
        to_entity("\\cent").syntax,
        @r###"
    ENTITY@0..5
      BACKSLASH@0..1 "\\"
      TEXT@1..5 "cent"
    "###
    );

    insta::assert_debug_snapshot!(
        to_entity("\\S").syntax,
        @r###"
    ENTITY@0..2
      BACKSLASH@0..1 "\\"
      TEXT@1..2 "S"
    "###
    );

    insta::assert_debug_snapshot!(
        to_entity("\\frac12{}test").syntax,
        @r###"
    ENTITY@0..9
      BACKSLASH@0..1 "\\"
      TEXT@1..7 "frac12"
      TEXT@7..9 "{}"
    "###
    );

    insta::assert_debug_snapshot!(
        to_entity("\\_                   ").syntax,
        @r###"
    ENTITY@0..21
      BACKSLASH@0..1 "\\"
      UNDERSCORE@1..2 "_"
      TEXT@2..21 "                   "
    "###
    );

    let c = ParseConfig::default();

    assert!(entity_node(("\\poi", &c).into()).is_err());
}
