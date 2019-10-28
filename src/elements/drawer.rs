use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while1},
    error::ParseError,
    sequence::delimited,
    IResult,
};

use crate::parsers::{blank_lines, eol, line, take_lines_while};

/// Drawer Element
#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct Drawer<'a> {
    /// Drawer name
    pub name: Cow<'a, str>,
    /// Numbers of blank lines
    pub pre_blank: usize,
    /// Numbers of blank lines
    pub post_blank: usize,
}

impl Drawer<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, (Drawer, &str))> {
        parse_drawer::<()>(input).ok()
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
pub fn parse_drawer<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, (Drawer, &str), E> {
    let (input, (mut drawer, content)) = parse_drawer_without_blank(input)?;

    let (content, blank) = blank_lines(content);
    drawer.pre_blank = blank;

    let (input, blank) = blank_lines(input);
    drawer.post_blank = blank;

    Ok((input, (drawer, content)))
}

pub fn parse_drawer_without_blank<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, (Drawer, &str), E> {
    let (input, name) = delimited(
        tag(":"),
        take_while1(|c: char| c.is_ascii_alphabetic() || c == '-' || c == '_'),
        tag(":"),
    )(input)?;
    let (input, _) = eol(input)?;
    let (input, contents) =
        take_lines_while(|line| !line.trim().eq_ignore_ascii_case(":END:"))(input);
    let (input, _) = line(input)?;

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
    use nom::error::VerboseError;

    assert_eq!(
        parse_drawer::<VerboseError<&str>>(
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
        parse_drawer::<VerboseError<&str>>(
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
}
