use regex::Regex;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Block;

impl Block {
    // return (name, args, contents-begin, contents-end, end)
    pub fn parse(src: &str) -> Option<(&str, Option<&str>, usize, usize, usize)> {
        if src.len() < 17 || !src[0..8].eq_ignore_ascii_case("#+BEGIN_") {
            return None;
        }

        let args = eol!(src);
        let name = until_while!(src, 8, |c| c == b' ' || c == b'\n', |c: u8| c
            .is_ascii_alphabetic())?;
        let end_re = format!(r"(?im)^[ \t]*#\+END_{}[ \t]*$", &src[8..name]);
        let end_re = Regex::new(&end_re).unwrap();
        let (content, end) = end_re.find(src).map(|m| (m.start(), m.end()))?;

        Some((
            &src[8..name],
            if name == args {
                None
            } else {
                Some(&src[name..args])
            },
            args,
            content,
            // including the eol character
            if end < src.len() && src.as_bytes()[end] == b'\n' {
                end + 1
            } else {
                end
            },
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Block::parse("#+BEGIN_SRC\n#+END_SRC"),
        Some(("SRC", None, 11, 12, 21))
    );
    assert_eq!(
        Block::parse(
            r#"#+BEGIN_SRC rust
fn main() {
    // print "Hello World!" to the console
    println!("Hello World!");
}
#+END_SRC
"#
        ),
        Some(("SRC", Some(" rust"), 16, 104, 114))
    );
    // TODO: more testing
}
