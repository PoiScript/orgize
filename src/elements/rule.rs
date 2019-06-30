use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while_m_n},
    character::complete::space0,
    combinator::{map, not},
    IResult,
};
use std::usize;

use crate::elements::Element;

pub struct Rule;

impl Rule {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Element<'_>> {
        let (input, _) = space0(input)?;
        let (input, _) = take_while_m_n(5, usize::MAX, |c| c == '-')(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = alt((tag("\n"), map(not(take(1usize)), |_| "")))(input)?;
        Ok((input, Element::Rule))
    }
}

#[test]
fn parse() {
    assert_eq!(Rule::parse("-----"), Ok(("", Element::Rule)));
    assert_eq!(Rule::parse("--------"), Ok(("", Element::Rule)));
    assert_eq!(Rule::parse("   -----"), Ok(("", Element::Rule)));
    assert_eq!(Rule::parse("\t\t-----"), Ok(("", Element::Rule)));
    assert_eq!(Rule::parse("\t\t-----\n"), Ok(("", Element::Rule)));
    assert_eq!(Rule::parse("\t\t-----  \n"), Ok(("", Element::Rule)));
    assert!(Rule::parse("").is_err());
    assert!(Rule::parse("----").is_err());
    assert!(Rule::parse("   ----").is_err());
    assert!(Rule::parse("  None----").is_err());
    assert!(Rule::parse("None  ----").is_err());
    assert!(Rule::parse("None------").is_err());
    assert!(Rule::parse("----None----").is_err());
    assert!(Rule::parse("\t\t----").is_err());
    assert!(Rule::parse("------None").is_err());
    assert!(Rule::parse("----- None").is_err());
}
