use nom::{bytes::complete::take_while_m_n, error::ParseError, IResult};

use crate::parsers::{blank_lines, eol};

#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct Rule {
    pub post_blank: usize,
}

impl Rule {
    pub(crate) fn parse(input: &str) -> Option<(&str, Rule)> {
        parse_rule::<()>(input).ok()
    }
}

fn parse_rule<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Rule, E> {
    let (input, _) = take_while_m_n(5, usize::max_value(), |c| c == '-')(input)?;
    let (input, _) = eol(input)?;
    let (input, blank) = blank_lines(input);
    Ok((input, Rule { post_blank: blank }))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_rule::<VerboseError<&str>>("-----"),
        Ok(("", Rule { post_blank: 0 }))
    );
    assert_eq!(
        parse_rule::<VerboseError<&str>>("--------"),
        Ok(("", Rule { post_blank: 0 }))
    );
    assert_eq!(
        parse_rule::<VerboseError<&str>>("-----\n\n\n"),
        Ok(("", Rule { post_blank: 2 }))
    );
    assert_eq!(
        parse_rule::<VerboseError<&str>>("-----  \n"),
        Ok(("", Rule { post_blank: 0 }))
    );
    assert!(parse_rule::<VerboseError<&str>>("").is_err());
    assert!(parse_rule::<VerboseError<&str>>("----").is_err());
    assert!(parse_rule::<VerboseError<&str>>("----").is_err());
    assert!(parse_rule::<VerboseError<&str>>("None----").is_err());
    assert!(parse_rule::<VerboseError<&str>>("None  ----").is_err());
    assert!(parse_rule::<VerboseError<&str>>("None------").is_err());
    assert!(parse_rule::<VerboseError<&str>>("----None----").is_err());
    assert!(parse_rule::<VerboseError<&str>>("\t\t----").is_err());
    assert!(parse_rule::<VerboseError<&str>>("------None").is_err());
    assert!(parse_rule::<VerboseError<&str>>("----- None").is_err());
}
