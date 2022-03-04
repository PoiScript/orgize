use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    sequence::delimited,
    IResult,
};

/// Link Object
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Link<'a> {
    /// Link destination
    pub path: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub desc: Option<Cow<'a, str>>,
}

impl Link<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> Option<(&str, Link)> {
        parse_internal(input).ok()
    }

    pub fn into_owned(self) -> Link<'static> {
        Link {
            path: self.path.into_owned().into(),
            desc: self.desc.map(Into::into).map(Cow::Owned),
        }
    }
}

#[inline]
fn parse_internal(input: &str) -> IResult<&str, Link, ()> {
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
    assert_eq!(
        Link::parse("[[#id]]"),
        Some((
            "",
            Link {
                path: "#id".into(),
                desc: None
            }
        ))
    );
    assert_eq!(
        Link::parse("[[#id][desc]]"),
        Some((
            "",
            Link {
                path: "#id".into(),
                desc: Some("desc".into())
            }
        ))
    );
    assert!(Link::parse("[[#id][desc]").is_none());
}
