use memchr::{memchr, memchr2};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Cookie<'a> {
    pub value: &'a str,
}

impl Cookie<'_> {
    #[inline]
    pub(crate) fn parse(src: &str) -> Option<(&str, Cookie<'_>)> {
        debug_assert!(src.starts_with('['));

        let bytes = src.as_bytes();
        let num1 =
            memchr2(b'%', b'/', bytes).filter(|&i| bytes[1..i].iter().all(u8::is_ascii_digit))?;

        if bytes[num1] == b'%' && *bytes.get(num1 + 1)? == b']' {
            Some((
                &src[num1 + 2..],
                Cookie {
                    value: &src[0..num1 + 2],
                },
            ))
        } else {
            let num2 = memchr(b']', bytes)
                .filter(|&i| bytes[num1 + 1..i].iter().all(u8::is_ascii_digit))?;

            Some((
                &src[num2 + 1..],
                Cookie {
                    value: &src[0..num2 + 1],
                },
            ))
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Cookie::parse("[1/10]"),
        Some(("", Cookie { value: "[1/10]" }))
    );
    assert_eq!(
        Cookie::parse("[1/1000]"),
        Some(("", Cookie { value: "[1/1000]" }))
    );
    assert_eq!(
        Cookie::parse("[10%]"),
        Some(("", Cookie { value: "[10%]" }))
    );
    assert_eq!(Cookie::parse("[%]"), Some(("", Cookie { value: "[%]" })));
    assert_eq!(Cookie::parse("[/]"), Some(("", Cookie { value: "[/]" })));
    assert_eq!(
        Cookie::parse("[100/]"),
        Some(("", Cookie { value: "[100/]" }))
    );
    assert_eq!(
        Cookie::parse("[/100]"),
        Some(("", Cookie { value: "[/100]" }))
    );

    assert_eq!(Cookie::parse("[10% ]"), None);
    assert_eq!(Cookie::parse("[1//100]"), None);
    assert_eq!(Cookie::parse("[1\\100]"), None);
    assert_eq!(Cookie::parse("[10%%]"), None);
}
