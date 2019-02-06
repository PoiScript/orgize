use lines::Lines;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Block;

impl Block {
    // return (name, args, contents-begin, contents-end, end)
    pub fn parse(src: &str) -> Option<(&str, Option<&str>, usize, usize, usize)> {
        if src.len() < 17 || !src[0..8].eq_ignore_ascii_case("#+BEGIN_") {
            return None;
        }

        let name = until_while!(src, 8, |c| c == b' ' || c == b'\n', |c: u8| c
            .is_ascii_alphabetic())?;
        let mut lines = Lines::new(src);
        let (pre_cont_end, cont_beg, _) = lines.next()?;
        let args = if pre_cont_end == name {
            None
        } else {
            Some(&src[name..pre_cont_end])
        };
        let name = &src[8..name];
        let end_line = format!(r"#+END_{}", name);
        let mut pre_end = cont_beg;

        while let Some((_, end, line)) = lines.next() {
            if line.trim().eq_ignore_ascii_case(&end_line) {
                return Some((name, args, cont_beg, pre_end, end));
            } else {
                pre_end = end;
            }
        }

        None
    }
}

#[test]
fn parse() {
    assert_eq!(
        Block::parse("#+BEGIN_SRC\n#+END_SRC"),
        Some(("SRC", None, 12, 12, 21))
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
        Some(("SRC", Some(" rust"), 17, 104, 114))
    );
    // TODO: more testing
}
