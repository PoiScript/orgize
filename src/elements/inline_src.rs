use memchr::{memchr, memchr2};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct InlineSrc<'a> {
    pub lang: &'a str,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub option: Option<&'a str>,
    pub body: &'a str,
}

impl<'a> InlineSrc<'a> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(InlineSrc<'_>, usize)> {
        debug_assert!(text.starts_with("src_"));

        let (lang, off) = memchr2(b'[', b'{', text.as_bytes())
            .map(|i| (&text["src_".len()..i], i))
            .filter(|(lang, off)| {
                *off != 4 && lang.as_bytes().iter().all(|c| !c.is_ascii_whitespace())
            })?;

        let (option, off) = if text.as_bytes()[off] == b'[' {
            memchr(b']', text[off..].as_bytes())
                .filter(|&i| text[off..off + i].as_bytes().iter().all(|c| *c != b'\n'))
                .map(|i| (Some(&text[off + 1..off + i]), off + i + 1))?
        } else {
            (None, off)
        };

        let (body, off) = memchr(b'}', text[off..].as_bytes())
            .map(|i| (&text[off + 1..off + i], off + i + 1))
            .filter(|(body, _)| body.as_bytes().iter().all(|c| *c != b'\n'))?;

        Some((InlineSrc { lang, option, body }, off))
    }
}

#[test]
fn parse() {
    assert_eq!(
        InlineSrc::parse("src_C{int a = 0;}"),
        Some((
            InlineSrc {
                lang: "C",
                option: None,
                body: "int a = 0;"
            },
            "src_C{int a = 0;}".len()
        ))
    );
    assert_eq!(
        InlineSrc::parse("src_xml[:exports code]{<tag>text</tag>}"),
        Some((
            InlineSrc {
                lang: "xml",
                option: Some(":exports code"),
                body: "<tag>text</tag>",
            },
            "src_xml[:exports code]{<tag>text</tag>}".len()
        ))
    );
    assert_eq!(
        InlineSrc::parse("src_xml[:exports code]{<tag>text</tag>"),
        None
    );
    assert_eq!(
        InlineSrc::parse("src_[:exports code]{<tag>text</tag>}"),
        None
    );
    // assert_eq!(parse("src_xml[:exports code]"), None);
}
