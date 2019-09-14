use std::usize;

use nom::{bytes::complete::take_while_m_n, IResult};

use crate::parsers::eol;

pub(crate) fn parse_rule(input: &str) -> IResult<&str, ()> {
    let (input, _) = take_while_m_n(5, usize::MAX, |c| c == '-')(input)?;
    let (input, _) = eol(input)?;
    Ok((input, ()))
}

#[test]
fn parse() {
    assert_eq!(parse_rule("-----"), Ok(("", ())));
    assert_eq!(parse_rule("--------"), Ok(("", ())));
    assert_eq!(parse_rule("-----\n"), Ok(("", ())));
    assert_eq!(parse_rule("-----  \n"), Ok(("", ())));
    assert!(parse_rule("").is_err());
    assert!(parse_rule("----").is_err());
    assert!(parse_rule("----").is_err());
    assert!(parse_rule("None----").is_err());
    assert!(parse_rule("None  ----").is_err());
    assert!(parse_rule("None------").is_err());
    assert!(parse_rule("----None----").is_err());
    assert!(parse_rule("\t\t----").is_err());
    assert!(parse_rule("------None").is_err());
    assert!(parse_rule("----- None").is_err());
}
