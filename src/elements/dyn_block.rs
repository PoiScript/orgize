use crate::lines::Lines;
use memchr::memchr2;

/// return (name, parameters, contents-begin, contents-end, end)
#[inline]
pub fn parse(src: &str) -> Option<(&str, Option<&str>, usize, usize, usize)> {
    debug_assert!(src.starts_with("#+"));

    if src.len() <= 9 || !src[2..9].eq_ignore_ascii_case("BEGIN: ") {
        return None;
    }

    let bytes = src.as_bytes();

    let args = memchr::memchr(b'\n', src.as_bytes())
        .map(|i| i + 1)
        .unwrap_or_else(|| src.len());
    let name = memchr2(b' ', b'\n', &bytes[9..])
        .map(|i| i + 9)
        .filter(|&i| {
            src.as_bytes()[9..i]
                .iter()
                .all(|&c| c.is_ascii_alphabetic())
        })?;
    let mut lines = Lines::new(src);
    let (mut pre_cont_end, _, _) = lines.next()?;

    for (cont_end, end, line) in lines {
        if line.trim().eq_ignore_ascii_case("#+END:") {
            return Some((
                &src[8..name].trim(),
                if name == args {
                    None
                } else {
                    Some(&src[name..args].trim())
                },
                args,
                pre_cont_end,
                end,
            ));
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
