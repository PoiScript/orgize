use jetscii::Substring;
use memchr::memchr2;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Macros<'a> {
    pub name: &'a str,
    pub args: Option<&'a str>,
}

impl<'a> Macros<'a> {
    pub fn parse(src: &'a str) -> Option<(Macros<'a>, usize)> {
        debug_assert!(src.starts_with("{{{"));

        expect!(src, 3, |c: u8| c.is_ascii_alphabetic())?;

        let bytes = src.as_bytes();
        let name = memchr2(b'}', b'(', bytes).filter(|&i| {
            bytes[3..i]
                .iter()
                .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        })?;

        Some(if bytes[name] == b'}' {
            expect!(src, name + 1, b'}')?;
            expect!(src, name + 2, b'}')?;
            (
                Macros {
                    name: &src[3..name],
                    args: None,
                },
                name + 3,
            )
        } else {
            let end = Substring::new(")}}}")
                .find(&src[name..])
                .map(|i| i + name)?;
            (
                Macros {
                    name: &src[3..name],
                    args: if name == end {
                        None
                    } else {
                        Some(&src[name + 1..end])
                    },
                },
                end + 4,
            )
        })
    }
}

#[test]
fn parse() {
    assert_eq!(
        Macros::parse("{{{poem(red,blue)}}}"),
        Some((
            Macros {
                name: "poem",
                args: Some("red,blue")
            },
            "{{{poem(red,blue)}}}".len()
        ))
    );
    assert_eq!(
        Macros::parse("{{{poem())}}}"),
        Some((
            Macros {
                name: "poem",
                args: Some(")")
            },
            "{{{poem())}}}".len()
        ))
    );
    assert_eq!(
        Macros::parse("{{{author}}}"),
        Some((
            Macros {
                name: "author",
                args: None
            },
            "{{{author}}}".len()
        ))
    );

    assert_eq!(Macros::parse("{{{0uthor}}}"), None);
    assert_eq!(Macros::parse("{{{author}}"), None);
    assert_eq!(Macros::parse("{{{poem(}}}"), None);
    assert_eq!(Macros::parse("{{{poem)}}}"), None);
}
