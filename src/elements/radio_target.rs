use nom::{
    bytes::complete::{tag, take_while},
    combinator::verify,
    IResult,
};

use crate::elements::Element;

// TODO: text-markup, entities, latex-fragments, subscript and superscript
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct RadioTarget<'a> {
    #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
    contents: &'a str,
}

impl RadioTarget<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Element<'_>> {
        let (input, _) = tag("<<<")(input)?;
        let (input, contents) = verify(
            take_while(|c: char| c != '<' && c != '\n' && c != '>'),
            |s: &str| s.starts_with(|c| c != ' ') && s.ends_with(|c| c != ' '),
        )(input)?;
        let (input, _) = tag(">>>")(input)?;

        Ok((input, Element::RadioTarget(RadioTarget { contents })))
    }
}

#[test]
fn parse() {
    assert_eq!(
        RadioTarget::parse("<<<target>>>"),
        Ok(("", Element::RadioTarget(RadioTarget { contents: "target" })))
    );
    assert_eq!(
        RadioTarget::parse("<<<tar get>>>"),
        Ok((
            "",
            Element::RadioTarget(RadioTarget {
                contents: "tar get"
            },)
        ))
    );
    assert!(RadioTarget::parse("<<<target >>>").is_err());
    assert!(RadioTarget::parse("<<< target>>>").is_err());
    assert!(RadioTarget::parse("<<<ta<get>>>").is_err());
    assert!(RadioTarget::parse("<<<ta>get>>>").is_err());
    assert!(RadioTarget::parse("<<<ta\nget>>>").is_err());
    assert!(RadioTarget::parse("<<<target>>").is_err());
}
