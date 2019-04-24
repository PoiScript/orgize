use jetscii::Substring;
use memchr::memchr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Snippet<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

impl<'a> Snippet<'a> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(Snippet<'_>, usize)> {
        debug_assert!(text.starts_with("@@"));

        let (name, off) = memchr(b':', text.as_bytes())
            .filter(|&i| {
                i != 2
                    && text.as_bytes()[2..i]
                        .iter()
                        .all(|&c| c.is_ascii_alphanumeric() || c == b'-')
            })
            .map(|i| (&text[2..i], i + 1))?;

        let (value, off) = Substring::new("@@")
            .find(&text[off..])
            .map(|i| (&text[off..off + i], off + i + "@@".len()))?;

        Some((Snippet { name, value }, off))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Snippet::parse("@@html:<b>@@"),
        Some((
            Snippet {
                name: "html",
                value: "<b>"
            },
            "@@html:<b>@@".len()
        ))
    );
    assert_eq!(
        Snippet::parse("@@latex:any arbitrary LaTeX code@@"),
        Some((
            Snippet {
                name: "latex",
                value: "any arbitrary LaTeX code",
            },
            "@@latex:any arbitrary LaTeX code@@".len()
        ))
    );
    assert_eq!(
        Snippet::parse("@@html:@@"),
        Some((
            Snippet {
                name: "html",
                value: "",
            },
            "@@html:@@".len()
        ))
    );
    assert_eq!(Snippet::parse("@@html:<b>@"), None);
    assert_eq!(Snippet::parse("@@html<b>@@"), None);
    assert_eq!(Snippet::parse("@@:<b>@@"), None);
}
