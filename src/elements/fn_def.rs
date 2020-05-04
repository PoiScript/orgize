use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while1},
    error::ParseError,
    sequence::delimited,
    IResult,
};

use crate::parse::combinators::{blank_lines_count, line};

/// Footnote Definition Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Default, Clone)]
pub struct FnDef<'a> {
    /// Footnote label, used for reference
    pub label: Cow<'a, str>,
    /// Numbers of blank lines between last footnote definition's line and next
    /// non-blank line or buffer's end
    pub post_blank: usize,
}

impl FnDef<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, (FnDef, &str))> {
        Self::parse_internal::<()>(input).ok()
    }

    fn parse_internal<'a, E>(input: &'a str) -> IResult<&str, (FnDef, &str), E>
    where
        E: ParseError<&'a str>,
    {
        let (input, label) = delimited(
            tag("[fn:"),
            take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
            tag("]"),
        )(input)?;

        let (input, content) = line(input)?;

        let (input, post_blank) = blank_lines_count(input)?;

        Ok((
            input,
            (
                FnDef {
                    label: label.into(),
                    post_blank,
                },
                content,
            ),
        ))
    }

    pub fn into_owned(self) -> FnDef<'static> {
        FnDef {
            label: self.label.into_owned().into(),
            post_blank: self.post_blank,
        }
    }
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        FnDef::parse_internal::<VerboseError<&str>>("[fn:1] https://orgmode.org"),
        Ok((
            "",
            (
                FnDef {
                    label: "1".into(),
                    post_blank: 0
                },
                " https://orgmode.org"
            )
        ))
    );
    assert_eq!(
        FnDef::parse_internal::<VerboseError<&str>>("[fn:word_1] https://orgmode.org"),
        Ok((
            "",
            (
                FnDef {
                    label: "word_1".into(),
                    post_blank: 0,
                },
                " https://orgmode.org"
            )
        ))
    );
    assert_eq!(
        FnDef::parse_internal::<VerboseError<&str>>("[fn:WORD-1] https://orgmode.org"),
        Ok((
            "",
            (
                FnDef {
                    label: "WORD-1".into(),
                    post_blank: 0,
                },
                " https://orgmode.org"
            )
        ))
    );
    assert_eq!(
        FnDef::parse_internal::<VerboseError<&str>>("[fn:WORD]"),
        Ok((
            "",
            (
                FnDef {
                    label: "WORD".into(),
                    post_blank: 0,
                },
                ""
            )
        ))
    );

    assert!(FnDef::parse_internal::<VerboseError<&str>>("[fn:] https://orgmode.org").is_err());
    assert!(FnDef::parse_internal::<VerboseError<&str>>("[fn:wor d] https://orgmode.org").is_err());
    assert!(FnDef::parse_internal::<VerboseError<&str>>("[fn:WORD https://orgmode.org").is_err());
}
