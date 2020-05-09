use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take, take_until, take_while1},
    sequence::{delimited, separated_pair},
    IResult,
};

/// Export Snippet Object
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Snippet<'a> {
    /// Back-end name
    pub name: Cow<'a, str>,
    /// Export code
    pub value: Cow<'a, str>,
}

impl Snippet<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, Snippet)> {
        parse_internal(input).ok()
    }

    pub fn into_owned(self) -> Snippet<'static> {
        Snippet {
            name: self.name.into_owned().into(),
            value: self.value.into_owned().into(),
        }
    }
}

#[inline]
fn parse_internal(input: &str) -> IResult<&str, Snippet, ()> {
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
    assert_eq!(
        Snippet::parse("@@html:<b>@@"),
        Some((
            "",
            Snippet {
                name: "html".into(),
                value: "<b>".into()
            }
        ))
    );
    assert_eq!(
        Snippet::parse("@@latex:any arbitrary LaTeX code@@"),
        Some((
            "",
            Snippet {
                name: "latex".into(),
                value: "any arbitrary LaTeX code".into(),
            }
        ))
    );
    assert_eq!(
        Snippet::parse("@@html:@@"),
        Some((
            "",
            Snippet {
                name: "html".into(),
                value: "".into(),
            }
        ))
    );
    assert_eq!(
        Snippet::parse("@@html:<p>@</p>@@"),
        Some((
            "",
            Snippet {
                name: "html".into(),
                value: "<p>@</p>".into(),
            }
        ))
    );

    assert!(Snippet::parse("@@html:<b>@").is_none());
    assert!(Snippet::parse("@@html<b>@@").is_none());
    assert!(Snippet::parse("@@:<b>@@").is_none());
}
