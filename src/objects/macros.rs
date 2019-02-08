use jetscii::Substring;
use memchr::memchr2;

/// returns (macros name, macros arguments, offset)
#[inline]
pub fn parse(src: &str) -> Option<(&str, Option<&str>, usize)> {
    debug_assert!(src.starts_with("{{{"));

    expect!(src, 3, |c: u8| c.is_ascii_alphabetic())?;

    let bytes = src.as_bytes();
    let name = memchr2(b'}', b'(', bytes).filter(|&i| {
        bytes[3..i]
            .iter()
            .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    })?;

    Some(if bytes[name] == b'}' {
        expect!(src, name + 1, b'}')?;
        expect!(src, name + 2, b'}')?;
        (&src[3..name], None, name + 3)
    } else {
        let end = Substring::new(")}}}")
            .find(&src[name..])
            .map(|i| i + name)?;
        (
            &src[3..name],
            if name == end {
                None
            } else {
                Some(&src[name + 1..end])
            },
            end + 4,
        )
    })
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
