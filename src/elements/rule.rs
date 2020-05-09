use nom::{bytes::complete::take_while_m_n, character::complete::space0, IResult};

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
        parse_internal(input).ok()
    }
}

fn parse_internal(input: &str) -> IResult<&str, Rule, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = take_while_m_n(5, usize::max_value(), |c| c == '-')(input)?;
    let (input, _) = eol(input)?;
    let (input, post_blank) = blank_lines_count(input)?;
    Ok((input, Rule { post_blank }))
}

#[test]
fn parse() {
    assert_eq!(Rule::parse("-----"), Some(("", Rule { post_blank: 0 })));
    assert_eq!(Rule::parse("--------"), Some(("", Rule { post_blank: 0 })));
    assert_eq!(
        Rule::parse("-----\n\n\n"),
        Some(("", Rule { post_blank: 2 }))
    );
    assert_eq!(Rule::parse("-----  \n"), Some(("", Rule { post_blank: 0 })));

    assert!(Rule::parse("").is_none());
    assert!(Rule::parse("----").is_none());
    assert!(Rule::parse("----").is_none());
    assert!(Rule::parse("None----").is_none());
    assert!(Rule::parse("None  ----").is_none());
    assert!(Rule::parse("None------").is_none());
    assert!(Rule::parse("----None----").is_none());
    assert!(Rule::parse("\t\t----").is_none());
    assert!(Rule::parse("------None").is_none());
    assert!(Rule::parse("----- None").is_none());
}
