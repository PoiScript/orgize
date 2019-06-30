use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    IResult,
};

use crate::elements::Element;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Link<'a> {
    pub path: &'a str,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub desc: Option<&'a str>,
}

impl Link<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Element<'_>> {
        let (input, _) = tag("[[")(input)?;
        let (input, path) =
            take_while(|c: char| c != '<' && c != '>' && c != '\n' && c != ']')(input)?;
        let (input, _) = tag("]")(input)?;
        let (input, desc) = opt(|input| {
            let (input, _) = tag("[")(input)?;
            let (input, desc) = take_while(|c: char| c != '[' && c != ']')(input)?;
            let (input, _) = tag("]")(input)?;
            Ok((input, desc))
        })(input)?;
        let (input, _) = tag("]")(input)?;
        Ok((input, Element::Link(Link { path, desc })))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Link::parse("[[#id]]"),
        Ok((
            "",
            Element::Link(Link {
                path: "#id",
                desc: None
            },)
        ))
    );
    assert_eq!(
        Link::parse("[[#id][desc]]"),
        Ok((
            "",
            Element::Link(Link {
                path: "#id",
                desc: Some("desc")
            })
        ))
    );
    assert!(Link::parse("[[#id][desc]").is_err());
}
