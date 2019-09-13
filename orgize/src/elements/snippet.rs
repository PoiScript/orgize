use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_until, take_while1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Snippet<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl Snippet<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Snippet<'_>> {
        let (input, (name, value)) = delimited(
            tag("@@"),
            separated_pair(
                take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-'),
                tag(":"),
                take_until("@@"),
            ),
            tag("@@"),
        )(input)?;

        Ok((
            input,
            Snippet {
                name: name.into(),
                value: value.into(),
            },
        ))
    }

    pub fn into_owned(self) -> Snippet<'static> {
        Snippet {
            name: self.name.into_owned().into(),
            value: self.value.into_owned().into(),
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Snippet::parse("@@html:<b>@@"),
        Ok((
            "",
            Snippet {
                name: "html".into(),
                value: "<b>".into()
            }
        ))
    );
    assert_eq!(
        Snippet::parse("@@latex:any arbitrary LaTeX code@@"),
        Ok((
            "",
            Snippet {
                name: "latex".into(),
                value: "any arbitrary LaTeX code".into(),
            }
        ))
    );
    assert_eq!(
        Snippet::parse("@@html:@@"),
        Ok((
            "",
            Snippet {
                name: "html".into(),
                value: "".into(),
            }
        ))
    );
    assert_eq!(
        Snippet::parse("@@html:<p>@</p>@@"),
        Ok((
            "",
            Snippet {
                name: "html".into(),
                value: "<p>@</p>".into(),
            }
        ))
    );
    assert!(Snippet::parse("@@html:<b>@").is_err());
    assert!(Snippet::parse("@@html<b>@@").is_err());
    assert!(Snippet::parse("@@:<b>@@").is_err());
}
