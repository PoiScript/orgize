use std::borrow::Cow;

use nom::{bytes::complete::tag_no_case, character::complete::alpha1, sequence::preceded, IResult};

use crate::parsers::{line, take_lines_while};

/// Special Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct SpecialBlock<'a> {
    /// Optional block parameters
    pub parameters: Option<Cow<'a, str>>,
    /// Block name
    pub name: Cow<'a, str>,
}

impl SpecialBlock<'_> {
    pub fn into_owned(self) -> SpecialBlock<'static> {
        SpecialBlock {
            name: self.name.into_owned().into(),
            parameters: self.parameters.map(Into::into).map(Cow::Owned),
        }
    }
}

/// Quote Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct QuoteBlock<'a> {
    /// Optional block parameters
    pub parameters: Option<Cow<'a, str>>,
}

impl QuoteBlock<'_> {
    pub fn into_owned(self) -> QuoteBlock<'static> {
        QuoteBlock {
            parameters: self.parameters.map(Into::into).map(Cow::Owned),
        }
    }
}

/// Center Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct CenterBlock<'a> {
    /// Optional block parameters
    pub parameters: Option<Cow<'a, str>>,
}

impl CenterBlock<'_> {
    pub fn into_owned(self) -> CenterBlock<'static> {
        CenterBlock {
            parameters: self.parameters.map(Into::into).map(Cow::Owned),
        }
    }
}

/// Verse Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct VerseBlock<'a> {
    /// Optional block parameters
    pub parameters: Option<Cow<'a, str>>,
}

impl VerseBlock<'_> {
    pub fn into_owned(self) -> VerseBlock<'static> {
        VerseBlock {
            parameters: self.parameters.map(Into::into).map(Cow::Owned),
        }
    }
}

/// Comment Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct CommentBlock<'a> {
    pub data: Option<Cow<'a, str>>,
    /// Comment, without block's boundaries
    pub contents: Cow<'a, str>,
}

impl CommentBlock<'_> {
    pub fn into_owned(self) -> CommentBlock<'static> {
        CommentBlock {
            data: self.data.map(Into::into).map(Cow::Owned),
            contents: self.contents.into_owned().into(),
        }
    }
}

/// Example Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct ExampleBlock<'a> {
    pub data: Option<Cow<'a, str>>,
    ///  Block contents
    pub contents: Cow<'a, str>,
}

impl ExampleBlock<'_> {
    pub fn into_owned(self) -> ExampleBlock<'static> {
        ExampleBlock {
            data: self.data.map(Into::into).map(Cow::Owned),
            contents: self.contents.into_owned().into(),
        }
    }
}

/// Export Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct ExportBlock<'a> {
    pub data: Cow<'a, str>,
    ///  Block contents
    pub contents: Cow<'a, str>,
}

impl ExportBlock<'_> {
    pub fn into_owned(self) -> ExportBlock<'static> {
        ExportBlock {
            data: self.data.into_owned().into(),
            contents: self.contents.into_owned().into(),
        }
    }
}

/// Src Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct SourceBlock<'a> {
    ///  Block contents
    pub contents: Cow<'a, str>,
    /// Language of the code in the block
    pub language: Cow<'a, str>,
    pub arguments: Cow<'a, str>,
}

impl SourceBlock<'_> {
    pub fn into_owned(self) -> SourceBlock<'static> {
        SourceBlock {
            language: self.language.into_owned().into(),
            arguments: self.arguments.into_owned().into(),
            contents: self.contents.into_owned().into(),
        }
    }

    // TODO: fn number_lines() -> Some(New) | Some(Continued) | None {  }
    // TODO: fn preserve_indent() -> bool {  }
    // TODO: fn use_labels() -> bool {  }
    // TODO: fn label_fmt() -> Option<String> {  }
    // TODO: fn retain_labels() -> bool {  }
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
