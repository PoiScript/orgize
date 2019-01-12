#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct DynBlock;

impl DynBlock {
    // return (name, parameters, contents-begin, contents-end, end)
    pub fn parse(src: &str) -> Option<(&str, Option<&str>, usize, usize, usize)> {
        if src.len() < 17 || !src[0..9].eq_ignore_ascii_case("#+BEGIN: ") {
            return None;
        }

        let args = eol!(src);
        let name = until_while!(src, 9, |c| c == b' ' || c == b'\n', |c: u8| c
            .is_ascii_alphabetic());
        // TODO: ignore case matching
        let content = src.find("\n#+END:")?;
        let end = eol!(src, content + 1);

        Some((
            &src[9..name],
            if name == args {
                None
            } else {
                Some(&src[name..args].trim())
            },
            args,
            content + 1,
            end + 1,
        ))
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
        Some(("clocktable", Some(":scope file"), 31, 41, 48))
    )
}
