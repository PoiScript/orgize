#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct DynBlock<'a> {
    pub name: &'a str,
    pub para: &'a str,
}

impl<'a> DynBlock<'a> {
    pub fn parse(src: &'a str) -> Option<(DynBlock<'a>, usize, usize)> {
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
            DynBlock {
                name: &src[9..name],
                para: &src[name..args].trim(),
            },
            content,
            end,
        ))
    }
}

#[test]
fn parse() {
    // TODO: testing
    assert_eq!(
        DynBlock::parse(
            r"#+BEGIN: clocktable :scope file :block yesterday
CONTENTS
#+END:
"
        ),
        Some((
            DynBlock {
                name: "clocktable",
                para: ":scope file :block yesterday"
            },
            57,
            64
        ))
    )
}
