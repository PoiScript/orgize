#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Link<'a> {
    pub path: &'a str,
    pub desc: Option<&'a str>,
}

impl<'a> Link<'a> {
    pub fn parse(src: &'a str) -> Option<(Link<'a>, usize)> {
        if cfg!(test) {
            starts_with!(src, "[[");
        }

        let path = until_while!(src, 2, b']', |c| c != b'<' && c != b'>' && c != b'\n')?;

        if cond_eq!(src, path + 1, b']') {
            Some((
                Link {
                    path: &src[2..path],
                    desc: None,
                },
                path + 2,
            ))
        } else if src.as_bytes()[path + 1] == b'[' {
            let desc = until_while!(src, path + 2, b']', |c| c != b'[')?;
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
    assert!(Link::parse("[#id][desc]]").is_none());
}
