use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    error::ParseError,
    sequence::delimited,
    IResult,
};

/// Link Object
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Link<'a> {
    /// Link destination
    pub path: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub desc: Option<Cow<'a, str>>,
}

impl Link<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> Option<(&str, Link)> {
        parse_link::<()>(input).ok()
    }

    pub fn into_owned(self) -> Link<'static> {
        Link {
            path: self.path.into_owned().into(),
            desc: self.desc.map(Into::into).map(Cow::Owned),
        }
    }
}

#[inline]
fn parse_link<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Link, E> {
    let (input, path) = delimited(
        tag("[["),
        take_while(|c: char| c != '<' && c != '>' && c != '\n' && c != ']'),
        tag("]"),
    )(input)?;
    let (input, desc) = opt(delimited(
        tag("["),
        take_while(|c: char| c != '[' && c != ']'),
        tag("]"),
    ))(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((
        input,
        Link {
            path: path.into(),
            desc: desc.map(Into::into),
        },
    ))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_link::<VerboseError<&str>>("[[#id]]"),
        Ok((
            "",
            Link {
                path: "#id".into(),
                desc: None
            }
        ))
    );
    assert_eq!(
        parse_link::<VerboseError<&str>>("[[#id][desc]]"),
        Ok((
            "",
            Link {
                path: "#id".into(),
                desc: Some("desc".into())
            }
        ))
    );
    assert!(parse_link::<VerboseError<&str>>("[[#id][desc]").is_err());
}
