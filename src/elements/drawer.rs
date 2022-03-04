use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    sequence::delimited,
    IResult,
};

use crate::parse::combinators::{blank_lines_count, eol, lines_till};

/// Drawer Element
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct Drawer<'a> {
    /// Drawer name
    pub name: Cow<'a, str>,
    /// Numbers of blank lines between first drawer's line and next non-blank
    /// line
    pub pre_blank: usize,
    /// Numbers of blank lines between last drawer's line and next non-blank
    /// line or buffer's end
    pub post_blank: usize,
}

impl Drawer<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, (Drawer, &str))> {
        parse_drawer(input).ok()
    }

    pub fn into_owned(self) -> Drawer<'static> {
        Drawer {
            name: self.name.into_owned().into(),
            pre_blank: self.pre_blank,
            post_blank: self.post_blank,
        }
    }
}

#[inline]
pub fn parse_drawer(input: &str) -> IResult<&str, (Drawer, &str), ()> {
    let (input, (mut drawer, content)) = parse_drawer_without_blank(input)?;

    let (content, blank) = blank_lines_count(content)?;
    drawer.pre_blank = blank;

    let (input, blank) = blank_lines_count(input)?;
    drawer.post_blank = blank;

    Ok((input, (drawer, content)))
}

pub fn parse_drawer_without_blank(input: &str) -> IResult<&str, (Drawer, &str), ()> {
    let (input, _) = space0(input)?;
    let (input, name) = delimited(
        tag(":"),
        take_while1(|c: char| c.is_ascii_alphabetic() || c == '-' || c == '_'),
        tag(":"),
    )(input)?;
    let (input, _) = eol(input)?;
    let (input, contents) = lines_till(|line| line.trim().eq_ignore_ascii_case(":END:"))(input)?;

    Ok((
        input,
        (
            Drawer {
                name: name.into(),
                pre_blank: 0,
                post_blank: 0,
            },
            contents,
        ),
    ))
}

#[test]
fn parse() {
    assert_eq!(
        parse_drawer(
            r#":PROPERTIES:
  :CUSTOM_ID: id
  :END:"#
        ),
        Ok((
            "",
            (
                Drawer {
                    name: "PROPERTIES".into(),
                    pre_blank: 0,
                    post_blank: 0
                },
                "  :CUSTOM_ID: id\n"
            )
        ))
    );
    assert_eq!(
        parse_drawer(
            r#":PROPERTIES:


  :END:

"#
        ),
        Ok((
            "",
            (
                Drawer {
                    name: "PROPERTIES".into(),
                    pre_blank: 2,
                    post_blank: 1,
                },
                ""
            )
        ))
    );

    // https://github.com/PoiScript/orgize/issues/9
    assert!(parse_drawer(":SPAGHETTI:\n").is_err());
}
