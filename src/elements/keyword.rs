use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::space0,
    combinator::opt,
    error::ParseError,
    sequence::delimited,
    IResult,
};

use crate::elements::Element;
use crate::parse::combinators::{blank_lines_count, line};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct RawKeyword<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub optional: Option<&'a str>,
    pub post_blank: usize,
}

impl<'a> RawKeyword<'a> {
    pub fn parse(input: &'a str) -> Option<(&str, RawKeyword)> {
        Self::parse_internal::<()>(input).ok()
    }

    fn parse_internal<E>(input: &'a str) -> IResult<&str, RawKeyword, E>
    where
        E: ParseError<&'a str>,
    {
        let (input, _) = space0(input)?;
        let (input, _) = tag("#+")(input)?;
        let (input, key) =
            take_till(|c: char| c.is_ascii_whitespace() || c == ':' || c == '[')(input)?;
        let (input, optional) = opt(delimited(
            tag("["),
            take_till(|c| c == ']' || c == '\n'),
            tag("]"),
        ))(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, value) = line(input)?;
        let (input, post_blank) = blank_lines_count(input)?;

        Ok((
            input,
            RawKeyword {
                key,
                optional,
                value: value.trim(),
                post_blank,
            },
        ))
    }

    pub fn into_element(self) -> Element<'a> {
        let RawKeyword {
            key,
            value,
            optional,
            post_blank,
        } = self;

        if (&*key).eq_ignore_ascii_case("CALL") {
            BabelCall {
                value: value.into(),
                post_blank,
            }
            .into()
        } else {
            Keyword {
                key: key.into(),
                optional: optional.map(Into::into),
                value: value.into(),
                post_blank,
            }
            .into()
        }
    }
}

/// Keyword Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Keyword<'a> {
    /// Keyword name
    pub key: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub optional: Option<Cow<'a, str>>,
    /// Keyword value
    pub value: Cow<'a, str>,
    /// Numbers of blank lines between keyword line and next non-blank line or
    /// buffer's end
    pub post_blank: usize,
}

impl Keyword<'_> {
    pub fn into_owned(self) -> Keyword<'static> {
        Keyword {
            key: self.key.into_owned().into(),
            optional: self.optional.map(Into::into).map(Cow::Owned),
            value: self.value.into_owned().into(),
            post_blank: self.post_blank,
        }
    }
}

/// Babel Call Element
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct BabelCall<'a> {
    /// Babel call value
    pub value: Cow<'a, str>,
    /// Numbers of blank lines between babel call line and next non-blank line
    /// or buffer's end
    pub post_blank: usize,
}

impl BabelCall<'_> {
    pub fn into_owned(self) -> BabelCall<'static> {
        BabelCall {
            value: self.value.into_owned().into(),
            post_blank: self.post_blank,
        }
    }
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        RawKeyword::parse_internal::<VerboseError<&str>>("#+KEY:"),
        Ok((
            "",
            RawKeyword {
                key: "KEY",
                optional: None,
                value: "",
                post_blank: 0
            }
        ))
    );
    assert_eq!(
        RawKeyword::parse_internal::<VerboseError<&str>>("#+KEY: VALUE"),
        Ok((
            "",
            RawKeyword {
                key: "KEY",
                optional: None,
                value: "VALUE",
                post_blank: 0
            }
        ))
    );
    assert_eq!(
        RawKeyword::parse_internal::<VerboseError<&str>>("#+K_E_Y: VALUE"),
        Ok((
            "",
            RawKeyword {
                key: "K_E_Y",
                optional: None,
                value: "VALUE",
                post_blank: 0
            }
        ))
    );
    assert_eq!(
        RawKeyword::parse_internal::<VerboseError<&str>>("#+KEY:VALUE\n"),
        Ok((
            "",
            RawKeyword {
                key: "KEY",
                optional: None,
                value: "VALUE",
                post_blank: 0
            }
        ))
    );
    assert!(RawKeyword::parse_internal::<VerboseError<&str>>("#+KE Y: VALUE").is_err());
    assert!(RawKeyword::parse_internal::<VerboseError<&str>>("#+ KEY: VALUE").is_err());

    assert_eq!(
        RawKeyword::parse_internal::<VerboseError<&str>>("#+RESULTS:"),
        Ok((
            "",
            RawKeyword {
                key: "RESULTS",
                optional: None,
                value: "",
                post_blank: 0
            }
        ))
    );

    assert_eq!(
        RawKeyword::parse_internal::<VerboseError<&str>>("#+ATTR_LATEX: :width 5cm\n"),
        Ok((
            "",
            RawKeyword {
                key: "ATTR_LATEX",
                optional: None,
                value: ":width 5cm",
                post_blank: 0
            }
        ))
    );

    assert_eq!(
        RawKeyword::parse_internal::<VerboseError<&str>>("#+CALL: double(n=4)"),
        Ok((
            "",
            RawKeyword {
                key: "CALL",
                optional: None,
                value: "double(n=4)",
                post_blank: 0
            }
        ))
    );

    assert_eq!(
        RawKeyword::parse_internal::<VerboseError<&str>>(
            "#+CAPTION[Short caption]: Longer caption."
        ),
        Ok((
            "",
            RawKeyword {
                key: "CAPTION",
                optional: Some("Short caption"),
                value: "Longer caption.",
                post_blank: 0
            }
        ))
    );
}
