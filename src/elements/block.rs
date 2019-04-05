use crate::lines::Lines;
use memchr::memchr2;

// return (name, args, contents-begin, contents-end, end)
#[inline]
pub fn parse(src: &str) -> Option<(&str, Option<&str>, usize, usize, usize)> {
    debug_assert!(src.starts_with("#+"));

    if src.len() <= 8 || src[2..8].to_uppercase() != "BEGIN_" {
        return None;
    }

    let name = memchr2(b' ', b'\n', src.as_bytes())
        .filter(|&i| src.as_bytes()[8..i].iter().all(u8::is_ascii_alphabetic))?;
    let mut lines = Lines::new(src);
    let (pre_limit, begin, _) = lines.next()?;
    let args = if pre_limit == name {
        None
    } else {
        Some(&src[name..pre_limit])
    };
    let name = &src[8..name];
    let end_line = format!(r"#+END_{}", name.to_uppercase());
    let mut pre_end = begin;

    for (_, end, line) in lines {
        if line.trim() == end_line {
            return Some((name, args, begin, pre_end, end));
        } else {
            pre_end = end;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(
            parse("#+BEGIN_SRC\n#+END_SRC"),
            Some(("SRC", None, 12, 12, 21))
        );
        assert_eq!(
            parse(
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
}
