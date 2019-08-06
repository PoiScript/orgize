use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_till},
    combinator::opt,
    sequence::{delimited, preceded},
    IResult,
};

use crate::elements::Element;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct InlineCall<'a> {
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub inside_header: Option<Cow<'a, str>>,
    pub arguments: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub end_header: Option<Cow<'a, str>>,
}

impl<'a> InlineCall<'a> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Element<'_>> {
        let (input, name) = preceded(
            tag("call_"),
            take_till(|c| c == '[' || c == '\n' || c == '(' || c == ')'),
        )(input)?;
        let (input, inside_header) = opt(delimited(
            tag("["),
            take_till(|c| c == ']' || c == '\n'),
            tag("]"),
        ))(input)?;
        let (input, arguments) =
            delimited(tag("("), take_till(|c| c == ')' || c == '\n'), tag(")"))(input)?;
        let (input, end_header) = opt(delimited(
            tag("["),
            take_till(|c| c == ']' || c == '\n'),
            tag("]"),
        ))(input)?;

        Ok((
            input,
            Element::InlineCall(InlineCall {
                name: name.into(),
                arguments: arguments.into(),
                inside_header: inside_header.map(Into::into),
                end_header: end_header.map(Into::into),
            }),
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        InlineCall::parse("call_square(4)"),
        Ok((
            "",
            Element::InlineCall(InlineCall {
                name: "square".into(),
                arguments: "4".into(),
                inside_header: None,
                end_header: None,
            }),
        ))
    );
    assert_eq!(
        InlineCall::parse("call_square[:results output](4)"),
        Ok((
            "",
            Element::InlineCall(InlineCall {
                name: "square".into(),
                arguments: "4".into(),
                inside_header: Some(":results output".into()),
                end_header: None,
            }),
        ))
    );
    assert_eq!(
        InlineCall::parse("call_square(4)[:results html]"),
        Ok((
            "",
            Element::InlineCall(InlineCall {
                name: "square".into(),
                arguments: "4".into(),
                inside_header: None,
                end_header: Some(":results html".into()),
            }),
        ))
    );
    assert_eq!(
        InlineCall::parse("call_square[:results output](4)[:results html]"),
        Ok((
            "",
            Element::InlineCall(InlineCall {
                name: "square".into(),
                arguments: "4".into(),
                inside_header: Some(":results output".into()),
                end_header: Some(":results html".into()),
            }),
        ))
    );
}
