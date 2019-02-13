use memchr::{memchr, memchr2};

/// returns (language, option, body, offset)
#[inline]
pub fn parse(src: &str) -> Option<(&str, Option<&str>, &str, usize)> {
    debug_assert!(src.starts_with("src_"));

    let bytes = src.as_bytes();
    let lang = memchr2(b'[', b'{', bytes)
        .filter(|&i| i != 4 && bytes[4..i].iter().all(|c| !c.is_ascii_whitespace()))?;

    if bytes[lang] == b'[' {
        let option = memchr(b']', bytes).filter(|&i| bytes[lang..i].iter().all(|c| *c != b'\n'))?;
        let body = memchr(b'}', &bytes[option..])
            .map(|i| i + option)
            .filter(|&i| bytes[option..i].iter().all(|c| *c != b'\n'))?;

        Some((
            &src[4..lang],
            Some(&src[lang + 1..option]),
            &src[option + 2..body],
            body + 1,
        ))
    } else {
        let body = memchr(b'}', bytes).filter(|&i| bytes[lang..i].iter().all(|c| *c != b'\n'))?;

        Some((&src[4..lang], None, &src[lang + 1..body], body + 1))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(
            parse("src_C{int a = 0;}"),
            Some(("C", None, "int a = 0;", "src_C{int a = 0;}".len()))
        );
        assert_eq!(
            parse("src_xml[:exports code]{<tag>text</tag>}"),
            Some((
                "xml",
                Some(":exports code"),
                "<tag>text</tag>",
                "src_xml[:exports code]{<tag>text</tag>}".len()
            ))
        );
        assert_eq!(parse("src_xml[:exports code]{<tag>text</tag>"), None);
        assert_eq!(parse("src_[:exports code]{<tag>text</tag>}"), None);
        assert_eq!(parse("src_xml[:exports code]"), None);
    }
}
