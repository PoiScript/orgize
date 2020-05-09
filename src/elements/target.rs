use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while},
    combinator::verify,
    sequence::delimited,
    IResult,
};

/// Target Object
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Target<'a> {
    /// Target ID
    pub target: Cow<'a, str>,
}

impl Target<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> Option<(&str, Target)> {
        parse_internal(input).ok()
    }

    pub fn into_owned(self) -> Target<'static> {
        Target {
            target: self.target.into_owned().into(),
        }
    }
}

#[inline]
fn parse_internal(input: &str) -> IResult<&str, Target, ()> {
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
    assert_eq!(
        Target::parse("<<target>>"),
        Some((
            "",
            Target {
                target: "target".into()
            }
        ))
    );
    assert_eq!(
        Target::parse("<<tar get>>"),
        Some((
            "",
            Target {
                target: "tar get".into()
            }
        ))
    );

    assert!(Target::parse("<<target >>").is_none());
    assert!(Target::parse("<< target>>").is_none());
    assert!(Target::parse("<<ta<get>>").is_none());
    assert!(Target::parse("<<ta>get>>").is_none());
    assert!(Target::parse("<<ta\nget>>").is_none());
    assert!(Target::parse("<<target>").is_none());
}
