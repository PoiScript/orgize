use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while1},
    sequence::delimited,
    IResult,
};

use crate::parsers::line;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct FnDef<'a> {
    pub label: Cow<'a, str>,
}

impl FnDef<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, (FnDef<'_>, &str)> {
        let (input, label) = delimited(
            tag("[fn:"),
            take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
            tag("]"),
        )(input)?;
        let (input, content) = line(input)?;

        Ok((
            input,
            (
                FnDef {
                    label: label.into(),
                },
                content,
            ),
        ))
    }

    pub fn into_owned(self) -> FnDef<'static> {
        FnDef {
            label: self.label.into_owned().into(),
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        FnDef::parse("[fn:1] https://orgmode.org"),
        Ok(("", (FnDef { label: "1".into() }, " https://orgmode.org")))
    );
    assert_eq!(
        FnDef::parse("[fn:word_1] https://orgmode.org"),
        Ok((
            "",
            (
                FnDef {
                    label: "word_1".into()
                },
                " https://orgmode.org"
            )
        ))
    );
    assert_eq!(
        FnDef::parse("[fn:WORD-1] https://orgmode.org"),
        Ok((
            "",
            (
                FnDef {
                    label: "WORD-1".into()
                },
                " https://orgmode.org"
            )
        ))
    );
    assert_eq!(
        FnDef::parse("[fn:WORD]"),
        Ok((
            "",
            (
                FnDef {
                    label: "WORD".into()
                },
                ""
            )
        ))
    );

    assert!(FnDef::parse("[fn:] https://orgmode.org").is_err());
    assert!(FnDef::parse("[fn:wor d] https://orgmode.org").is_err());
    assert!(FnDef::parse("[fn:WORD https://orgmode.org").is_err());
}
