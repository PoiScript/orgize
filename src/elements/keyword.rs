use nom::{
    bytes::complete::{tag, take_till},
    combinator::opt,
    sequence::delimited,
    IResult,
};

use crate::elements::Element;
use crate::parsers::take_until_eol;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Keyword<'a> {
    pub key: &'a str,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub optional: Option<&'a str>,
    pub value: &'a str,
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct BabelCall<'a> {
    pub value: &'a str,
}

impl Keyword<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Element<'_>> {
        let (input, _) = tag("#+")(input)?;
        let (input, key) =
            take_till(|c: char| c.is_ascii_whitespace() || c == ':' || c == '[')(input)?;
        let (input, optional) = opt(delimited(
            tag("["),
            take_till(|c| c == ']' || c == '\n'),
            tag("]"),
        ))(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, value) = take_until_eol(input)?;

        if key.eq_ignore_ascii_case("CALL") {
            Ok((input, Element::BabelCall(BabelCall { value })))
        } else {
            Ok((
                input,
                Element::Keyword(Keyword {
                    key,
                    optional,
                    value,
                }),
            ))
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Keyword::parse("#+KEY:"),
        Ok((
            "",
            Element::Keyword(Keyword {
                key: "KEY",
                optional: None,
                value: "",
            })
        ))
    );
    assert_eq!(
        Keyword::parse("#+KEY: VALUE"),
        Ok((
            "",
            Element::Keyword(Keyword {
                key: "KEY",
                optional: None,
                value: "VALUE",
            })
        ))
    );
    assert_eq!(
        Keyword::parse("#+K_E_Y: VALUE"),
        Ok((
            "",
            Element::Keyword(Keyword {
                key: "K_E_Y",
                optional: None,
                value: "VALUE",
            })
        ))
    );
    assert_eq!(
        Keyword::parse("#+KEY:VALUE\n"),
        Ok((
            "",
            Element::Keyword(Keyword {
                key: "KEY",
                optional: None,
                value: "VALUE",
            })
        ))
    );
    assert!(Keyword::parse("#+KE Y: VALUE").is_err());
    assert!(Keyword::parse("#+ KEY: VALUE").is_err());

    assert_eq!(
        Keyword::parse("#+RESULTS:"),
        Ok((
            "",
            Element::Keyword(Keyword {
                key: "RESULTS",
                optional: None,
                value: "",
            })
        ))
    );

    assert_eq!(
        Keyword::parse("#+ATTR_LATEX: :width 5cm\n"),
        Ok((
            "",
            Element::Keyword(Keyword {
                key: "ATTR_LATEX",
                optional: None,
                value: ":width 5cm",
            })
        ))
    );

    assert_eq!(
        Keyword::parse("#+CALL: double(n=4)"),
        Ok((
            "",
            Element::BabelCall(BabelCall {
                value: "double(n=4)",
            })
        ))
    );

    assert_eq!(
        Keyword::parse("#+CAPTION[Short caption]: Longer caption."),
        Ok((
            "",
            Element::Keyword(Keyword {
                key: "CAPTION",
                optional: Some("Short caption"),
                value: "Longer caption.",
            })
        ))
    );
}
