use crate::elements::Element;
use crate::parsers::{eol, take_lines_till};

use nom::{
    bytes::complete::{tag, take_while1},
    sequence::delimited,
    IResult,
};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Drawer<'a> {
    pub name: &'a str,
}

impl Drawer<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, (Element<'_>, &str)> {
        let (input, name) = delimited(
            tag(":"),
            take_while1(|c: char| c.is_ascii_alphabetic() || c == '-' || c == '_'),
            tag(":"),
        )(input)?;
        let (input, _) = eol(input)?;
        let (input, contents) = take_lines_till(|line| line.eq_ignore_ascii_case(":END:"))(input)?;

        Ok((input, (Element::Drawer(Drawer { name }), contents)))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Drawer::parse(":PROPERTIES:\n  :CUSTOM_ID: id\n  :END:"),
        Ok((
            "",
            (
                Element::Drawer(Drawer { name: "PROPERTIES" }),
                "  :CUSTOM_ID: id\n"
            )
        ))
    )
}
