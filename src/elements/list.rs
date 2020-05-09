use std::borrow::Cow;
use std::iter::once;

use memchr::{memchr, memchr_iter};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, recognize},
    sequence::terminated,
    IResult,
};

/// Plain List Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct List {
    /// List indent, number of whitespaces
    pub indent: usize,
    /// List's type, determined by the first item of this list
    pub ordered: bool,
    /// Numbers of blank lines between last list's line and next non-blank line
    /// or buffer's end
    pub post_blank: usize,
}

/// List Item Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct ListItem<'a> {
    /// List item bullet
    pub bullet: Cow<'a, str>,
    /// List item indent, number of whitespaces
    pub indent: usize,
    /// List item type
    pub ordered: bool,
    // TODO checkbox
    // TODO counter
    // TODO tag
}

impl ListItem<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> Option<(&str, (ListItem, &str))> {
        list_item(input).ok()
    }

    pub fn into_owned(self) -> ListItem<'static> {
        ListItem {
            bullet: self.bullet.into_owned().into(),
            indent: self.indent,
            ordered: self.ordered,
        }
    }
}

fn list_item(input: &str) -> IResult<&str, (ListItem, &str), ()> {
    let (input, indent) = map(space0, |s: &str| s.len())(input)?;
    let (input, bullet) = recognize(alt((
        tag("+ "),
        tag("* "),
        tag("- "),
        terminated(digit1, tag(". ")),
    )))(input)?;
    let (input, contents) = list_item_contents(input, indent);
    Ok((
        input,
        (
            ListItem {
                bullet: bullet.into(),
                indent,
                ordered: bullet.starts_with(|c: char| c.is_ascii_digit()),
            },
            contents,
        ),
    ))
}

fn list_item_contents(input: &str, indent: usize) -> (&str, &str) {
    let mut last_end = memchr(b'\n', input.as_bytes())
        .map(|i| i + 1)
        .unwrap_or_else(|| input.len());

    for i in memchr_iter(b'\n', input.as_bytes())
        .map(|i| i + 1)
        .chain(once(input.len()))
        .skip(1)
    {
        if input[last_end..i]
            .as_bytes()
            .iter()
            .all(u8::is_ascii_whitespace)
        {
            let x = memchr(b'\n', &input[i..].as_bytes())
                .map(|ii| i + ii + 1)
                .unwrap_or_else(|| input.len());

            // two consecutive empty lines
            if input[i..x].as_bytes().iter().all(u8::is_ascii_whitespace) {
                return (&input[x..], &input[0..x]);
            }
        }

        // line less or equally indented than the starting line
        if input[last_end..i]
            .as_bytes()
            .iter()
            .take(indent + 1)
            .any(|c| !c.is_ascii_whitespace())
        {
            return (&input[last_end..], &input[0..last_end]);
        }

        last_end = i;
    }

    ("", input)
}

#[test]
fn parse() {
    assert_eq!(
        list_item(
            r#"+ item1
+ item2"#
        ),
        Ok((
            "+ item2",
            (
                ListItem {
                    bullet: "+ ".into(),
                    indent: 0,
                    ordered: false,
                },
                r#"item1
"#
            )
        ))
    );
    assert_eq!(
        list_item(
            r#"* item1

* item2"#
        ),
        Ok((
            "* item2",
            (
                ListItem {
                    bullet: "* ".into(),
                    indent: 0,
                    ordered: false,
                },
                r#"item1

"#
            )
        ))
    );
    assert_eq!(
        list_item(
            r#"* item1


* item2"#
        ),
        Ok((
            "* item2",
            (
                ListItem {
                    bullet: "* ".into(),
                    indent: 0,
                    ordered: false,
                },
                r#"item1


"#
            )
        ))
    );
    assert_eq!(
        list_item(
            r#"* item1

"#
        ),
        Ok((
            "",
            (
                ListItem {
                    bullet: "* ".into(),
                    indent: 0,
                    ordered: false,
                },
                r#"item1

"#
            )
        ))
    );
    assert_eq!(
        list_item(
            r#"+ item1
  + item2
"#
        ),
        Ok((
            "",
            (
                ListItem {
                    bullet: "+ ".into(),
                    indent: 0,
                    ordered: false,
                },
                r#"item1
  + item2
"#
            )
        ))
    );
    assert_eq!(
        list_item(
            r#"+ item1

  + item2

+ item 3"#
        ),
        Ok((
            "+ item 3",
            (
                ListItem {
                    bullet: "+ ".into(),
                    indent: 0,
                    ordered: false,
                },
                r#"item1

  + item2

"#
            )
        ))
    );
    assert_eq!(
        list_item(
            r#"  + item1

  + item2"#
        ),
        Ok((
            "  + item2",
            (
                ListItem {
                    bullet: "+ ".into(),
                    indent: 2,
                    ordered: false,
                },
                r#"item1

"#
            )
        ))
    );
    assert_eq!(
        list_item(
            r#"  1. item1
2. item2
  3. item3"#
        ),
        Ok((
            r#"2. item2
  3. item3"#,
            (
                ListItem {
                    bullet: "1. ".into(),
                    indent: 2,
                    ordered: true,
                },
                r#"item1
"#
            )
        ))
    );
    assert_eq!(
        list_item(
            r#"+ 1

  - 2

  - 3

+ 4"#
        ),
        Ok((
            "+ 4",
            (
                ListItem {
                    bullet: "+ ".into(),
                    indent: 0,
                    ordered: false,
                },
                r#"1

  - 2

  - 3

"#
            )
        ))
    );
}
