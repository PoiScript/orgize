use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_till, take_while1},
    combinator::opt,
    sequence::delimited,
    IResult,
};

use crate::elements::Element;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct InlineSrc<'a> {
    pub lang: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub options: Option<Cow<'a, str>>,
    pub body: Cow<'a, str>,
}

impl InlineSrc<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Element<'_>> {
        let (input, _) = tag("src_")(input)?;
        let (input, lang) =
            take_while1(|c: char| !c.is_ascii_whitespace() && c != '[' && c != '{')(input)?;
        let (input, options) = opt(delimited(
            tag("["),
            take_till(|c| c == '\n' || c == ']'),
            tag("]"),
        ))(input)?;
        let (input, body) =
            delimited(tag("{"), take_till(|c| c == '\n' || c == '}'), tag("}"))(input)?;

        Ok((
            input,
            Element::InlineSrc(InlineSrc {
                lang: lang.into(),
                options: options.map(Into::into),
                body: body.into(),
            }),
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        InlineSrc::parse("src_C{int a = 0;}"),
        Ok((
            "",
            Element::InlineSrc(InlineSrc {
                lang: "C".into(),
                options: None,
                body: "int a = 0;".into()
            }),
        ))
    );
    assert_eq!(
        InlineSrc::parse("src_xml[:exports code]{<tag>text</tag>}"),
        Ok((
            "",
            Element::InlineSrc(InlineSrc {
                lang: "xml".into(),
                options: Some(":exports code".into()),
                body: "<tag>text</tag>".into(),
            }),
        ))
    );

    assert!(InlineSrc::parse("src_xml[:exports code]{<tag>text</tag>").is_err());
    assert!(InlineSrc::parse("src_[:exports code]{<tag>text</tag>}").is_err());
    assert!(InlineSrc::parse("src_xml[:exports code]").is_err());
}
