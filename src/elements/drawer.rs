use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    sequence::delimited,
    IResult,
};

use crate::{
    parse::combinators::{blank_lines_count, eol, lines_till},
    parsers::lines_until_headline_at_level_le,
};

/// Drawer Element
#[derive(Debug, Default, Clone)]
#[cfg_attr(test, derive(PartialEq))]
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

    // Restrict the search for the end of the drawer to the current headline.
    let (_input_after_headline, (input_until_headline, _level)) =
        lines_until_headline_at_level_le(input, std::usize::MAX)?;

    // tail is the remaining not used for the drawer out of
    // input_until_headline.
    let (tail, contents) =
        lines_till(|line| line.trim().eq_ignore_ascii_case(":END:"))(input_until_headline)?;

    // Skip over the amount used by the drawer.
    let input = &input[input_until_headline.len() - tail.len()..];

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

    // https://github.com/PoiScript/orgize/issues/24
    // A drawer may not contain a headline.
    assert!(parse_drawer(
        r#":MYDRAWER:
* Node
  :END:"#
    )
    .is_err(),);

    // A drawer may not contain another drawer. An attempt to do so will result
    // in the drawer ending at the first end line.
    assert_eq!(
        parse_drawer(":OUTER:\nOuter Text\n:INNER:\nInner Text\n:END:\n:END:"),
        Ok((
            ":END:",
            (
                Drawer {
                    name: "OUTER".into(),
                    pre_blank: 0,
                    post_blank: 0
                },
                "Outer Text\n:INNER:\nInner Text\n"
            )
        ))
    );
}
