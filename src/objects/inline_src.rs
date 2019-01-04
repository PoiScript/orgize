#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct InlineSrc<'a> {
    pub lang: &'a str,
    pub option: Option<&'a str>,
    pub body: &'a str,
}

impl<'a> InlineSrc<'a> {
    pub fn parse(src: &'a str) -> Option<(InlineSrc, usize)> {
        starts_with!(src, "src_");

        let lang = until_while!(src, 4, |c| c == b'[' || c == b'{', |c: u8| !c
            .is_ascii_whitespace());

        if lang == 4 {
            return None;
        }

        if src.as_bytes()[lang] == b'[' {
            let option = until_while!(src, lang, b']', |c| c != b'\n');
            let body = until_while!(src, option, b'}', |c| c != b'\n');

            Some((
                InlineSrc {
                    lang: &src[4..lang],
                    option: Some(&src[lang + 1..option]),
                    body: &src[option + 2..body],
                },
                body + 1,
            ))
        } else {
            let body = until_while!(src, lang, b'}', |c| c != b'\n');

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
