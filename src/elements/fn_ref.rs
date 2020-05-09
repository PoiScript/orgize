use std::borrow::Cow;

use memchr::memchr2_iter;
use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    error::{make_error, ErrorKind},
    sequence::preceded,
    Err, IResult,
};

/// Footnote Reference Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct FnRef<'a> {
    /// Footnote label
    pub label: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub definition: Option<Cow<'a, str>>,
}

impl FnRef<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, FnRef)> {
        parse_internal(input).ok()
    }

    pub fn into_owned(self) -> FnRef<'static> {
        FnRef {
            label: self.label.into_owned().into(),
            definition: self.definition.map(Into::into).map(Cow::Owned),
        }
    }
}

#[inline]
fn parse_internal(input: &str) -> IResult<&str, FnRef, ()> {
    let (input, _) = tag("[fn:")(input)?;
    let (input, label) =
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_')(input)?;
    let (input, definition) = opt(preceded(tag(":"), balanced_brackets))(input)?;
    let (input, _) = tag("]")(input)?;

    Ok((
        input,
        FnRef {
            label: label.into(),
            definition: definition.map(Into::into),
        },
    ))
}

fn balanced_brackets(input: &str) -> IResult<&str, &str, ()> {
    let mut pairs = 1;
    for i in memchr2_iter(b'[', b']', input.as_bytes()) {
        if input.as_bytes()[i] == b'[' {
            pairs += 1;
        } else if pairs != 1 {
            pairs -= 1;
        } else {
            return Ok((&input[i..], &input[0..i]));
        }
    }
    Err(Err::Error(make_error(input, ErrorKind::Tag)))
}

#[test]
fn parse() {
    assert_eq!(
        FnRef::parse("[fn:1]"),
        Some((
            "",
            FnRef {
                label: "1".into(),
                definition: None
            },
        ))
    );
    assert_eq!(
        FnRef::parse("[fn:1:2]"),
        Some((
            "",
            FnRef {
                label: "1".into(),
                definition: Some("2".into())
            },
        ))
    );
    assert_eq!(
        FnRef::parse("[fn::2]"),
        Some((
            "",
            FnRef {
                label: "".into(),
                definition: Some("2".into())
            },
        ))
    );
    assert_eq!(
        FnRef::parse("[fn::[]]"),
        Some((
            "",
            FnRef {
                label: "".into(),
                definition: Some("[]".into())
            },
        ))
    );

    assert!(FnRef::parse("[fn::[]").is_none());
}
