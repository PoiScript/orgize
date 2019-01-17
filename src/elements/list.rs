pub struct List;

impl List {
    #[inline]
    fn is_item(src: &str) -> bool {
        if src.len() < 2 {
            return false;
        }

        let bytes = src.as_bytes();
        let i = match bytes[0] {
            b'*' | b'-' | b'+' => 1,
            b'0'...b'9' => {
                let i = bytes
                    .iter()
                    .position(|&c| !c.is_ascii_digit())
                    .unwrap_or_else(|| src.len());
                if i >= src.len() - 1 {
                    return false;
                }
                let c = bytes[i];
                if !(c == b'.' || c == b')') {
                    return false;
                }
                i + 1
            }
            _ => return false,
        };

        // bullet is follwed by a space or line ending
        bytes[i] == b' ' || bytes[i] == b'\n'
    }

    #[inline]
    pub fn is_ordered(byte: u8) -> bool {
        match byte {
            b'*' | b'-' | b'+' => false,
            b'0'...b'9' => true,
            _ => unreachable!(),
        }
    }

    // returns (contents_begin, contents_end)
    // TODO: handle nested list
    pub fn parse_item(src: &str, ident: usize) -> (usize, usize) {
        (
            src[ident..].find(' ').map(|i| ident + i + 1).unwrap(),
            if ident > 0 {
                src.find(&format!("\n{:1$}", " ", ident))
                    .map(|i| i + 1)
                    .unwrap_or_else(|| src.len())
            } else {
                src.find('\n').map(|i| i + 1).unwrap_or_else(|| src.len())
            },
        )
    }

    // return (ident, is_ordered, end)
    pub fn parse(src: &str) -> Option<(usize, bool, usize)> {
        macro_rules! ident {
            ($src:expr) => {
                $src.as_bytes()
                    .iter()
                    .position(|&c| c != b' ' && c != b'\t')
                    .unwrap_or(0)
            };
        }

        let bytes = src.as_bytes();
        let starting_ident = ident!(src);

        if !Self::is_item(&src[starting_ident..]) {
            return None;
        }

        let is_ordered = Self::is_ordered(bytes[starting_ident]);
        let mut pos = starting_ident;
        while let Some(i) = src[pos..]
            .find('\n')
            .map(|i| i + pos + 1)
            .filter(|&i| i != src.len())
        {
            let ident = ident!(src[i..]);

            // less indented than its starting line
            if ident < starting_ident {
                return Some((starting_ident, is_ordered, i - 1));
            }

            if ident > starting_ident {
                pos = i;
                continue;
            }

            if bytes[ident + i] == b'\n' && pos < src.len() {
                let nextline_ident = ident!(src[ident + i + 1..]);

                // check if it's two consecutive empty lines
                if nextline_ident < starting_ident
                    || (ident + i + 1 + nextline_ident < src.len()
                        && bytes[ident + i + 1 + nextline_ident] == b'\n')
                {
                    return Some((starting_ident, is_ordered, ident + i + 1 + nextline_ident));
                }

                if nextline_ident == starting_ident {
                    if Self::is_item(&src[i + nextline_ident + 1..]) {
                        pos = i + nextline_ident + 1;
                        continue;
                    } else {
                        return Some((starting_ident, is_ordered, ident + i + 1 + nextline_ident));
                    }
                }
            }

            if Self::is_item(&src[i + ident..]) {
                pos = i;
                continue;
            } else {
                return Some((starting_ident, is_ordered, i - 1));
            }
        }

        Some((starting_ident, is_ordered, src.len()))
    }
}

#[test]
fn parse() {
    assert_eq!(
        List::parse(
            r"+ item1
+ item2
+ item3"
        ),
        Some((0, false, 23))
    );
    assert_eq!(
        List::parse(
            r"* item1
* item2

* item3"
        ),
        Some((0, false, 24))
    );
    assert_eq!(
        List::parse(
            r"- item1
- item2


- item1"
        ),
        Some((0, false, 17))
    );
    assert_eq!(
        List::parse(
            r"1. item1
  2. item1
3. item2"
        ),
        Some((0, true, 28))
    );
    assert_eq!(
        List::parse(
            r"  1) item1
 2) item1
  3) item2"
        ),
        Some((2, true, 10))
    );
    assert_eq!(
        List::parse(
            r"  + item1
    1) item1
  + item2"
        ),
        Some((2, false, 32))
    );
    assert_eq!(
        List::parse(
            r" item1
 + item1
 + item2"
        ),
        None
    );
}

#[test]
fn is_item() {
    assert!(List::is_item("+ item"));
    assert!(List::is_item("- item"));
    assert!(List::is_item("10. item"));
    assert!(List::is_item("10) item"));
    assert!(List::is_item("1. item"));
    assert!(List::is_item("1) item"));
    assert!(List::is_item("10. "));
    assert!(List::is_item("10.\n"));
    assert!(!List::is_item("10."));
    assert!(!List::is_item("-item"));
    assert!(!List::is_item("+item"));
}

#[test]
fn parse_item() {
    assert_eq!(List::parse_item("+ Item1\n+ Item2", 0), (2, 8));
    assert_eq!(
        List::parse_item(
            r"+ item1
 + item1
 + item2",
            0
        ),
        (2, 8)
    );
    assert_eq!(
        List::parse_item(
            r"  1. item1
  + item2",
            2
        ),
        (5, 11)
    );
}
