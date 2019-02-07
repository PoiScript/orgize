use memchr::{memchr, memchr2};

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct InlineSrc<'a> {
    pub lang: &'a str,
    pub option: Option<&'a str>,
    pub body: &'a str,
}

impl<'a> InlineSrc<'a> {
    pub fn parse(src: &'a str) -> Option<(InlineSrc, usize)> {
        debug_assert!(src.starts_with("src_"));

        let bytes = src.as_bytes();
        let lang = memchr2(b'[', b'{', bytes)
            .filter(|&i| i != 4 && bytes[4..i].iter().all(|c| !c.is_ascii_whitespace()))?;

        if bytes[lang] == b'[' {
            let option =
                memchr(b']', bytes).filter(|&i| bytes[lang..i].iter().all(|c| *c != b'\n'))?;
            let body = memchr(b'}', &bytes[option..])
                .map(|i| i + option)
                .filter(|&i| bytes[option..i].iter().all(|c| *c != b'\n'))?;

            Some((
                InlineSrc {
                    lang: &src[4..lang],
                    option: Some(&src[lang + 1..option]),
                    body: &src[option + 2..body],
                },
                body + 1,
            ))
        } else {
            let body =
                memchr(b'}', bytes).filter(|&i| bytes[lang..i].iter().all(|c| *c != b'\n'))?;

            Some((
                InlineSrc {
                    lang: &src[4..lang],
                    option: None,
                    body: &src[lang + 1..body],
                },
                body + 1,
            ))
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        InlineSrc::parse("src_C{int a = 0;}").unwrap(),
        (
            InlineSrc {
                lang: "C",
                option: None,
                body: "int a = 0;"
            },
            "src_C{int a = 0;}".len()
        )
    );
    assert_eq!(
        InlineSrc::parse("src_xml[:exports code]{<tag>text</tag>}").unwrap(),
        (
            InlineSrc {
                lang: "xml",
                option: Some(":exports code"),
                body: "<tag>text</tag>"
            },
            "src_xml[:exports code]{<tag>text</tag>}".len()
        )
    );
    assert!(InlineSrc::parse("src_xml[:exports code]{<tag>text</tag>").is_none());
    assert!(InlineSrc::parse("src_[:exports code]{<tag>text</tag>}").is_none());
    assert!(InlineSrc::parse("src_xml[:exports code]").is_none());
}
