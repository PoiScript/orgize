#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Block;

impl Block {
    pub fn parse(src: &str) -> Option<(usize, usize, usize, usize)> {
        if src.len() < 17 || !src[0..8].eq_ignore_ascii_case("#+BEGIN_") {
            return None;
        }

        let args = eol!(src);
        let name = until_while!(src, 8, |c| c == b' ' || c == b'\n', |c: u8| c
            .is_ascii_alphabetic());
        // TODO: ignore case match
        let content = src.find(&format!("\n#+END_{}", &src[8..name]))?;
        let end = eol!(src, content + 1);

        Some((name, args, content, end + 1))
    }
}

#[test]
fn parse() {
    // TODO: testing
}
