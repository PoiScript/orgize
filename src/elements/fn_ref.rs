use memchr::memchr2_iter;
use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    error::ErrorKind,
    error_position,
    sequence::preceded,
    Err, IResult,
};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct FnRef<'a> {
    pub label: &'a str,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub definition: Option<&'a str>,
}

fn balanced_brackets(input: &str) -> IResult<&str, &str> {
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
    Err(Err::Error(error_position!(input, ErrorKind::Tag)))
}

impl FnRef<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, FnRef<'_>> {
        let (input, _) = tag("[fn:")(input)?;
        let (input, label) =
            take_while(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_')(input)?;
        let (input, definition) = opt(preceded(tag(":"), balanced_brackets))(input)?;
        let (input, _) = tag("]")(input)?;

        Ok((input, FnRef { label, definition }))
    }
}

#[test]
fn parse() {
    assert_eq!(
        FnRef::parse("[fn:1]"),
        Ok((
            "",
            FnRef {
                label: "1",
                definition: None
            },
        ))
    );
    assert_eq!(
        FnRef::parse("[fn:1:2]"),
        Ok((
            "",
            FnRef {
                label: "1",
                definition: Some("2")
            },
        ))
    );
    assert_eq!(
        FnRef::parse("[fn::2]"),
        Ok((
            "",
            FnRef {
                label: "",
                definition: Some("2")
            },
        ))
    );
    assert_eq!(
        FnRef::parse("[fn::[]]"),
        Ok((
            "",
            FnRef {
                label: "",
                definition: Some("[]")
            },
        ))
    );

    assert!(FnRef::parse("[fn::[]").is_err());
}
