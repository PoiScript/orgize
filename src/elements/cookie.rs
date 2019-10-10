use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit0,
    combinator::recognize,
    error::ParseError,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

/// Statistics Cookie Object
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Cookie<'a> {
    /// Full cookie value
    pub value: Cow<'a, str>,
}

impl Cookie<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, Cookie)> {
        parse_cookie::<()>(input).ok()
    }

    pub fn into_owned(self) -> Cookie<'static> {
        Cookie {
            value: self.value.into_owned().into(),
        }
    }
}

#[inline]
fn parse_cookie<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Cookie, E> {
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
    use nom::error::VerboseError;

    assert_eq!(
        parse_cookie::<VerboseError<&str>>("[1/10]"),
        Ok((
            "",
            Cookie {
                value: "[1/10]".into()
            }
        ))
    );
    assert_eq!(
        parse_cookie::<VerboseError<&str>>("[1/1000]"),
        Ok((
            "",
            Cookie {
                value: "[1/1000]".into()
            }
        ))
    );
    assert_eq!(
        parse_cookie::<VerboseError<&str>>("[10%]"),
        Ok((
            "",
            Cookie {
                value: "[10%]".into()
            }
        ))
    );
    assert_eq!(
        parse_cookie::<VerboseError<&str>>("[%]"),
        Ok((
            "",
            Cookie {
                value: "[%]".into()
            }
        ))
    );
    assert_eq!(
        parse_cookie::<VerboseError<&str>>("[/]"),
        Ok((
            "",
            Cookie {
                value: "[/]".into()
            }
        ))
    );
    assert_eq!(
        parse_cookie::<VerboseError<&str>>("[100/]"),
        Ok((
            "",
            Cookie {
                value: "[100/]".into()
            }
        ))
    );
    assert_eq!(
        parse_cookie::<VerboseError<&str>>("[/100]"),
        Ok((
            "",
            Cookie {
                value: "[/100]".into()
            }
        ))
    );

    assert!(parse_cookie::<VerboseError<&str>>("[10% ]").is_err());
    assert!(parse_cookie::<VerboseError<&str>>("[1//100]").is_err());
    assert!(parse_cookie::<VerboseError<&str>>("[1\\100]").is_err());
    assert!(parse_cookie::<VerboseError<&str>>("[10%%]").is_err());
}
