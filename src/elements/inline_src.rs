use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_till, take_while1},
    combinator::opt,
    sequence::delimited,
    IResult,
};

/// Inline Src Block Object
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InlineSrc<'a> {
    /// Language of the code
    pub lang: Cow<'a, str>,
    /// Optional header arguments
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub options: Option<Cow<'a, str>>,
    /// Source code
    pub body: Cow<'a, str>,
}

impl InlineSrc<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, InlineSrc)> {
        parse_internal(input).ok()
    }

    pub fn into_owned(self) -> InlineSrc<'static> {
        InlineSrc {
            lang: self.lang.into_owned().into(),
            options: self.options.map(Into::into).map(Cow::Owned),
            body: self.body.into_owned().into(),
        }
    }
}

#[inline]
fn parse_internal(input: &str) -> IResult<&str, InlineSrc, ()> {
    let (input, _) = tag("src_")(input)?;
    let (input, lang) =
        take_while1(|c: char| !c.is_ascii_whitespace() && c != '[' && c != '{')(input)?;
    let (input, options) = opt(delimited(
        tag("["),
        take_till(|c| c == '\n' || c == ']'),
        tag("]"),
    ))(input)?;
    let (input, body) = delimited(tag("{"), take_till(|c| c == '\n' || c == '}'), tag("}"))(input)?;

    Ok((
        input,
        InlineSrc {
            lang: lang.into(),
            options: options.map(Into::into),
            body: body.into(),
        },
    ))
}

#[test]
fn parse() {
    assert_eq!(
        InlineSrc::parse("src_C{int a = 0;}"),
        Some((
            "",
            InlineSrc {
                lang: "C".into(),
                options: None,
                body: "int a = 0;".into()
            },
        ))
    );
    assert_eq!(
        InlineSrc::parse("src_xml[:exports code]{<tag>text</tag>}"),
        Some((
            "",
            InlineSrc {
                lang: "xml".into(),
                options: Some(":exports code".into()),
                body: "<tag>text</tag>".into(),
            },
        ))
    );

    assert!(InlineSrc::parse("src_xml[:exports code]{<tag>text</tag>").is_none());
    assert!(InlineSrc::parse("src_[:exports code]{<tag>text</tag>}").is_none());
    assert!(InlineSrc::parse("src_xml[:exports code]").is_none());
}
