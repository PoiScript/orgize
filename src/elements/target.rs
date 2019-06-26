use jetscii::Substring;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Target<'a> {
    pub target: &'a str,
}

impl Target<'_> {
    #[inline]
    // return (target, offset)
    pub(crate) fn parse(text: &str) -> Option<(Target<'_>, usize)> {
        debug_assert!(text.starts_with("<<"));

        let bytes = text.as_bytes();

        Substring::new(">>")
            .find(text)
            .filter(|&i| {
                bytes[2] != b' '
                    && bytes[i - 1] != b' '
                    && bytes[2..i]
                        .iter()
                        .all(|&c| c != b'<' && c != b'\n' && c != b'>')
            })
            .map(|i| {
                (
                    Target {
                        target: &text[2..i],
                    },
                    i + ">>".len(),
                )
            })
    }
}

#[test]
fn parse() {
    assert_eq!(
        Target::parse("<<target>>"),
        Some((Target { target: "target" }, "<<target>>".len()))
    );
    assert_eq!(
        Target::parse("<<tar get>>"),
        Some((Target { target: "tar get" }, "<<tar get>>".len()))
    );
    assert_eq!(Target::parse("<<target >>"), None);
    assert_eq!(Target::parse("<< target>>"), None);
    assert_eq!(Target::parse("<<ta<get>>"), None);
    assert_eq!(Target::parse("<<ta>get>>"), None);
    assert_eq!(Target::parse("<<ta\nget>>"), None);
    assert_eq!(Target::parse("<<target>"), None);
}
