use memchr::{memchr2, memchr2_iter};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct FnRef<'a> {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<&'a str>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub definition: Option<&'a str>,
}

impl FnRef<'_> {
    #[inline]
    // return (fn_ref, offset)
    pub fn parse(text: &str) -> Option<(FnRef<'_>, usize)> {
        debug_assert!(text.starts_with("[fn:"));

        let bytes = text.as_bytes();
        let (label, off) = memchr2(b']', b':', &bytes["[fn:".len()..])
            .filter(|&i| {
                bytes["[fn:".len().."[fn:".len() + i]
                    .iter()
                    .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
            })
            .map(|i| {
                (
                    if i == 0 {
                        None
                    } else {
                        Some(&text["[fn:".len().."[fn:".len() + i])
                    },
                    "[fn:".len() + i,
                )
            })?;

        let (definition, off) = if bytes[off] == b':' {
            let mut pairs = 1;
            memchr2_iter(b'[', b']', &bytes[off..])
                .find(|&i| {
                    if bytes[i + off] == b'[' {
                        pairs += 1;
                    } else {
                        pairs -= 1;
                    }
                    pairs == 0
                })
                .map(|i| (Some(&text[off + 1..off + i]), i + off + 1))?
        } else {
            (None, off + 1)
        };

        Some((FnRef { label, definition }, off))
    }
}

#[test]
fn parse() {
    assert_eq!(
        FnRef::parse("[fn:1]"),
        Some((
            FnRef {
                label: Some("1"),
                definition: None
            },
            "[fn:1]".len()
        ))
    );
    assert_eq!(
        FnRef::parse("[fn:1:2]"),
        Some((
            FnRef {
                label: Some("1"),
                definition: Some("2")
            },
            "[fn:1:2]".len()
        ))
    );
    assert_eq!(
        FnRef::parse("[fn::2]"),
        Some((
            FnRef {
                label: None,
                definition: Some("2")
            },
            "[fn::2]".len()
        ))
    );
    assert_eq!(
        FnRef::parse("[fn::[]]"),
        Some((
            FnRef {
                label: None,
                definition: Some("[]")
            },
            "[fn::[]]".len()
        ))
    );
    assert_eq!(FnRef::parse("[fn::[]"), None);
}
