use std::usize;

use nom::{bytes::complete::take_while_m_n, error::ParseError, IResult};

use crate::parsers::eol;

pub fn parse_rule(input: &str) -> Option<&str> {
    parse_rule_internal::<()>(input)
        .ok()
        .map(|(input, _)| input)
}

fn parse_rule_internal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, (), E> {
    let (input, _) = take_while_m_n(5, usize::MAX, |c| c == '-')(input)?;
    let (input, _) = eol(input)?;
    Ok((input, ()))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_rule_internal::<VerboseError<&str>>("-----"),
        Ok(("", ()))
    );
    assert_eq!(
        parse_rule_internal::<VerboseError<&str>>("--------"),
        Ok(("", ()))
    );
    assert_eq!(
        parse_rule_internal::<VerboseError<&str>>("-----\n"),
        Ok(("", ()))
    );
    assert_eq!(
        parse_rule_internal::<VerboseError<&str>>("-----  \n"),
        Ok(("", ()))
    );
    assert!(parse_rule_internal::<VerboseError<&str>>("").is_err());
    assert!(parse_rule_internal::<VerboseError<&str>>("----").is_err());
    assert!(parse_rule_internal::<VerboseError<&str>>("----").is_err());
    assert!(parse_rule_internal::<VerboseError<&str>>("None----").is_err());
    assert!(parse_rule_internal::<VerboseError<&str>>("None  ----").is_err());
    assert!(parse_rule_internal::<VerboseError<&str>>("None------").is_err());
    assert!(parse_rule_internal::<VerboseError<&str>>("----None----").is_err());
    assert!(parse_rule_internal::<VerboseError<&str>>("\t\t----").is_err());
    assert!(parse_rule_internal::<VerboseError<&str>>("------None").is_err());
    assert!(parse_rule_internal::<VerboseError<&str>>("----- None").is_err());
}
