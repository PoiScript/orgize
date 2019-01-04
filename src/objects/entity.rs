pub struct Entity<'a> {
    pub name: &'a str,
    pub contents: Option<&'a str>,
}

impl<'a> Entity<'a> {
    pub fn parse(src: &'a str) -> Option<(Entity<'a>, usize)> {
        expect!(src, 0, b'\\');

        let name = position!(src, 1, |c| !c.is_ascii_alphabetic());

        if src.as_bytes()[name] == b'[' {
            Some((
                Entity {
                    name: &src[1..name],
                    contents: None,
                },
                name,
            ))
        } else if src.as_bytes()[name] == b'{' {
            Some((
                Entity {
                    name: &src[1..name],
                    contents: None,
                },
                name,
            ))
        } else {
            Some((
                Entity {
                    name: &src[1..name],
                    contents: None,
                },
                name,
            ))
        }
    }
}
