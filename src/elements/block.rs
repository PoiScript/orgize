use std::borrow::Cow;

use nom::{bytes::complete::tag_no_case, character::complete::alpha1, sequence::preceded, IResult};

use crate::parsers::{take_lines_till, take_until_eol};

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Block<'a> {
    pub name: Cow<'a, str>,
    pub args: Option<Cow<'a, str>>,
}

impl Block<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, (Block<'_>, &str)> {
        let (input, name) = preceded(tag_no_case("#+BEGIN_"), alpha1)(input)?;
        let (input, args) = take_until_eol(input)?;
        let end_line = format!(r"#+END_{}", name);
        let (input, contents) =
            take_lines_till(|line| line.eq_ignore_ascii_case(&end_line))(input)?;

        Ok((
            input,
            (
                Block {
                    name: name.into(),
                    args: if args.is_empty() {
                        None
                    } else {
                        Some(args.into())
                    },
                },
                contents,
            ),
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Block::parse("#+BEGIN_SRC\n#+END_SRC"),
        Ok((
            "",
            (
                Block {
                    name: "SRC".into(),
                    args: None,
                },
                ""
            )
        ))
    );
    assert_eq!(
        Block::parse("#+BEGIN_SRC javascript  \nconsole.log('Hello World!');\n#+END_SRC\n"),
        Ok((
            "",
            (
                Block {
                    name: "SRC".into(),
                    args: Some("javascript".into()),
                },
                "console.log('Hello World!');\n"
            )
        ))
    );
    // TODO: more testing
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct SpecialBlock<'a> {
    pub parameters: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct QuoteBlock<'a> {
    pub parameters: Option<Cow<'a, str>>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct CenterBlock<'a> {
    pub parameters: Option<Cow<'a, str>>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct VerseBlock<'a> {
    pub parameters: Option<Cow<'a, str>>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct CommentBlock<'a> {
    pub data: Option<Cow<'a, str>>,
    pub contents: Cow<'a, str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct ExampleBlock<'a> {
    pub data: Option<Cow<'a, str>>,
    pub contents: Cow<'a, str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct ExportBlock<'a> {
    pub data: Cow<'a, str>,
    pub contents: Cow<'a, str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct SourceBlock<'a> {
    pub contents: Cow<'a, str>,
    pub language: Cow<'a, str>,
    pub arguments: Cow<'a, str>,
}
