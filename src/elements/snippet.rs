use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take, take_until, take_while1},
    error::ParseError,
    sequence::{delimited, separated_pair},
    IResult,
};

/// Export Snippet Object
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Snippet<'a> {
    /// Back-end name
    pub name: Cow<'a, str>,
    /// Export code
    pub value: Cow<'a, str>,
}

impl Snippet<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, Snippet)> {
        parse_snippet::<()>(input).ok()
    }

    pub fn into_owned(self) -> Snippet<'static> {
        Snippet {
            name: self.name.into_owned().into(),
            value: self.value.into_owned().into(),
        }
    }
}

#[inline]
fn parse_snippet<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Snippet, E> {
    let (input, (name, value)) = delimited(
        tag("@@"),
        separated_pair(
            take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-'),
            tag(":"),
            take_until("@@"),
        ),
        take(2usize),
    )(input)?;

    Ok((
        input,
        Snippet {
            name: name.into(),
            value: value.into(),
        },
    ))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_snippet::<VerboseError<&str>>("@@html:<b>@@"),
        Ok((
            "",
            Snippet {
                name: "html".into(),
                value: "<b>".into()
            }
        ))
    );
    assert_eq!(
        parse_snippet::<VerboseError<&str>>("@@latex:any arbitrary LaTeX code@@"),
        Ok((
            "",
            Snippet {
                name: "latex".into(),
                value: "any arbitrary LaTeX code".into(),
            }
        ))
    );
    assert_eq!(
        parse_snippet::<VerboseError<&str>>("@@html:@@"),
        Ok((
            "",
            Snippet {
                name: "html".into(),
                value: "".into(),
            }
        ))
    );
    assert_eq!(
        parse_snippet::<VerboseError<&str>>("@@html:<p>@</p>@@"),
        Ok((
            "",
            Snippet {
                name: "html".into(),
                value: "<p>@</p>".into(),
            }
        ))
    );
    assert!(parse_snippet::<VerboseError<&str>>("@@html:<b>@").is_err());
    assert!(parse_snippet::<VerboseError<&str>>("@@html<b>@@").is_err());
    assert!(parse_snippet::<VerboseError<&str>>("@@:<b>@@").is_err());
}
