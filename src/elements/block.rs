use std::borrow::Cow;

use nom::{
    bytes::complete::tag_no_case, character::complete::alpha1, error::ParseError,
    sequence::preceded, IResult,
};

use crate::parsers::{blank_lines, line, take_lines_while};

/// Special Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct SpecialBlock<'a> {
    /// Optional block parameters
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub parameters: Option<Cow<'a, str>>,
    /// Block name
    pub name: Cow<'a, str>,
    /// Numbers of blank lines
    pub pre_blank: usize,
    /// Numbers of blank lines
    pub post_blank: usize,
}

impl SpecialBlock<'_> {
    pub fn into_owned(self) -> SpecialBlock<'static> {
        SpecialBlock {
            name: self.name.into_owned().into(),
            parameters: self.parameters.map(Into::into).map(Cow::Owned),
            pre_blank: self.pre_blank,
            post_blank: self.post_blank,
        }
    }
}

/// Quote Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct QuoteBlock<'a> {
    /// Optional block parameters
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub parameters: Option<Cow<'a, str>>,
    /// Numbers of blank lines
    pub pre_blank: usize,
    /// Numbers of blank lines
    pub post_blank: usize,
}

impl QuoteBlock<'_> {
    pub fn into_owned(self) -> QuoteBlock<'static> {
        QuoteBlock {
            parameters: self.parameters.map(Into::into).map(Cow::Owned),
            pre_blank: self.pre_blank,
            post_blank: self.post_blank,
        }
    }
}

/// Center Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct CenterBlock<'a> {
    /// Optional block parameters
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub parameters: Option<Cow<'a, str>>,
    /// Numbers of blank lines
    pub pre_blank: usize,
    /// Numbers of blank lines
    pub post_blank: usize,
}

impl CenterBlock<'_> {
    pub fn into_owned(self) -> CenterBlock<'static> {
        CenterBlock {
            parameters: self.parameters.map(Into::into).map(Cow::Owned),
            pre_blank: self.pre_blank,
            post_blank: self.post_blank,
        }
    }
}

/// Verse Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct VerseBlock<'a> {
    /// Optional block parameters
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub parameters: Option<Cow<'a, str>>,
    /// Numbers of blank lines
    pub pre_blank: usize,
    /// Numbers of blank lines
    pub post_blank: usize,
}

impl VerseBlock<'_> {
    pub fn into_owned(self) -> VerseBlock<'static> {
        VerseBlock {
            parameters: self.parameters.map(Into::into).map(Cow::Owned),
            pre_blank: self.pre_blank,
            post_blank: self.post_blank,
        }
    }
}

/// Comment Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct CommentBlock<'a> {
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub data: Option<Cow<'a, str>>,
    /// Comment, without block's boundaries
    pub contents: Cow<'a, str>,
    /// Numbers of blank lines
    pub post_blank: usize,
}

impl CommentBlock<'_> {
    pub fn into_owned(self) -> CommentBlock<'static> {
        CommentBlock {
            data: self.data.map(Into::into).map(Cow::Owned),
            contents: self.contents.into_owned().into(),
            post_blank: self.post_blank,
        }
    }
}

/// Example Block Element
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct ExampleBlock<'a> {
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub data: Option<Cow<'a, str>>,
    ///  Block contents
    pub contents: Cow<'a, str>,
    /// Numbers of blank lines
    pub post_blank: usize,
}

impl ExampleBlock<'_> {
    pub fn into_owned(self) -> ExampleBlock<'static> {
        ExampleBlock {
            data: self.data.map(Into::into).map(Cow::Owned),
            contents: self.contents.into_owned().into(),
            post_blank: self.post_blank,
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
    /// Numbers of blank lines
    pub post_blank: usize,
}

impl ExportBlock<'_> {
    pub fn into_owned(self) -> ExportBlock<'static> {
        ExportBlock {
            data: self.data.into_owned().into(),
            contents: self.contents.into_owned().into(),
            post_blank: self.post_blank,
        }
    }
}

/// Src Block Element
#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct SourceBlock<'a> {
    ///  Block contents
    pub contents: Cow<'a, str>,
    /// Language of the code in the block
    pub language: Cow<'a, str>,
    pub arguments: Cow<'a, str>,
    /// Numbers of blank lines
    pub post_blank: usize,
}

impl SourceBlock<'_> {
    pub fn into_owned(self) -> SourceBlock<'static> {
        SourceBlock {
            language: self.language.into_owned().into(),
            arguments: self.arguments.into_owned().into(),
            contents: self.contents.into_owned().into(),
            post_blank: self.post_blank,
        }
    }

    // TODO: fn number_lines() -> Some(New) | Some(Continued) | None {  }
    // TODO: fn preserve_indent() -> bool {  }
    // TODO: fn use_labels() -> bool {  }
    // TODO: fn label_fmt() -> Option<String> {  }
    // TODO: fn retain_labels() -> bool {  }
}

#[inline]
pub fn parse_block_element(input: &str) -> Option<(&str, (&str, Option<&str>, &str, usize))> {
    parse_block_element_internal::<()>(input).ok()
}

#[inline]
fn parse_block_element_internal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, (&str, Option<&str>, &str, usize), E> {
    let (input, name) = preceded(tag_no_case("#+BEGIN_"), alpha1)(input)?;
    let (input, args) = line(input)?;
    let end_line = format!("#+END_{}", name);
    let (input, contents) =
        take_lines_while(|line| !line.trim().eq_ignore_ascii_case(&end_line))(input);
    let (input, _) = line(input)?;
    let (input, blank) = blank_lines(input);

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
            blank,
        ),
    ))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_block_element_internal::<VerboseError<&str>>(
            r#"#+BEGIN_SRC
#+END_SRC"#
        ),
        Ok(("", ("SRC".into(), None, "", 0)))
    );
    assert_eq!(
        parse_block_element_internal::<VerboseError<&str>>(
            r#"#+begin_src
   #+end_src"#
        ),
        Ok(("", ("src".into(), None, "", 0)))
    );
    assert_eq!(
        parse_block_element_internal::<VerboseError<&str>>(
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
                "console.log('Hello World!');\n",
                1
            )
        ))
    );
    // TODO: more testing
}
