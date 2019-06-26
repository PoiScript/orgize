use jetscii::Substring;
use memchr::memchr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Link<'a> {
    pub path: &'a str,
    pub desc: Option<&'a str>,
}

impl Link<'_> {
    #[inline]
    // return (link, offset)
    pub(crate) fn parse(text: &str) -> Option<(Link<'_>, usize)> {
        debug_assert!(text.starts_with("[["));

        let (path, off) = memchr(b']', text.as_bytes())
            .map(|i| (&text["[[".len()..i], i))
            .filter(|(path, _)| {
                path.as_bytes()
                    .iter()
                    .all(|&c| c != b'<' && c != b'>' && c != b'\n')
            })?;

        if *text.as_bytes().get(off + 1)? == b']' {
            Some((Link { path, desc: None }, off + 2))
        } else if text.as_bytes()[off + 1] == b'[' {
            let (desc, off) = Substring::new("]]")
                .find(&text[off + 1..])
                .map(|i| (&text[off + 2..off + 1 + i], off + 1 + i + "]]".len()))
                .filter(|(desc, _)| desc.as_bytes().iter().all(|&c| c != b'[' && c != b']'))?;
            Some((
                Link {
                    path,
                    desc: Some(desc),
                },
                off,
            ))
        } else {
            None
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Link::parse("[[#id]]"),
        Some((
            Link {
                path: "#id",
                desc: None
            },
            "[[#id]]".len()
        ))
    );
    assert_eq!(
        Link::parse("[[#id][desc]]"),
        Some((
            Link {
                path: "#id",
                desc: Some("desc")
            },
            "[[#id][desc]]".len()
        ))
    );
    assert_eq!(Link::parse("[[#id][desc]"), None);
}
