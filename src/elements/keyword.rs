use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_till},
    combinator::opt,
    sequence::delimited,
    IResult,
};

use crate::parsers::line;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Keyword<'a> {
    pub key: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub optional: Option<Cow<'a, str>>,
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

pub(crate) fn parse_keyword(input: &str) -> IResult<&str, (&str, Option<&str>, &str)> {
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
    assert_eq!(parse_keyword("#+KEY:"), Ok(("", ("KEY", None, ""))));
    assert_eq!(
        parse_keyword("#+KEY: VALUE"),
        Ok(("", ("KEY", None, "VALUE")))
    );
    assert_eq!(
        parse_keyword("#+K_E_Y: VALUE"),
        Ok(("", ("K_E_Y", None, "VALUE")))
    );
    assert_eq!(
        parse_keyword("#+KEY:VALUE\n"),
        Ok(("", ("KEY", None, "VALUE")))
    );
    assert!(parse_keyword("#+KE Y: VALUE").is_err());
    assert!(parse_keyword("#+ KEY: VALUE").is_err());

    assert_eq!(parse_keyword("#+RESULTS:"), Ok(("", ("RESULTS", None, ""))));

    assert_eq!(
        parse_keyword("#+ATTR_LATEX: :width 5cm\n"),
        Ok(("", ("ATTR_LATEX", None, ":width 5cm")))
    );

    assert_eq!(
        parse_keyword("#+CALL: double(n=4)"),
        Ok(("", ("CALL", None, "double(n=4)")))
    );

    assert_eq!(
        parse_keyword("#+CAPTION[Short caption]: Longer caption."),
        Ok(("", ("CAPTION", Some("Short caption"), "Longer caption.",)))
    );
}
