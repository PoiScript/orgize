use crate::elements::Element;
use crate::parsers::{take_lines_till, take_until_eol};

use nom::{
    bytes::complete::tag_no_case,
    character::complete::{alpha1, space1},
    IResult,
};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct DynBlock<'a> {
    pub block_name: &'a str,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub arguments: Option<&'a str>,
}

impl DynBlock<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, (Element<'_>, &str)> {
        let (input, _) = tag_no_case("#+BEGIN:")(input)?;
        let (input, _) = space1(input)?;
        let (input, name) = alpha1(input)?;
        let (input, args) = take_until_eol(input)?;

        let (input, contents) = take_lines_till(|line| line.eq_ignore_ascii_case("#+END:"))(input)?;

        Ok((
            input,
            (
                Element::DynBlock(DynBlock {
                    block_name: name,
                    arguments: if args.is_empty() { None } else { Some(args) },
                }),
                contents,
            ),
        ))
    }
}

#[test]
fn parse() {
    // TODO: testing
    assert_eq!(
        DynBlock::parse("#+BEGIN: clocktable :scope file\nCONTENTS\n#+END:\n"),
        Ok((
            "",
            (
                Element::DynBlock(DynBlock {
                    block_name: "clocktable",
                    arguments: Some(":scope file"),
                }),
                "CONTENTS\n"
            )
        ))
    );
}
