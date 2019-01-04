#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Snippet<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

impl<'a> Snippet<'a> {
    pub fn parse(src: &'a str) -> Option<(Snippet<'a>, usize)> {
        starts_with!(src, "@@");

        let name = until_while!(src, 2, b':', |c: u8| c.is_ascii_alphanumeric() || c == b'-');

        if name == 2 {
            return None;
        }

        let end = find!(src, name + 1, "@@");

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
    assert!(Snippet::parse("@html:<b>@@").is_none());
    assert!(Snippet::parse("@@html<b>@@").is_none());
    assert!(Snippet::parse("@@:<b>@@").is_none());
}
