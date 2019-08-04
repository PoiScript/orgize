use memchr::{memchr, memchr_iter};

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Block<'a> {
    pub name: &'a str,
    pub args: Option<&'a str>,
}

impl Block<'_> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(&str, Block<'_>, &str)> {
        debug_assert!(text.starts_with("#+"));

        if text.len() <= 8 || text[2..8].to_uppercase() != "BEGIN_" {
            return None;
        }

        let mut lines = memchr_iter(b'\n', text.as_bytes());

        let (name, args, off) = lines
            .next()
            .map(|i| {
                memchr(b' ', &text.as_bytes()[8..i])
                    .map(|x| (&text[8..8 + x], Some(text[8 + x..i].trim()), i + 1))
                    .unwrap_or((&text[8..i], None, i + 1))
            })
            .filter(|(name, _, _)| name.as_bytes().iter().all(|&c| c.is_ascii_alphabetic()))?;

        let mut pos = off;
        let end = format!(r"#+END_{}", name.to_uppercase());

        for i in lines {
            if text[pos..i].trim().eq_ignore_ascii_case(&end) {
                return Some((&text[i + 1..], Block { name, args }, &text[off..pos]));
            }

            pos = i + 1;
        }

        if text[pos..].trim().eq_ignore_ascii_case(&end) {
            Some(("", Block { name, args }, &text[off..pos]))
        } else {
            None
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Block::parse("#+BEGIN_SRC\n#+END_SRC"),
        Some((
            "",
            Block {
                name: "SRC",
                args: None,
            },
            ""
        ))
    );
    assert_eq!(
        Block::parse("#+BEGIN_SRC javascript  \nconsole.log('Hello World!');\n#+END_SRC\n"),
        Some((
            "",
            Block {
                name: "SRC",
                args: Some("javascript"),
            },
            "console.log('Hello World!');\n"
        ))
    );
    // TODO: more testing
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SpecialBlock<'a> {
    pub parameters: Option<&'a str>,
    pub name: &'a str,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct QuoteBlock<'a> {
    pub parameters: Option<&'a str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct CenterBlock<'a> {
    pub parameters: Option<&'a str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VerseBlock<'a> {
    pub parameters: Option<&'a str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct CommentBlock<'a> {
    pub data: Option<&'a str>,
    pub contents: &'a str,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExampleBlock<'a> {
    pub data: Option<&'a str>,
    pub contents: &'a str,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExportBlock<'a> {
    pub data: &'a str,
    pub contents: &'a str,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SourceBlock<'a> {
    pub contents: &'a str,
    pub language: &'a str,
    pub arguments: &'a str,
}
