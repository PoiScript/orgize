use crate::lines::Lines;
use memchr::{memchr, memchr2};

/// return (name, parameters, contents-begin, contents-end, end)
#[inline]
pub fn parse(src: &str) -> Option<(&str, Option<&str>, usize, usize, usize)> {
    debug_assert!(src.starts_with("#+"));

    if src.len() <= 9 || !src[2..9].eq_ignore_ascii_case("BEGIN: ") {
        return None;
    }

    let mut lines = Lines::new(src);
    let (mut pre_cont_end, _, _) = lines.next()?;

    for (cont_end, end, line) in lines {
        if line.trim().eq_ignore_ascii_case("#+END:") {
            let bytes = src.as_bytes();

            let i = memchr2(b' ', b'\n', &bytes[9..])
                .map(|i| i + 9)
                .filter(|&i| bytes[9..i].iter().all(|&c| c.is_ascii_alphabetic()))?;
            let name = &src[8..i].trim();

            return Some(if bytes[i] == b'\n' {
                (name, None, i, pre_cont_end, end)
            } else {
                let cont_beg = memchr(b'\n', bytes)
                    .map(|i| i + 1)
                    .unwrap_or_else(|| src.len());
                (
                    name,
                    Some(&src[i..cont_beg].trim()),
                    cont_beg,
                    pre_cont_end,
                    end,
                )
            });
        }
        pre_cont_end = cont_end;
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        // TODO: testing
        assert_eq!(
            parse(
                r"#+BEGIN: clocktable :scope file
CONTENTS
#+END:
"
            ),
            Some(("clocktable", Some(":scope file"), 32, 40, 48))
        );
    }
}
