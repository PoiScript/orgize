use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_till, take_while1},
    combinator::opt,
    error::ParseError,
    sequence::delimited,
    IResult,
};

/// Inline Src Block Object
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
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
    pub(crate) fn parse(input: &str) -> Option<(&str, InlineSrc<'_>)> {
        parse_inline_src::<()>(input).ok()
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
fn parse_inline_src<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, InlineSrc<'a>, E> {
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
    use nom::error::VerboseError;

    assert_eq!(
        parse_inline_src::<VerboseError<&str>>("src_C{int a = 0;}"),
        Ok((
            "",
            InlineSrc {
                lang: "C".into(),
                options: None,
                body: "int a = 0;".into()
            },
        ))
    );
    assert_eq!(
        parse_inline_src::<VerboseError<&str>>("src_xml[:exports code]{<tag>text</tag>}"),
        Ok((
            "",
            InlineSrc {
                lang: "xml".into(),
                options: Some(":exports code".into()),
                body: "<tag>text</tag>".into(),
            },
        ))
    );

    assert!(
        parse_inline_src::<VerboseError<&str>>("src_xml[:exports code]{<tag>text</tag>").is_err()
    );
    assert!(
        parse_inline_src::<VerboseError<&str>>("src_[:exports code]{<tag>text</tag>}").is_err()
    );
    assert!(parse_inline_src::<VerboseError<&str>>("src_xml[:exports code]").is_err());
}
