use nom::{
    bytes::complete::take_while_m_n, character::complete::space0, error::ParseError, IResult,
};

use crate::parse::combinators::{blank_lines_count, eol};

#[derive(Debug, Default, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct Rule {
    /// Numbers of blank lines between rule line and next non-blank line or
    /// buffer's end
    pub post_blank: usize,
}

impl Rule {
    pub(crate) fn parse(input: &str) -> Option<(&str, Rule)> {
        parse_rule::<()>(input).ok()
    }
}

fn parse_rule<'a, E>(input: &'a str) -> IResult<&str, Rule, E>
where
    E: ParseError<&'a str>,
{
    let (input, _) = space0(input)?;
    let (input, _) = take_while_m_n(5, usize::max_value(), |c| c == '-')(input)?;
    let (input, _) = eol(input)?;
    let (input, post_blank) = blank_lines_count(input)?;
    Ok((input, Rule { post_blank }))
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
