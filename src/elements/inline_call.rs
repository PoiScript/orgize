use memchr::{memchr, memchr2};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct InlineCall<'a> {
    pub name: &'a str,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub inside_header: Option<&'a str>,
    pub args: &'a str,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub end_header: Option<&'a str>,
}

impl<'a> InlineCall<'a> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(InlineCall<'_>, usize)> {
        debug_assert!(text.starts_with("call_"));

        let bytes = text.as_bytes();

        let (name, off) = memchr2(b'[', b'(', bytes)
            .map(|i| (&text["call_".len()..i], i))
            .filter(|(name, _)| name.as_bytes().iter().all(u8::is_ascii_graphic))?;

        let (inside_header, off) = if bytes[off] == b'[' {
            memchr(b']', &bytes[off..])
                .filter(|&i| {
                    bytes[off + i + 1] == b'('
                        && bytes[off + 1..off + i].iter().all(|&c| c != b'\n')
                })
                .map(|i| (Some(&text[off + 1..off + i]), off + i + 1))?
        } else {
            (None, off)
        };

        let (args, off) = memchr(b')', &bytes[off..])
            .map(|i| (&text[off + 1..off + i], off + i + 1))
            .filter(|(args, _)| args.as_bytes().iter().all(|&c| c != b'\n'))?;

        let (end_header, off) = if text.len() > off && text.as_bytes()[off] == b'[' {
            memchr(b']', &bytes[off..])
                .filter(|&i| bytes[off + 1..off + i].iter().all(|&c| c != b'\n'))
                .map(|i| (Some(&text[off + 1..off + i]), off + i + 1))?
        } else {
            (None, off)
        };

        Some((
            InlineCall {
                name,
                args,
                inside_header,
                end_header,
            },
            off,
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        InlineCall::parse("call_square(4)"),
        Some((
            InlineCall {
                name: "square",
                args: "4",
                inside_header: None,
                end_header: None,
            },
            "call_square(4)".len()
        ))
    );
    assert_eq!(
        InlineCall::parse("call_square[:results output](4)"),
        Some((
            InlineCall {
                name: "square",
                args: "4",
                inside_header: Some(":results output"),
                end_header: None,
            },
            "call_square[:results output](4)".len()
        ))
    );
    assert_eq!(
        InlineCall::parse("call_square(4)[:results html]"),
        Some((
            InlineCall {
                name: "square",
                args: "4",
                inside_header: None,
                end_header: Some(":results html"),
            },
            "call_square(4)[:results html]".len()
        ))
    );
    assert_eq!(
        InlineCall::parse("call_square[:results output](4)[:results html]"),
        Some((
            InlineCall {
                name: "square",
                args: "4",
                inside_header: Some(":results output"),
                end_header: Some(":results html"),
            },
            "call_square[:results output](4)[:results html]".len()
        ))
    );
}
