use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_while},
    combinator::verify,
    sequence::delimited,
    IResult,
};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Target<'a> {
    pub target: Cow<'a, str>,
}

impl Target<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Target<'_>> {
        let (input, target) = delimited(
            tag("<<"),
            verify(
                take_while(|c: char| c != '<' && c != '\n' && c != '>'),
                |s: &str| s.starts_with(|c| c != ' ') && s.ends_with(|c| c != ' '),
            ),
            tag(">>"),
        )(input)?;

        Ok((
            input,
            Target {
                target: target.into(),
            },
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Target::parse("<<target>>"),
        Ok((
            "",
            Target {
                target: "target".into()
            }
        ))
    );
    assert_eq!(
        Target::parse("<<tar get>>"),
        Ok((
            "",
            Target {
                target: "tar get".into()
            }
        ))
    );
    assert!(Target::parse("<<target >>").is_err());
    assert!(Target::parse("<< target>>").is_err());
    assert!(Target::parse("<<ta<get>>").is_err());
    assert!(Target::parse("<<ta>get>>").is_err());
    assert!(Target::parse("<<ta\nget>>").is_err());
    assert!(Target::parse("<<target>").is_err());
}
