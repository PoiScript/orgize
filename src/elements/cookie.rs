use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit0,
    combinator::recognize,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Cookie<'a> {
    pub value: &'a str,
}

impl Cookie<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Cookie<'_>> {
        let (input, value) = recognize(delimited(
            tag("["),
            alt((
                separated_pair(digit0, tag("/"), digit0),
                pair(digit0, tag("%")),
            )),
            tag("]"),
        ))(input)?;

        Ok((input, Cookie { value }))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Cookie::parse("[1/10]"),
        Ok(("", Cookie { value: "[1/10]" }))
    );
    assert_eq!(
        Cookie::parse("[1/1000]"),
        Ok(("", Cookie { value: "[1/1000]" }))
    );
    assert_eq!(Cookie::parse("[10%]"), Ok(("", Cookie { value: "[10%]" })));
    assert_eq!(Cookie::parse("[%]"), Ok(("", Cookie { value: "[%]" })));
    assert_eq!(Cookie::parse("[/]"), Ok(("", Cookie { value: "[/]" })));
    assert_eq!(
        Cookie::parse("[100/]"),
        Ok(("", Cookie { value: "[100/]" }))
    );
    assert_eq!(
        Cookie::parse("[/100]"),
        Ok(("", Cookie { value: "[/100]" }))
    );

    assert!(Cookie::parse("[10% ]").is_err());
    assert!(Cookie::parse("[1//100]").is_err());
    assert!(Cookie::parse("[1\\100]").is_err());
    assert!(Cookie::parse("[10%%]").is_err());
}
