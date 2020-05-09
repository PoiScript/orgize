use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while1},
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
        parse_internal(input).ok()
    }

    pub fn into_owned(self) -> FnDef<'static> {
        FnDef {
            label: self.label.into_owned().into(),
            post_blank: self.post_blank,
        }
    }
}

fn parse_internal(input: &str) -> IResult<&str, (FnDef, &str), ()> {
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

#[test]
fn parse() {
    assert_eq!(
        FnDef::parse("[fn:1] https://orgmode.org"),
        Some((
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
        FnDef::parse("[fn:word_1] https://orgmode.org"),
        Some((
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
        FnDef::parse("[fn:WORD-1] https://orgmode.org"),
        Some((
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
        FnDef::parse("[fn:WORD]"),
        Some((
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

    assert!(FnDef::parse("[fn:] https://orgmode.org").is_none());
    assert!(FnDef::parse("[fn:wor d] https://orgmode.org").is_none());
    assert!(FnDef::parse("[fn:WORD https://orgmode.org").is_none());
}
