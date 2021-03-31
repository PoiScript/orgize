use nom::{
    bytes::complete::{tag, take_until},
    sequence::delimited,
    IResult,
};
use std::borrow::Cow;

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct LatexEnvironment<'a> {
    pub contents: Cow<'a, str>,
    pub argument: Cow<'a, str>,
    pub inline: bool,
}

impl<'a> LatexEnvironment<'a> {
    #[inline]
    pub fn parse(input: &str) -> Option<(&str, LatexEnvironment)> {
        let bytes = input.as_bytes();
        match (bytes[0], bytes[1]) {
            (b'\\', b'[') => parse_internal_delimited(input, "\\[", "\\]", false).ok(),
            (b'\\', b'(') => parse_internal_delimited(input, "\\(", "\\)", true).ok(),
            (b'\\', _) => parse_internal_environment(input).ok(),
            (b'$', b'$') => parse_internal_delimited(input, "$$", "$$", false).ok(),
            (b'$', _) => parse_internal_delimited(input, "$", "$", true).ok(),
            _ => None,
        }
    }

    pub fn into_owned(self) -> LatexEnvironment<'static> {
        LatexEnvironment {
            contents: self.contents.into_owned().into(),
            argument: self.argument.into_owned().into(),
            inline: self.inline,
        }
    }
}

fn parse_internal_delimited<'a>(
    input: &'a str,
    starts: &str,
    ends: &str,
    inline: bool,
) -> IResult<&'a str, LatexEnvironment<'a>, ()> {
    let (input, contents) = delimited(tag(starts), take_until(ends), tag(ends))(input)?;
    Ok((
        input,
        LatexEnvironment {
            contents: contents.trim().into(),
            argument: "".into(),
            inline: inline,
        },
    ))
}

fn parse_internal_environment(input: &str) -> IResult<&str, LatexEnvironment, ()> {
    let (input, argument) = delimited(tag("\\begin{"), take_until("}"), tag("}"))(input)?;
    let end = &format!("\\end{{{}}}", argument)[..];
    let (input, contents) = take_until(end)(input)?;
    let (input, _) = tag(end)(input)?;
    Ok((
        input,
        LatexEnvironment {
            contents: contents.trim().into(),
            argument: argument.into(),
            inline: false,
        },
    ))
}

#[test]
fn parse_environment() {
    assert_eq!(
        LatexEnvironment::parse("\\[\n\\frac{1}{3}\n\n\\]"),
        Some((
            "",
            LatexEnvironment {
                contents: "\\frac{1}{3}".into(),
                argument: "".into(),
                inline: false,
            }
        ))
    );

    assert_eq!(
        LatexEnvironment::parse("$$\n42!\n\n$$"),
        Some((
            "",
            LatexEnvironment {
                contents: "42!".into(),
                argument: "".into(),
                inline: false,
            }
        ))
    );

    assert_eq!(
        LatexEnvironment::parse(
            r#"\begin{equation}
\int^{a}_{b}\,f(x)\,dx
\end{equation}"#
        ),
        Some((
            "",
            LatexEnvironment {
                contents: "\\int^{a}_{b}\\,f(x)\\,dx".into(),
                argument: "equation".into(),
                inline: false,
            }
        ))
    );

    assert_eq!(
        LatexEnvironment::parse(
            r#"\begin{equation}
\int^{a}_{b}\,f(x)\,dx
\end{align}"#
        ),
        None
    );
}

#[test]
fn parse_inline() {
    assert_eq!(
        LatexEnvironment::parse("$\\frac{1}{3}$"),
        Some((
            "",
            LatexEnvironment {
                contents: "\\frac{1}{3}".into(),
                argument: "".into(),
                inline: true,
            }
        ))
    );

    assert_eq!(
        LatexEnvironment::parse("\\(\\frac{1}{3}\\)"),
        Some((
            "",
            LatexEnvironment {
                contents: "\\frac{1}{3}".into(),
                argument: "".into(),
                inline: true,
            }
        ))
    );
    assert_eq!(
        LatexEnvironment::parse("$ text   with spaces   $"),
        Some((
            "",
            LatexEnvironment {
                contents: "text   with spaces".into(),
                argument: "".into(),
                inline: true,
            }
        ))
    );
    assert_eq!(
        LatexEnvironment::parse("\\( text   with spaces   \\)"),
        Some((
            "",
            LatexEnvironment {
                contents: "text   with spaces".into(),
                argument: "".into(),
                inline: true,
            }
        ))
    );
    assert_eq!(
        LatexEnvironment::parse("$ LaTeXxxx$"),
        Some((
            "",
            LatexEnvironment {
                contents: "LaTeXxxx".into(),
                argument: "".into(),
                inline: true,
            }
        ))
    );
    assert_eq!(
        LatexEnvironment::parse("\\(b\nol\nd\\)"),
        Some((
            "",
            LatexEnvironment {
                contents: "b\nol\nd".into(),
                argument: "".into(),
                inline: true,
            }
        ))
    );
    assert_eq!(LatexEnvironment::parse("$$b\nol\nd*"), None);
    assert_eq!(LatexEnvironment::parse("$b\nol\nd*"), None);
}
