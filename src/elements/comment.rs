use std::borrow::Cow;

use crate::parsers::{blank_lines, take_lines_while};

#[derive(Debug, Default)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct Comment<'a> {
    /// Comments value, with pound signs
    pub value: Cow<'a, str>,
    /// Numbers of blank lines between last comment's line and next non-blank
    /// line or buffer's end
    pub post_blank: usize,
}

impl Comment<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, Comment<'_>)> {
        let (input, value) = take_lines_while(|line| line == "#" || line.starts_with("# "))(input);
        let (input, blank) = blank_lines(input);

        if value.is_empty() {
            return None;
        }

        Some((
            input,
            Comment {
                value: value.into(),
                post_blank: blank,
            },
        ))
    }

    pub fn into_owned(self) -> Comment<'static> {
        Comment {
            value: self.value.into_owned().into(),
            post_blank: self.post_blank,
        }
    }
}
