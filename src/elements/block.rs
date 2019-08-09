use std::borrow::Cow;

use nom::{bytes::complete::tag_no_case, character::complete::alpha1, sequence::preceded, IResult};

use crate::parsers::{line, take_lines_while};

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

pub(crate) fn parse_block_element(input: &str) -> IResult<&str, (&str, Option<&str>, &str)> {
    let (input, name) = preceded(tag_no_case("#+BEGIN_"), alpha1)(input)?;
    let (input, args) = line(input)?;
    let end_line = format!(r"#+END_{}", name);
    let (input, contents) =
        take_lines_while(|line| !line.trim().eq_ignore_ascii_case(&end_line))(input)?;
    let (input, _) = line(input)?;

    Ok((
        input,
        (
            name,
            if args.trim().is_empty() {
                None
            } else {
                Some(args.trim())
            },
            contents,
        ),
    ))
}

#[test]
fn parse() {
    assert_eq!(
        parse_block_element(
            r#"#+BEGIN_SRC
#+END_SRC"#
        ),
        Ok(("", ("SRC".into(), None, "")))
    );
    assert_eq!(
        parse_block_element(
            r#"#+begin_src
   #+end_src"#
        ),
        Ok(("", ("src".into(), None, "")))
    );
    assert_eq!(
        parse_block_element(
            r#"#+BEGIN_SRC javascript
console.log('Hello World!');
#+END_SRC
"#
        ),
        Ok((
            "",
            (
                "SRC".into(),
                Some("javascript".into()),
                "console.log('Hello World!');\n"
            )
        ))
    );
    // TODO: more testing
}
