use jetscii::Substring;

// TODO: text-markup, entities, latex-fragments, subscript and superscript
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct RadioTarget<'a> {
    contents: &'a str,
}

impl RadioTarget<'_> {
    #[inline]
    // return (radio_target, offset)
    pub(crate) fn parse(src: &str) -> Option<(RadioTarget<'_>, usize)> {
        debug_assert!(src.starts_with("<<<"));

        let bytes = src.as_bytes();
        let (contents, off) = Substring::new(">>>")
            .find(src)
            .filter(|&i| {
                bytes[3] != b' '
                    && bytes[i - 1] != b' '
                    && bytes[3..i]
                        .iter()
                        .all(|&c| c != b'<' && c != b'\n' && c != b'>')
            })
            .map(|i| (&src[3..i], i + ">>>".len()))?;

        Some((RadioTarget { contents }, off))
    }
}

#[test]
fn parse() {
    assert_eq!(
        RadioTarget::parse("<<<target>>>"),
        Some((RadioTarget { contents: "target" }, "<<<target>>>".len()))
    );
    assert_eq!(
        RadioTarget::parse("<<<tar get>>>"),
        Some((
            RadioTarget {
                contents: "tar get"
            },
            "<<<tar get>>>".len()
        ))
    );
    assert_eq!(RadioTarget::parse("<<<target >>>"), None);
    assert_eq!(RadioTarget::parse("<<< target>>>"), None);
    assert_eq!(RadioTarget::parse("<<<ta<get>>>"), None);
    assert_eq!(RadioTarget::parse("<<<ta>get>>>"), None);
    assert_eq!(RadioTarget::parse("<<<ta\nget>>>"), None);
    assert_eq!(RadioTarget::parse("<<<target>>"), None);
}
