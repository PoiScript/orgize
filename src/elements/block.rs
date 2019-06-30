use memchr::{memchr, memchr_iter};

use crate::elements::Element;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Block<'a> {
    pub name: &'a str,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub args: Option<&'a str>,
    #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
    pub contents: &'a str,
}

impl Block<'_> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(&str, Element<'_>)> {
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
                return Some((
                    &text[i + 1..],
                    Element::Block(Block {
                        name,
                        args,
                        contents: &text[off..pos],
                    }),
                ));
            }

            pos = i + 1;
        }

        if text[pos..].trim().eq_ignore_ascii_case(&end) {
            Some((
                "",
                Element::Block(Block {
                    name,
                    args,
                    contents: &text[off..pos],
                }),
            ))
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
            Element::Block(Block {
                name: "SRC",
                args: None,
                contents: ""
            }),
        ))
    );
    assert_eq!(
        Block::parse("#+BEGIN_SRC javascript  \nconsole.log('Hello World!');\n#+END_SRC\n"),
        Some((
            "",
            Element::Block(Block {
                name: "SRC",
                args: Some("javascript"),
                contents: "console.log('Hello World!');\n"
            }),
        ))
    );
    // TODO: more testing
}
