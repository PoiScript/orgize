use std::borrow::Cow;

use crate::parsers::{blank_lines, take_lines_while};

#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct FixedWidth<'a> {
    pub value: Cow<'a, str>,
    pub post_blank: usize,
}

impl FixedWidth<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, FixedWidth<'_>)> {
        let (input, value) = take_lines_while(|line| line == ":" || line.starts_with(": "))(input);
        let (input, blank) = blank_lines(input);

        if value.is_empty() {
            return None;
        }

        Some((
            input,
            FixedWidth {
                value: value.into(),
                post_blank: blank,
            },
        ))
    }

    pub fn into_owned(self) -> FixedWidth<'static> {
        FixedWidth {
            value: self.value.into_owned().into(),
            post_blank: self.post_blank,
        }
    }
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
