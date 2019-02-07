use jetscii::Substring;
use memchr::memchr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Snippet<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

impl<'a> Snippet<'a> {
    pub fn parse(src: &'a str) -> Option<(Snippet<'a>, usize)> {
        debug_assert!(src.starts_with("@@"));

        let name = memchr(b':', src.as_bytes()).filter(|&i| {
            i != 2
                && src.as_bytes()[2..i]
                    .iter()
                    .all(|&c| c.is_ascii_alphanumeric() || c == b'-')
        })?;

        let end = Substring::new("@@")
            .find(&src[name + 1..])
            .map(|i| i + name + 1)?;

        Some((
            Snippet {
                name: &src[2..name],
                value: &src[name + 1..end],
            },
            end + 2,
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Snippet::parse("@@html:<b>@@").unwrap(),
        (
            Snippet {
                name: "html",
                value: "<b>"
            },
            "@@html:<b>@@".len()
        )
    );
    assert_eq!(
        Snippet::parse("@@latex:any arbitrary LaTeX code@@").unwrap(),
        (
            Snippet {
                name: "latex",
                value: "any arbitrary LaTeX code"
            },
            "@@latex:any arbitrary LaTeX code@@".len()
        )
    );
    assert_eq!(
        Snippet::parse("@@html:@@").unwrap(),
        (
            Snippet {
                name: "html",
                value: ""
            },
            "@@html:@@".len()
        )
    );
    assert!(Snippet::parse("@@html:<b>@").is_none());
    assert!(Snippet::parse("@@html<b>@@").is_none());
    assert!(Snippet::parse("@@:<b>@@").is_none());
}
