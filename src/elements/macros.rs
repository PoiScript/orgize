use jetscii::Substring;
use memchr::memchr2;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Macros<'a> {
    pub name: &'a str,
    pub arguments: Option<&'a str>,
}

impl Macros<'_> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(Macros<'_>, usize)> {
        debug_assert!(text.starts_with("{{{"));

        let bytes = text.as_bytes();
        if text.len() <= 3 || !bytes[3].is_ascii_alphabetic() {
            return None;
        }

        let (name, off) = memchr2(b'}', b'(', bytes)
            .filter(|&i| {
                bytes[3..i]
                    .iter()
                    .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
            })
            .map(|i| (&text[3..i], i))?;

        let (arguments, off) = if bytes[off] == b'}' {
            if text.len() <= off + 2 || bytes[off + 1] != b'}' || bytes[off + 2] != b'}' {
                return None;
            }
            (None, off + "}}}".len())
        } else {
            Substring::new(")}}}")
                .find(&text[off..])
                .map(|i| (Some(&text[off + 1..off + i]), off + i + ")}}}".len()))?
        };

        Some((Macros { name, arguments }, off))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Macros::parse("{{{poem(red,blue)}}}"),
        Some((
            Macros {
                name: "poem",
                arguments: Some("red,blue")
            },
            "{{{poem(red,blue)}}}".len()
        ))
    );
    assert_eq!(
        Macros::parse("{{{poem())}}}"),
        Some((
            Macros {
                name: "poem",
                arguments: Some(")")
            },
            "{{{poem())}}}".len()
        ))
    );
    assert_eq!(
        Macros::parse("{{{author}}}"),
        Some((
            Macros {
                name: "author",
                arguments: None
            },
            "{{{author}}}".len()
        ))
    );
    assert_eq!(Macros::parse("{{{0uthor}}}"), None);
    assert_eq!(Macros::parse("{{{author}}"), None);
    assert_eq!(Macros::parse("{{{poem(}}}"), None);
    assert_eq!(Macros::parse("{{{poem)}}}"), None);
}
