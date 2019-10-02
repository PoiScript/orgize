use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while},
    combinator::verify,
    error::ParseError,
    sequence::delimited,
    IResult,
};

/// Target Object
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Target<'a> {
    /// Target ID
    pub target: Cow<'a, str>,
}

impl Target<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> Option<(&str, Target<'_>)> {
        parse_target::<()>(input).ok()
    }

    pub fn into_owned(self) -> Target<'static> {
        Target {
            target: self.target.into_owned().into(),
        }
    }
}

#[inline]
fn parse_target<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Target<'a>, E> {
    let (input, target) = delimited(
        tag("<<"),
        verify(
            take_while(|c: char| c != '<' && c != '\n' && c != '>'),
            |s: &str| s.starts_with(|c| c != ' ') && s.ends_with(|c| c != ' '),
        ),
        tag(">>"),
    )(input)?;

    Ok((
        input,
        Target {
            target: target.into(),
        },
    ))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_target::<VerboseError<&str>>("<<target>>"),
        Ok((
            "",
            Target {
                target: "target".into()
            }
        ))
    );
    assert_eq!(
        parse_target::<VerboseError<&str>>("<<tar get>>"),
        Ok((
            "",
            Target {
                target: "tar get".into()
            }
        ))
    );
    assert!(parse_target::<VerboseError<&str>>("<<target >>").is_err());
    assert!(parse_target::<VerboseError<&str>>("<< target>>").is_err());
    assert!(parse_target::<VerboseError<&str>>("<<ta<get>>").is_err());
    assert!(parse_target::<VerboseError<&str>>("<<ta>get>>").is_err());
    assert!(parse_target::<VerboseError<&str>>("<<ta\nget>>").is_err());
    assert!(parse_target::<VerboseError<&str>>("<<target>").is_err());
}
