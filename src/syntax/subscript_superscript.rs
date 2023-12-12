use memchr::memchr2_iter;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::opt,
    IResult, InputTake,
};

use crate::{
    syntax::{
        combinator::{caret_token, underscore_token},
        object::standard_object_nodes,
    },
    SyntaxKind,
};

use super::{
    combinator::{l_curly_token, node, r_curly_token, GreenElement},
    input::Input,
};

pub fn superscript_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, caret) = caret_token(input)?;

    let mut children = vec![caret];

    if let Ok((input, star)) = tag::<&str, Input, ()>("*")(input) {
        children.push(star.text_token());
        Ok((input, node(SyntaxKind::SUPERSCRIPT, children)))
    } else if let Ok((input, (l, contents, r))) = template1(input) {
        children.push(l);
        children.extend(standard_object_nodes(contents));
        children.push(r);
        Ok((input, node(SyntaxKind::SUPERSCRIPT, children)))
    } else if let Ok((input, (sign, contents))) = template2(input) {
        if let Some(s) = sign {
            children.push(s)
        }
        children.push(contents);
        Ok((input, node(SyntaxKind::SUPERSCRIPT, children)))
    } else {
        Err(nom::Err::Error(()))
    }
}

pub fn subscript_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, underscore) = underscore_token(input)?;

    let mut children = vec![underscore];

    if let Ok((input, star)) = tag::<&str, Input, ()>("*")(input) {
        children.push(star.text_token());
        Ok((input, node(SyntaxKind::SUBSCRIPT, children)))
    } else if let Ok((input, (l, contents, r))) = template1(input) {
        children.push(l);
        children.extend(standard_object_nodes(contents));
        children.push(r);
        Ok((input, node(SyntaxKind::SUBSCRIPT, children)))
    } else if let Ok((input, (sign, contents))) = template2(input) {
        if let Some(s) = sign {
            children.push(s)
        }
        children.push(contents);
        Ok((input, node(SyntaxKind::SUBSCRIPT, children)))
    } else {
        Err(nom::Err::Error(()))
    }
}

fn template1(input: Input) -> IResult<Input, (GreenElement, Input, GreenElement), ()> {
    let (input, l) = l_curly_token(input)?;
    let (input, contents) = balanced_brackets(input)?;
    let (input, r) = r_curly_token(input)?;
    Ok((input, (l, contents, r)))
}

fn template2(input: Input) -> IResult<Input, (Option<GreenElement>, GreenElement), ()> {
    let (input, sign) = opt(alt((tag("+"), tag("-"))))(input)?;

    let (input, contents) =
        take_while1(|c: char| c.is_alphanumeric() || c == ',' || c == '\\' || c == '.')(input)?;

    if contents.s.ends_with(|c: char| !c.is_alphanumeric()) {
        return Err(nom::Err::Error(()));
    }

    Ok((input, (sign.map(|x| x.text_token()), contents.text_token())))
}

fn balanced_brackets(input: Input) -> IResult<Input, Input, ()> {
    let mut pairs = 1;
    let bytes = input.as_bytes();
    for i in memchr2_iter(b'{', b'}', bytes) {
        if bytes[i] == b'{' {
            pairs += 1;
        } else if pairs != 1 {
            pairs -= 1;
        } else {
            return Ok(input.take_split(i));
        }
    }
    Err(nom::Err::Error(()))
}

pub fn verify_pre(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let last = s.as_bytes()[s.len() - 1];
    last != b' ' && last != b'\t'
}

#[test]
fn parse() {
    use crate::ast::Subscript;
    use crate::tests::to_ast;

    let to_subscript = to_ast::<Subscript>(subscript_node);

    insta::assert_debug_snapshot!(
        to_subscript("_*").syntax,
        @r###"
    SUBSCRIPT@0..2
      UNDERSCORE@0..1 "_"
      TEXT@1..2 "*"
    "###
    );

    insta::assert_debug_snapshot!(
        to_subscript("_{*bo\nld*}").syntax,
        @r###"
    SUBSCRIPT@0..10
      UNDERSCORE@0..1 "_"
      L_CURLY@1..2 "{"
      BOLD@2..9
        STAR@2..3 "*"
        TEXT@3..8 "bo\nld"
        STAR@8..9 "*"
      R_CURLY@9..10 "}"
    "###
    );

    insta::assert_debug_snapshot!(
        to_subscript("_+123").syntax,
        @r###"
    SUBSCRIPT@0..5
      UNDERSCORE@0..1 "_"
      TEXT@1..2 "+"
      TEXT@2..5 "123"
    "###
    );

    insta::assert_debug_snapshot!(
        to_subscript("_abc").syntax,
        @r###"
    SUBSCRIPT@0..4
      UNDERSCORE@0..1 "_"
      TEXT@1..4 "abc"
    "###
    );
}
