use memchr::{memchr, memchr2};

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum Cookie<'a> {
    Percent(&'a str),
    Slash(&'a str, &'a str),
}

#[inline]
pub fn parse(src: &str) -> Option<(Cookie<'_>, usize)> {
    debug_assert!(src.starts_with('['));

    let bytes = src.as_bytes();
    let num1 =
        memchr2(b'%', b'/', bytes).filter(|&i| bytes[1..i].iter().all(|c| c.is_ascii_digit()))?;

    if bytes[num1] == b'%' && *bytes.get(num1 + 1)? == b']' {
        Some((Cookie::Percent(&src[1..num1]), num1 + 2))
    } else {
        let num2 = memchr(b']', bytes)
            .filter(|&i| bytes[num1 + 1..i].iter().all(|c| c.is_ascii_digit()))?;

        Some((Cookie::Slash(&src[1..num1], &src[num1 + 1..num2]), num2 + 1))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;
        use super::Cookie::*;

        assert_eq!(parse("[1/10]"), Some((Slash("1", "10"), "[1/10]".len())));
        assert_eq!(
            parse("[1/1000]"),
            Some((Slash("1", "1000"), "[1/1000]".len()))
        );
        assert_eq!(parse("[10%]"), Some((Percent("10"), "[10%]".len())));
        assert_eq!(parse("[%]"), Some((Percent(""), "[%]".len())));
        assert_eq!(parse("[/]"), Some((Slash("", ""), "[/]".len())));
        assert_eq!(parse("[100/]"), Some((Slash("100", ""), "[100/]".len())));
        assert_eq!(parse("[/100]"), Some((Slash("", "100"), "[/100]".len())));

        assert_eq!(parse("[10% ]"), None);
        assert_eq!(parse("[1//100]"), None);
        assert_eq!(parse("[1\\100]"), None);
        assert_eq!(parse("[10%%]"), None);
    }
}
