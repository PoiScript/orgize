use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take, take_until, take_while1},
    combinator::{opt, verify},
    error::ParseError,
    sequence::delimited,
    IResult,
};

/// Macro Object
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Macros<'a> {
    /// Macro name
    pub name: Cow<'a, str>,
    /// Arguments passed to the macro
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub arguments: Option<Cow<'a, str>>,
}

impl Macros<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, Macros<'_>)> {
        parse_macros::<()>(input).ok()
    }

    pub fn into_owned(self) -> Macros<'static> {
        Macros {
            name: self.name.into_owned().into(),
            arguments: self.arguments.map(Into::into).map(Cow::Owned),
        }
    }
}

#[inline]
fn parse_macros<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Macros<'a>, E> {
    let (input, _) = tag("{{{")(input)?;
    let (input, name) = verify(
        take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
        |s: &str| s.starts_with(|c: char| c.is_ascii_alphabetic()),
    )(input)?;
    let (input, arguments) = opt(delimited(tag("("), take_until(")}}}"), take(1usize)))(input)?;
    let (input, _) = tag("}}}")(input)?;

    Ok((
        input,
        Macros {
            name: name.into(),
            arguments: arguments.map(Into::into),
        },
    ))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_macros::<VerboseError<&str>>("{{{poem(red,blue)}}}"),
        Ok((
            "",
            Macros {
                name: "poem".into(),
                arguments: Some("red,blue".into())
            }
        ))
    );
    assert_eq!(
        parse_macros::<VerboseError<&str>>("{{{poem())}}}"),
        Ok((
            "",
            Macros {
                name: "poem".into(),
                arguments: Some(")".into())
            }
        ))
    );
    assert_eq!(
        parse_macros::<VerboseError<&str>>("{{{author}}}"),
        Ok((
            "",
            Macros {
                name: "author".into(),
                arguments: None
            }
        ))
    );
    assert!(parse_macros::<VerboseError<&str>>("{{{0uthor}}}").is_err());
    assert!(parse_macros::<VerboseError<&str>>("{{{author}}").is_err());
    assert!(parse_macros::<VerboseError<&str>>("{{{poem(}}}").is_err());
    assert!(parse_macros::<VerboseError<&str>>("{{{poem)}}}").is_err());
}
