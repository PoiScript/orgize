use memchr::{memchr2, memchr2_iter};

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct FnRef<'a> {
    label: Option<&'a str>,
    definition: Option<&'a str>,
}

fn valid_label(ch: &u8) -> bool {
    ch.is_ascii_alphanumeric() || *ch == b'-' || *ch == b'_'
}

impl<'a> FnRef<'a> {
    pub fn parse(src: &'a str) -> Option<(FnRef<'a>, usize)> {
        debug_assert!(src.starts_with("[fn:"));

        let bytes = src.as_bytes();
        let label = memchr2(b']', b':', &bytes[4..])
            .map(|i| i + 4)
            .filter(|&i| bytes[4..i].iter().all(valid_label))?;

        if bytes[label] == b':' {
            let mut pairs = 1;
            let def = memchr2_iter(b'[', b']', &bytes[label..])
                .map(|i| i + label)
                .filter(|&i| {
                    if bytes[i] == b'[' {
                        pairs += 1;
                    } else {
                        pairs -= 1;
                    }
                    pairs == 0
                })
                .next()?;

            Some((
                FnRef {
                    label: if label == 4 {
                        None
                    } else {
                        Some(&src[4..label])
                    },
                    definition: Some(&src[label + 1..def]),
                },
                def + 1,
            ))
        } else {
            Some((
                FnRef {
                    label: if label == 4 {
                        None
                    } else {
                        Some(&src[4..label])
                    },
                    definition: None,
                },
                label + 1,
            ))
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        FnRef::parse("[fn:1]").unwrap(),
        (
            FnRef {
                label: Some("1"),
                definition: None,
            },
            "[fn:1]".len()
        )
    );
    assert_eq!(
        FnRef::parse("[fn:1:2]").unwrap(),
        (
            FnRef {
                label: Some("1"),
                definition: Some("2"),
            },
            "[fn:1:2]".len()
        )
    );
    assert_eq!(
        FnRef::parse("[fn::2]").unwrap(),
        (
            FnRef {
                label: None,
                definition: Some("2"),
            },
            "[fn::2]".len()
        )
    );
    assert_eq!(
        FnRef::parse("[fn::[]]").unwrap(),
        (
            FnRef {
                label: None,
                definition: Some("[]"),
            },
            "[fn::[]]".len()
        )
    );
    assert!(FnRef::parse("[fn::[]").is_none());
}
