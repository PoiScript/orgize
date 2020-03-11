use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while1},
    error::ParseError,
    sequence::delimited,
    IResult,
};

use crate::parsers::{blank_lines, line};

/// Footnote Definition Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Default)]
pub struct FnDef<'a> {
    /// Footnote label, used for reference
    pub label: Cow<'a, str>,
    /// Numbers of blank lines between last footnote definition's line and next
    /// non-blank line or buffer's end
    pub post_blank: usize,
}

impl FnDef<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, (FnDef, &str))> {
        parse_fn_def::<()>(input).ok()
    }

    pub fn into_owned(self) -> FnDef<'static> {
        FnDef {
            label: self.label.into_owned().into(),
            post_blank: self.post_blank,
        }
    }
}

#[inline]
fn parse_fn_def<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, (FnDef, &str), E> {
    let (input, label) = delimited(
        tag("[fn:"),
        take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
        tag("]"),
    )(input)?;
    let (input, content) = line(input)?;
    let (input, blank) = blank_lines(input);

    Ok((
        input,
        (
            FnDef {
                label: label.into(),
                post_blank: blank,
            },
            content,
        ),
    ))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_fn_def::<VerboseError<&str>>("[fn:1] https://orgmode.org"),
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
        parse_fn_def::<VerboseError<&str>>("[fn:word_1] https://orgmode.org"),
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
        parse_fn_def::<VerboseError<&str>>("[fn:WORD-1] https://orgmode.org"),
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
        parse_fn_def::<VerboseError<&str>>("[fn:WORD]"),
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

    assert!(parse_fn_def::<VerboseError<&str>>("[fn:] https://orgmode.org").is_err());
    assert!(parse_fn_def::<VerboseError<&str>>("[fn:wor d] https://orgmode.org").is_err());
    assert!(parse_fn_def::<VerboseError<&str>>("[fn:WORD https://orgmode.org").is_err());
}
