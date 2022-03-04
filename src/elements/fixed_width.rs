use std::borrow::Cow;

use nom::{
    error::{make_error, ErrorKind},
    Err, IResult,
};

use crate::parse::combinators::{blank_lines_count, lines_while};

#[derive(Debug, Default, PartialEq, Clone)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct FixedWidth<'a> {
    /// Fixed width value
    pub value: Cow<'a, str>,
    /// Numbers of blank lines between last fixed width's line and next
    /// non-blank line or buffer's end
    pub post_blank: usize,
}

impl FixedWidth<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, FixedWidth)> {
        parse_internal(input).ok()
    }

    pub fn into_owned(self) -> FixedWidth<'static> {
        FixedWidth {
            value: self.value.into_owned().into(),
            post_blank: self.post_blank,
        }
    }
}

fn parse_internal(input: &str) -> IResult<&str, FixedWidth, ()> {
    let (input, value) = lines_while(|line| {
        let line = line.trim_start();
        line == ":" || line.starts_with(": ")
    })(input)?;

    if value.is_empty() {
        // TODO: better error kind
        return Err(Err::Error(make_error(input, ErrorKind::Many0)));
    }

    let (input, post_blank) = blank_lines_count(input)?;

    Ok((
        input,
        FixedWidth {
            value: value.into(),
            post_blank,
        },
    ))
}

#[test]
fn parse() {
    assert_eq!(
        FixedWidth::parse(
            r#": A
:
: B
: C

"#
        ),
        Some((
            "",
            FixedWidth {
                value: r#": A
:
: B
: C
"#
                .into(),
                post_blank: 1
            }
        ))
    );
}
