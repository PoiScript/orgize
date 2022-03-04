use std::borrow::Cow;

use nom::{
    bytes::complete::tag_no_case,
    character::complete::{alpha1, space0},
    sequence::preceded,
    IResult,
};

use crate::elements::Element;
use crate::parse::combinators::{blank_lines_count, line, lines_till};

/// Special Block Element
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct SpecialBlock<'a> {
    /// Block parameters
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub parameters: Option<Cow<'a, str>>,
    /// Block name
    pub name: Cow<'a, str>,
    /// Numbers of blank lines between first block's line and next non-blank
    /// line
    pub pre_blank: usize,
    /// Numbers of blank lines between last block's line and next non-blank line
    /// or buffer's end
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct QuoteBlock<'a> {
    /// Optional block parameters
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub parameters: Option<Cow<'a, str>>,
    /// Numbers of blank lines between first block's line and next non-blank
    /// line
    pub pre_blank: usize,
    /// Numbers of blank lines between last block's line and next non-blank line
    /// or buffer's end
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct CenterBlock<'a> {
    /// Optional block parameters
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub parameters: Option<Cow<'a, str>>,
    /// Numbers of blank lines between first block's line and next non-blank
    /// line
    pub pre_blank: usize,
    /// Numbers of blank lines between last block's line and next non-blank line
    /// or buffer's end
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct VerseBlock<'a> {
    /// Optional block parameters
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub parameters: Option<Cow<'a, str>>,
    /// Numbers of blank lines between first block's line and next non-blank
    /// line
    pub pre_blank: usize,
    /// Numbers of blank lines between last block's line and next non-blank line
    /// or buffer's end
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct CommentBlock<'a> {
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub data: Option<Cow<'a, str>>,
    /// Comment block contents
    pub contents: Cow<'a, str>,
    /// Numbers of blank lines between last block's line and next non-blank line
    /// or buffer's end
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct ExampleBlock<'a> {
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub data: Option<Cow<'a, str>>,
    ///  Block contents
    pub contents: Cow<'a, str>,
    /// Numbers of blank lines between last block's line and next non-blank line
    /// or buffer's end
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct ExportBlock<'a> {
    pub data: Cow<'a, str>,
    ///  Block contents
    pub contents: Cow<'a, str>,
    /// Numbers of blank lines between last block's line and next non-blank line
    /// or buffer's end
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
#[derive(Debug, Default, PartialEq, Clone)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct SourceBlock<'a> {
    ///  Block contents
    pub contents: Cow<'a, str>,
    /// Language of the code in the block
    pub language: Cow<'a, str>,
    pub arguments: Cow<'a, str>,
    /// Numbers of blank lines between last block's line and next non-blank line
    /// or buffer's end
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

#[derive(Debug, PartialEq)]
pub(crate) struct RawBlock<'a> {
    pub name: &'a str,
    pub arguments: &'a str,

    pub pre_blank: usize,
    pub contents: &'a str,
    pub contents_without_blank_lines: &'a str,

    pub post_blank: usize,
}

impl<'a> RawBlock<'a> {
    pub fn parse(input: &str) -> Option<(&str, RawBlock)> {
        parse_internal(input).ok()
    }

    pub fn into_element(self) -> (Element<'a>, &'a str) {
        let RawBlock {
            name,
            contents,
            arguments,
            pre_blank,
            contents_without_blank_lines,
            post_blank,
        } = self;

        let arguments: Option<Cow<'a, str>> = if arguments.is_empty() {
            None
        } else {
            Some(arguments.into())
        };

        let element = match &*name.to_uppercase() {
            "CENTER" => CenterBlock {
                parameters: arguments,
                pre_blank,
                post_blank,
            }
            .into(),
            "QUOTE" => QuoteBlock {
                parameters: arguments,
                pre_blank,
                post_blank,
            }
            .into(),
            "VERSE" => VerseBlock {
                parameters: arguments,
                pre_blank,
                post_blank,
            }
            .into(),
            "COMMENT" => CommentBlock {
                data: arguments,
                contents: contents.into(),
                post_blank,
            }
            .into(),
            "EXAMPLE" => ExampleBlock {
                data: arguments,
                contents: contents.into(),
                post_blank,
            }
            .into(),
            "EXPORT" => ExportBlock {
                data: arguments.unwrap_or_default(),
                contents: contents.into(),
                post_blank,
            }
            .into(),
            "SRC" => {
                let (language, arguments) = match &arguments {
                    Some(Cow::Borrowed(args)) => {
                        let (language, arguments) =
                            args.split_at(args.find(' ').unwrap_or_else(|| args.len()));
                        (language.into(), arguments.into())
                    }
                    None => (Cow::Borrowed(""), Cow::Borrowed("")),
                    _ => unreachable!(
                        "`parse_block_element` returns `Some(Cow::Borrowed)` or `None`"
                    ),
                };
                SourceBlock {
                    arguments,
                    language,
                    contents: contents.into(),
                    post_blank,
                }
                .into()
            }
            _ => SpecialBlock {
                parameters: arguments,
                name: name.into(),
                pre_blank,
                post_blank,
            }
            .into(),
        };

        (element, contents_without_blank_lines)
    }
}

fn parse_internal(input: &str) -> IResult<&str, RawBlock, ()> {
    let (input, _) = space0(input)?;
    let (input, name) = preceded(tag_no_case("#+BEGIN_"), alpha1)(input)?;
    let (input, arguments) = line(input)?;
    let end_line = format!("#+END_{}", name);
    let (input, contents) = lines_till(|line| line.trim().eq_ignore_ascii_case(&end_line))(input)?;
    let (contents_without_blank_lines, pre_blank) = blank_lines_count(contents)?;
    let (input, post_blank) = blank_lines_count(input)?;

    Ok((
        input,
        RawBlock {
            name,
            contents,
            arguments: arguments.trim(),
            pre_blank,
            contents_without_blank_lines,
            post_blank,
        },
    ))
}

#[test]
fn parse() {
    assert_eq!(
        RawBlock::parse(
            r#"#+BEGIN_SRC
#+END_SRC"#
        ),
        Some((
            "",
            RawBlock {
                contents: "",
                contents_without_blank_lines: "",
                pre_blank: 0,
                post_blank: 0,
                name: "SRC".into(),
                arguments: ""
            }
        ))
    );

    assert_eq!(
        RawBlock::parse(
            r#"#+begin_src
   #+end_src"#
        ),
        Some((
            "",
            RawBlock {
                contents: "",
                contents_without_blank_lines: "",
                pre_blank: 0,
                post_blank: 0,
                name: "src".into(),
                arguments: ""
            }
        ))
    );

    assert_eq!(
        RawBlock::parse(
            r#"#+BEGIN_SRC javascript
console.log('Hello World!');
#+END_SRC

"#
        ),
        Some((
            "",
            RawBlock {
                contents: "console.log('Hello World!');\n",
                contents_without_blank_lines: "console.log('Hello World!');\n",
                pre_blank: 0,
                post_blank: 1,
                name: "SRC".into(),
                arguments: "javascript"
            }
        ))
    );
    // TODO: more testing
}
