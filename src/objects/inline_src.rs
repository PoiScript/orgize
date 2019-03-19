use memchr::{memchr, memchr2};

/// returns (language, option, body, offset)
#[inline]
pub fn parse(text: &str) -> Option<(&str, Option<&str>, &str, usize)> {
    debug_assert!(text.starts_with("src_"));

    let (lang, off) = memchr2(b'[', b'{', text.as_bytes())
        .map(|i| (&text[4..i], i))
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

    Some((lang, option, body, off))
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
        // assert_eq!(parse("src_xml[:exports code]"), None);
    }
}
