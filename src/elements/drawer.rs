use memchr::memchr_iter;

use crate::elements::Element;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Drawer<'a> {
    pub name: &'a str,
    #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
    pub contents: &'a str,
}

impl Drawer<'_> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(&str, Element<'_>)> {
        debug_assert!(text.starts_with(':'));

        let mut lines = memchr_iter(b'\n', text.as_bytes());

        let (name, off) = lines
            .next()
            .map(|i| (text[1..i].trim_end(), i + 1))
            .filter(|(name, _)| {
                name.ends_with(':')
                    && name[0..name.len() - 1]
                        .as_bytes()
                        .iter()
                        .all(|&c| c.is_ascii_alphabetic() || c == b'-' || c == b'_')
            })?;

        let mut pos = off;
        for i in lines {
            if text[pos..i].trim().eq_ignore_ascii_case(":END:") {
                return Some((
                    &text[i + 1..],
                    Element::Drawer(Drawer {
                        name: &name[0..name.len() - 1],
                        contents: &text[off..pos],
                    }),
                ));
            }
            pos = i + 1;
        }

        if text[pos..].trim().eq_ignore_ascii_case(":END:") {
            Some((
                "",
                Element::Drawer(Drawer {
                    name: &name[0..name.len() - 1],
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
        Drawer::parse(":PROPERTIES:\n  :CUSTOM_ID: id\n  :END:"),
        Some((
            "",
            Element::Drawer(Drawer {
                name: "PROPERTIES",
                contents: "  :CUSTOM_ID: id\n"
            })
        ))
    )
}
