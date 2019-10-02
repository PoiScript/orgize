use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while1},
    error::ParseError,
    sequence::delimited,
    IResult,
};

use crate::parsers::{eol, line, take_lines_while};

/// Drawer Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Drawer<'a> {
    /// Drawer name
    pub name: Cow<'a, str>,
}

impl Drawer<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, (Drawer<'_>, &str))> {
        parse_drawer::<()>(input).ok()
    }

    pub fn into_owned(self) -> Drawer<'static> {
        Drawer {
            name: self.name.into_owned().into(),
        }
    }
}

#[inline]
pub fn parse_drawer<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (Drawer<'a>, &'a str), E> {
    let (input, name) = delimited(
        tag(":"),
        take_while1(|c: char| c.is_ascii_alphabetic() || c == '-' || c == '_'),
        tag(":"),
    )(input)?;
    let (input, _) = eol(input)?;
    let (input, contents) =
        take_lines_while(|line| !line.trim().eq_ignore_ascii_case(":END:"))(input);
    let (input, _) = line(input)?;

    Ok((input, (Drawer { name: name.into() }, contents)))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_drawer::<VerboseError<&str>>(":PROPERTIES:\n  :CUSTOM_ID: id\n  :END:"),
        Ok((
            "",
            (
                Drawer {
                    name: "PROPERTIES".into()
                },
                "  :CUSTOM_ID: id\n"
            )
        ))
    )
}
