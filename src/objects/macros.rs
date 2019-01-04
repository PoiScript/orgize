#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Macros<'a> {
    pub name: &'a str,
    pub args: Option<&'a str>,
}

fn valid_name(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'-' && ch == b'_'
}

impl<'a> Macros<'a> {
    pub fn parse(src: &'a str) -> Option<(Macros<'a>, usize)> {
        starts_with!(src, "{{{");

        expect!(src, 3, |c: u8| c.is_ascii_alphabetic());

        let name = until_while!(src, 3, |c| c == b'}' || c == b'(', valid_name);

        if src.as_bytes()[name] == b'}' {
            expect!(src, name + 1, b'}');
            expect!(src, name + 2, b'}');
            Some((
                Macros {
                    name: &src[3..name],
                    args: None,
                },
                name + 3,
            ))
        } else {
            let end = find!(src, name, "}}}");
            expect!(src, end - 1, b')');
            Some((
                Macros {
                    name: &src[3..name],
                    args: if name == end {
                        None
                    } else {
                        Some(&src[name + 1..end - 1])
                    },
                },
                end + 3,
            ))
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Macros::parse("{{{poem(red,blue)}}}").unwrap(),
        (
            Macros {
                name: "poem",
                args: Some("red,blue")
            },
            "{{{poem(red,blue)}}}".len()
        )
    );
    assert_eq!(
        Macros::parse("{{{author}}}").unwrap(),
        (
            Macros {
                name: "author",
                args: None,
            },
            "{{{author}}}".len()
        )
    );
    assert!(Macros::parse("{{author}}}").is_none());
    assert!(Macros::parse("{{{0uthor}}}").is_none());
    assert!(Macros::parse("{{{author}}").is_none());
    assert!(Macros::parse("{{{poem(}}}").is_none());
    assert!(Macros::parse("{{{poem)}}}").is_none());
    // FIXME: assert_eq!(Macros::parse("{{{poem())}}}"), None);
}
