#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct FnRef<'a> {
    label: Option<&'a str>,
    definition: Option<&'a str>,
}

fn valid_label(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'-' || ch == b'_'
}

impl<'a> FnRef<'a> {
    pub fn parse(src: &'a str) -> Option<(FnRef<'a>, usize)> {
        starts_with!(src, "[fn:");

        let label = until_while!(src, 4, |c| c == b']' || c == b':', valid_label);

        if src.as_bytes()[label] == b':' {
            let mut pairs = 1;
            let def = until!(src[label..], |c| {
                if c == b'[' {
                    pairs += 1;
                } else if c == b']' {
                    pairs -= 1;
                }
                c == b']' && pairs == 0
            })? + label;

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
