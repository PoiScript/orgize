use nom::{bytes::complete::tag_no_case, character::complete::alpha1, sequence::preceded, IResult};

use crate::parsers::{take_lines_till, take_until_eol};

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Block<'a> {
    pub name: &'a str,
    pub args: Option<&'a str>,
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
                    name,
                    args: if args.is_empty() { None } else { Some(args) },
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
                    name: "SRC",
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
                    name: "SRC",
                    args: Some("javascript"),
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
    pub parameters: Option<&'a str>,
    pub name: &'a str,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct QuoteBlock<'a> {
    pub parameters: Option<&'a str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct CenterBlock<'a> {
    pub parameters: Option<&'a str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct VerseBlock<'a> {
    pub parameters: Option<&'a str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct CommentBlock<'a> {
    pub data: Option<&'a str>,
    pub contents: &'a str,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct ExampleBlock<'a> {
    pub data: Option<&'a str>,
    pub contents: &'a str,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct ExportBlock<'a> {
    pub data: &'a str,
    pub contents: &'a str,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct SourceBlock<'a> {
    pub contents: &'a str,
    pub language: &'a str,
    pub arguments: &'a str,
}
