use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit0,
    combinator::recognize,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

/// Statistics Cookie Object
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Cookie<'a> {
    /// Full cookie value
    pub value: Cow<'a, str>,
}

impl Cookie<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, Cookie)> {
        parse_internal(input).ok()
    }

    pub fn into_owned(self) -> Cookie<'static> {
        Cookie {
            value: self.value.into_owned().into(),
        }
    }
}

#[inline]
fn parse_internal(input: &str) -> IResult<&str, Cookie, ()> {
    let (input, value) = recognize(delimited(
        tag("["),
        alt((
            separated_pair(digit0, tag("/"), digit0),
            pair(digit0, tag("%")),
        )),
        tag("]"),
    ))(input)?;

    Ok((
        input,
        Cookie {
            value: value.into(),
        },
    ))
}

#[test]
fn parse() {
    assert_eq!(
        Cookie::parse("[1/10]"),
        Some((
            "",
            Cookie {
                value: "[1/10]".into()
            }
        ))
    );
    assert_eq!(
        Cookie::parse("[1/1000]"),
        Some((
            "",
            Cookie {
                value: "[1/1000]".into()
            }
        ))
    );
    assert_eq!(
        Cookie::parse("[10%]"),
        Some((
            "",
            Cookie {
                value: "[10%]".into()
            }
        ))
    );
    assert_eq!(
        Cookie::parse("[%]"),
        Some((
            "",
            Cookie {
                value: "[%]".into()
            }
        ))
    );
    assert_eq!(
        Cookie::parse("[/]"),
        Some((
            "",
            Cookie {
                value: "[/]".into()
            }
        ))
    );
    assert_eq!(
        Cookie::parse("[100/]"),
        Some((
            "",
            Cookie {
                value: "[100/]".into()
            }
        ))
    );
    assert_eq!(
        Cookie::parse("[/100]"),
        Some((
            "",
            Cookie {
                value: "[/100]".into()
            }
        ))
    );

    assert!(Cookie::parse("[10% ]").is_none());
    assert!(Cookie::parse("[1//100]").is_none());
    assert!(Cookie::parse("[1\\100]").is_none());
    assert!(Cookie::parse("[10%%]").is_none());
}
