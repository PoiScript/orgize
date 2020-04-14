use std::borrow::Cow;

use nom::{
    error::{ErrorKind, ParseError},
    Err, IResult,
};

use crate::parse::combinators::{blank_lines_count, lines_while};

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
    pub(crate) fn parse(input: &str) -> Option<(&str, Comment)> {
        Self::parse_internal::<()>(input).ok()
    }

    fn parse_internal<'a, E>(input: &'a str) -> IResult<&str, Comment, E>
    where
        E: ParseError<&'a str>,
    {
        let (input, value) = lines_while(|line| {
            let line = line.trim_start();
            line == "#" || line.starts_with("# ")
        })(input)?;

        if value.is_empty() {
            // TODO: better error kind
            return Err(Err::Error(E::from_error_kind(input, ErrorKind::Many0)));
        }

        let (input, post_blank) = blank_lines_count(input)?;

        Ok((
            input,
            Comment {
                value: value.into(),
                post_blank,
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
