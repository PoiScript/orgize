use nom::{
    bytes::complete::{tag, take, take_until, take_while1},
    combinator::{opt, verify},
    IResult,
};

use crate::elements::Element;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Macros<'a> {
    pub name: &'a str,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub arguments: Option<&'a str>,
}

impl Macros<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Element<'_>> {
        let (input, _) = tag("{{{")(input)?;
        let (input, name) = verify(
            take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
            |s: &str| s.starts_with(|c: char| c.is_ascii_alphabetic()),
        )(input)?;
        let (input, arguments) = opt(|input| {
            let (input, _) = tag("(")(input)?;
            let (input, args) = take_until(")}}}")(input)?;
            let (input, _) = take(1usize)(input)?;
            Ok((input, args))
        })(input)?;
        let (input, _) = tag("}}}")(input)?;

        Ok((input, Element::Macros(Macros { name, arguments })))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Macros::parse("{{{poem(red,blue)}}}"),
        Ok((
            "",
            Element::Macros(Macros {
                name: "poem",
                arguments: Some("red,blue")
            },)
        ))
    );
    assert_eq!(
        Macros::parse("{{{poem())}}}"),
        Ok((
            "",
            Element::Macros(Macros {
                name: "poem",
                arguments: Some(")")
            },)
        ))
    );
    assert_eq!(
        Macros::parse("{{{author}}}"),
        Ok((
            "",
            Element::Macros(Macros {
                name: "author",
                arguments: None
            },)
        ))
    );
    assert!(Macros::parse("{{{0uthor}}}").is_err());
    assert!(Macros::parse("{{{author}}").is_err());
    assert!(Macros::parse("{{{poem(}}}").is_err());
    assert!(Macros::parse("{{{poem)}}}").is_err());
}
