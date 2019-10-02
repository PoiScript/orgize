use std::borrow::Cow;

use memchr::memchr2_iter;
use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    error::{ErrorKind, ParseError},
    sequence::preceded,
    Err, IResult,
};

/// Footnote Reference Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct FnRef<'a> {
    /// Footnote label
    pub label: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub definition: Option<Cow<'a, str>>,
}

impl FnRef<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, FnRef<'_>)> {
        parse_fn_ref::<()>(input).ok()
    }

    pub fn into_owned(self) -> FnRef<'static> {
        FnRef {
            label: self.label.into_owned().into(),
            definition: self.definition.map(Into::into).map(Cow::Owned),
        }
    }
}

#[inline]
fn parse_fn_ref<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, FnRef<'a>, E> {
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

fn balanced_brackets<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
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
    Err(Err::Error(E::from_error_kind(input, ErrorKind::Tag)))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_fn_ref::<VerboseError<&str>>("[fn:1]"),
        Ok((
            "",
            FnRef {
                label: "1".into(),
                definition: None
            },
        ))
    );
    assert_eq!(
        parse_fn_ref::<VerboseError<&str>>("[fn:1:2]"),
        Ok((
            "",
            FnRef {
                label: "1".into(),
                definition: Some("2".into())
            },
        ))
    );
    assert_eq!(
        parse_fn_ref::<VerboseError<&str>>("[fn::2]"),
        Ok((
            "",
            FnRef {
                label: "".into(),
                definition: Some("2".into())
            },
        ))
    );
    assert_eq!(
        parse_fn_ref::<VerboseError<&str>>("[fn::[]]"),
        Ok((
            "",
            FnRef {
                label: "".into(),
                definition: Some("[]".into())
            },
        ))
    );

    assert!(parse_fn_ref::<VerboseError<&str>>("[fn::[]").is_err());
}
