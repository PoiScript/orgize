use jetscii::Substring;
use memchr::memchr2;

/// returns (macros name, macros arguments, offset)
#[inline]
pub fn parse(text: &str) -> Option<(&str, Option<&str>, usize)> {
    debug_assert!(text.starts_with("{{{"));

    expect!(text, 3, |c: u8| c.is_ascii_alphabetic())?;

    let bytes = text.as_bytes();
    let (name, off) = memchr2(b'}', b'(', bytes)
        .filter(|&i| {
            bytes[3..i]
                .iter()
                .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        })
        .map(|i| (&text[3..i], i))?;

    let (args, off) = if bytes[off] == b'}' {
        expect!(text, off + 1, b'}')?;
        expect!(text, off + 2, b'}')?;
        (None, off + 3 /* }}} */)
    } else {
        Substring::new(")}}}")
            .find(&text[off..])
            .map(|i| (Some(&text[off + 1..off + i]), off + i + 4 /* )}}} */))?
    };

    Some((name, args, off))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(
            parse("{{{poem(red,blue)}}}"),
            Some(("poem", Some("red,blue"), "{{{poem(red,blue)}}}".len()))
        );
        assert_eq!(
            parse("{{{poem())}}}"),
            Some(("poem", Some(")"), "{{{poem())}}}".len()))
        );
        assert_eq!(
            parse("{{{author}}}"),
            Some(("author", None, "{{{author}}}".len()))
        );

        assert_eq!(parse("{{{0uthor}}}"), None);
        assert_eq!(parse("{{{author}}"), None);
        assert_eq!(parse("{{{poem(}}}"), None);
        assert_eq!(parse("{{{poem)}}}"), None);
    }
}
