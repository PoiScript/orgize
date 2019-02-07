use crate::lines::Lines;
use memchr::memchr2;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct DynBlock;

impl DynBlock {
    // return (name, parameters, contents-begin, contents-end, end)
    pub fn parse(src: &str) -> Option<(&str, Option<&str>, usize, usize, usize)> {
        debug_assert!(src.starts_with("#+"));

        if !src[2..9].eq_ignore_ascii_case("BEGIN: ") {
            return None;
        }

        let bytes = src.as_bytes();
        let args = eol!(src);
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
}

#[test]
fn parse() {
    // TODO: testing
    assert_eq!(
        DynBlock::parse(
            r"#+BEGIN: clocktable :scope file
CONTENTS
#+END:
"
        ),
        Some(("clocktable", Some(":scope file"), 31, 40, 48))
    )
}
