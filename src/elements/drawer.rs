use memchr::memchr_iter;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Drawer<'a> {
    pub name: &'a str,
}

impl<'a> Drawer<'a> {
    #[inline]
    // return (drawer, contents-begin, contents-end , end)
    pub(crate) fn parse(text: &'a str) -> Option<(Drawer<'a>, usize, usize, usize)> {
        debug_assert!(text.starts_with(':'));

        let mut lines = memchr_iter(b'\n', text.as_bytes());

        let (name, off) = lines
            .next()
            .map(|i| (text[1..i].trim_end(), i + 1))
            .filter(|(name, _)| {
                name.ends_with(':')
                    && name[0..name.len() - 1]
                        .as_bytes()
                        .iter()
                        .all(|&c| c.is_ascii_alphabetic() || c == b'-' || c == b'_')
            })?;

        let mut pos = off;
        for i in lines {
            if text[pos..i].trim().eq_ignore_ascii_case(":END:") {
                return Some((
                    Drawer {
                        name: &name[0..name.len() - 1],
                    },
                    off,
                    pos,
                    i + 1,
                ));
            }
            pos = i + 1;
        }

        if text[pos..].trim().eq_ignore_ascii_case(":END:") {
            Some((
                Drawer {
                    name: &name[0..name.len() - 1],
                },
                off,
                pos,
                text.len(),
            ))
        } else {
            None
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Drawer::parse(":PROPERTIES:\n  :CUSTOM_ID: id\n  :END:"),
        Some((
            Drawer { name: "PROPERTIES" },
            ":PROPERTIES:\n".len(),
            ":PROPERTIES:\n  :CUSTOM_ID: id\n".len(),
            ":PROPERTIES:\n  :CUSTOM_ID: id\n  :END:".len()
        ))
    )
}
