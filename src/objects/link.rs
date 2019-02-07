use memchr::memchr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Link<'a> {
    pub path: &'a str,
    pub desc: Option<&'a str>,
}

impl<'a> Link<'a> {
    pub fn parse(src: &'a str) -> Option<(Link<'a>, usize)> {
        debug_assert!(src.starts_with("[["));

        let bytes = src.as_bytes();
        let path = memchr(b']', bytes).filter(|&i| {
            bytes[2..i]
                .iter()
                .all(|&c| c != b'<' && c != b'>' && c != b'\n')
        })?;

        if *bytes.get(path + 1)? == b']' {
            Some((
                Link {
                    path: &src[2..path],
                    desc: None,
                },
                path + 2,
            ))
        } else if bytes[path + 1] == b'[' {
            let desc = memchr(b']', &bytes[path + 2..])
                .map(|i| i + path + 2)
                .filter(|&i| bytes[path + 2..i].iter().all(|&c| c != b'['))?;
            expect!(src, desc + 1, b']')?;

            Some((
                Link {
                    path: &src[2..path],
                    desc: Some(&src[path + 2..desc]),
                },
                desc + 2,
            ))
        } else {
            None
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Link::parse("[[#id]]").unwrap(),
        (
            Link {
                path: "#id",
                desc: None,
            },
            "[[#id]]".len()
        )
    );
    assert_eq!(
        Link::parse("[[#id][desc]]").unwrap(),
        (
            Link {
                path: "#id",
                desc: Some("desc"),
            },
            "[[#id][desc]]".len()
        )
    );
    assert!(Link::parse("[[#id][desc]").is_none());
}
