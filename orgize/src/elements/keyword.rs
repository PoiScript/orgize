use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_till},
    combinator::opt,
    error::ParseError,
    sequence::delimited,
    IResult,
};

use crate::parsers::line;

/// Keyword Elemenet
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Keyword<'a> {
    /// Keyword name
    pub key: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub optional: Option<Cow<'a, str>>,
    /// Keyword value
    pub value: Cow<'a, str>,
}

impl Keyword<'_> {
    pub fn into_owned(self) -> Keyword<'static> {
        Keyword {
            key: self.key.into_owned().into(),
            optional: self.optional.map(Into::into).map(Cow::Owned),
            value: self.value.into_owned().into(),
        }
    }
}

/// Babel Call Elemenet
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct BabelCall<'a> {
    pub value: Cow<'a, str>,
}

impl BabelCall<'_> {
    pub fn into_owned(self) -> BabelCall<'static> {
        BabelCall {
            value: self.value.into_owned().into(),
        }
    }
}

#[inline]
pub fn parse_keyword(input: &str) -> Option<(&str, (&str, Option<&str>, &str))> {
    parse_keyword_internal::<()>(input).ok()
}

#[inline]
fn parse_keyword_internal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (&'a str, Option<&'a str>, &'a str), E> {
    let (input, _) = tag("#+")(input)?;
    let (input, key) = take_till(|c: char| c.is_ascii_whitespace() || c == ':' || c == '[')(input)?;
    let (input, optional) = opt(delimited(
        tag("["),
        take_till(|c| c == ']' || c == '\n'),
        tag("]"),
    ))(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, value) = line(input)?;

    Ok((input, (key, optional, value.trim())))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_keyword_internal::<VerboseError<&str>>("#+KEY:"),
        Ok(("", ("KEY", None, "")))
    );
    assert_eq!(
        parse_keyword_internal::<VerboseError<&str>>("#+KEY: VALUE"),
        Ok(("", ("KEY", None, "VALUE")))
    );
    assert_eq!(
        parse_keyword_internal::<VerboseError<&str>>("#+K_E_Y: VALUE"),
        Ok(("", ("K_E_Y", None, "VALUE")))
    );
    assert_eq!(
        parse_keyword_internal::<VerboseError<&str>>("#+KEY:VALUE\n"),
        Ok(("", ("KEY", None, "VALUE")))
    );
    assert!(parse_keyword_internal::<VerboseError<&str>>("#+KE Y: VALUE").is_err());
    assert!(parse_keyword_internal::<VerboseError<&str>>("#+ KEY: VALUE").is_err());

    assert_eq!(
        parse_keyword_internal::<VerboseError<&str>>("#+RESULTS:"),
        Ok(("", ("RESULTS", None, "")))
    );

    assert_eq!(
        parse_keyword_internal::<VerboseError<&str>>("#+ATTR_LATEX: :width 5cm\n"),
        Ok(("", ("ATTR_LATEX", None, ":width 5cm")))
    );

    assert_eq!(
        parse_keyword_internal::<VerboseError<&str>>("#+CALL: double(n=4)"),
        Ok(("", ("CALL", None, "double(n=4)")))
    );

    assert_eq!(
        parse_keyword_internal::<VerboseError<&str>>("#+CAPTION[Short caption]: Longer caption."),
        Ok(("", ("CAPTION", Some("Short caption"), "Longer caption.",)))
    );
}
