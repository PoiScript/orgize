use memchr::{memchr, memchr2};

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Cookie<'a> {
    value: &'a str,
}

impl<'a> Cookie<'a> {
    pub fn parse(src: &'a str) -> Option<(Cookie<'a>, usize)> {
        debug_assert!(src.starts_with('['));

        let num1 = memchr2(b'%', b'/', src.as_bytes())
            .filter(|&i| src.as_bytes()[1..i].iter().all(|c| c.is_ascii_digit()))?;

        if src.as_bytes()[num1] == b'%' && *src.as_bytes().get(num1 + 1)? == b']' {
            Some((
                Cookie {
                    value: &src[0..=num1 + 1],
                },
                num1 + 2,
            ))
        } else {
            let num2 = memchr(b']', src.as_bytes()).filter(|&i| {
                src.as_bytes()[num1 + 1..i]
                    .iter()
                    .all(|c| c.is_ascii_digit())
            })?;

            Some((
                Cookie {
                    value: &src[0..=num2],
                },
                num2 + 1,
            ))
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Cookie::parse("[1/10]").unwrap(),
        (Cookie { value: "[1/10]" }, "[1/10]".len())
    );
    assert_eq!(
        Cookie::parse("[1/1000]").unwrap(),
        (Cookie { value: "[1/1000]" }, "[1/1000]".len())
    );
    assert_eq!(
        Cookie::parse("[10%]").unwrap(),
        (Cookie { value: "[10%]" }, "[10%]".len())
    );
    assert_eq!(
        Cookie::parse("[%]").unwrap(),
        (Cookie { value: "[%]" }, "[%]".len())
    );
    assert_eq!(
        Cookie::parse("[/]").unwrap(),
        (Cookie { value: "[/]" }, "[/]".len())
    );
    assert_eq!(
        Cookie::parse("[100/]").unwrap(),
        (Cookie { value: "[100/]" }, "[100/]".len())
    );
    assert_eq!(
        Cookie::parse("[/100]").unwrap(),
        (Cookie { value: "[/100]" }, "[/100]".len())
    );

    assert!(Cookie::parse("[10% ]").is_none(),);
    assert!(Cookie::parse("[1//100]").is_none(),);
    assert!(Cookie::parse("[1\\100]").is_none(),);
    assert!(Cookie::parse("[10%%]").is_none(),);
}
