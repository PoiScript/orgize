pub struct List;

macro_rules! ident {
    ($src:expr) => {
        $src.as_bytes()
            .iter()
            .position(|&c| c != b' ' && c != b'\t')
            .unwrap_or(0)
    };
}

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
        let beg = src[ident..].find(' ').map(|i| ident + i + 1).unwrap();
        let mut lines = lines!(src);
        // skip first line
        let mut pos = lines.next().unwrap();
        for line_end in lines {
            let line = &src[pos..line_end];
            if !line.trim().is_empty() && ident!(line) == ident {
                break;
            }
            pos = line_end;
        }
        (beg, pos)
    }

    // return (ident, is_ordered, contents_end, end)
    pub fn parse(src: &str) -> Option<(usize, bool, usize, usize)> {
        let bytes = src.as_bytes();
        let starting_ident = ident!(src);

        if !Self::is_item(&src[starting_ident..]) {
            return None;
        }

        let mut lines = lines!(src);
        // skip the starting line
        let mut pos = lines.next().unwrap();
        let is_ordered = Self::is_ordered(bytes[starting_ident]);

        Some(loop {
            let mut curr_line = match lines.next() {
                Some(i) => i,
                None => break (starting_ident, is_ordered, pos, pos),
            };
            // current line is empty
            if src[pos..curr_line].trim().is_empty() {
                let next_line = match lines.next() {
                    Some(i) => i,
                    None => break (starting_ident, is_ordered, pos, pos),
                };

                // next line is emtpy, too
                if src[curr_line..next_line].trim().is_empty() {
                    break (starting_ident, is_ordered, pos, next_line);
                } else {
                    // move to next line
                    pos = curr_line;
                    curr_line = next_line;
                }
            }

            let ident = ident!(src[pos..curr_line]);

            // less indented than the starting line
            if ident < starting_ident {
                break (starting_ident, is_ordered, pos, pos);
            }

            if ident > starting_ident || Self::is_item(&src[pos + ident..]) {
                pos = curr_line;
            } else {
                break (starting_ident, is_ordered, pos, pos);
            }
        })
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
        Some((0, false, 23, 23))
    );
    assert_eq!(
        List::parse(
            r"* item1
* item2

* item3"
        ),
        Some((0, false, 24, 24))
    );
    assert_eq!(
        List::parse(
            r"- item1
- item2


- item1"
        ),
        Some((0, false, 16, 18))
    );
    assert_eq!(
        List::parse(
            r"1. item1
  2. item1
3. item2"
        ),
        Some((0, true, 28, 28))
    );
    assert_eq!(
        List::parse(
            r"  1) item1
 2) item1
  3) item2"
        ),
        Some((2, true, 11, 11))
    );
    assert_eq!(
        List::parse(
            r"  + item1
    1) item1
  + item2"
        ),
        Some((2, false, 32, 32))
    );
    assert_eq!(
        List::parse(
            r" item1
 + item1
 + item2"
        ),
        None
    );
    assert_eq!(
        List::parse(
            r#"- Lorem ipsum dolor sit amet, consectetur adipiscing elit.

  - Nulla et dolor vitae elit placerat sagittis. Aliquam a lobortis massa,
    aliquam efficitur arcu.

  - Lorem ipsum dolor sit amet, consectetur adipiscing elit.

  - Phasellus auctor lacus a orci imperdiet, ut facilisis neque lobortis.

  - Proin condimentum id orci vitae lobortis. Nunc sollicitudin risus neque,
    dapibus malesuada sem faucibus vitae.

- Sed vitae dolor augue. Phasellus at rhoncus arcu. Suspendisse potenti.

  - Nulla faucibus, metus ut porta hendrerit, urna lorem porta metus, in tempus
    nibh orci sed sapien.

  - Morbi tortor mi, dapibus vel faucibus a, iaculis sed turpis."#
        ),
        Some((0, false, 666, 666))
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
    assert_eq!(List::parse_item("+ Item1\n\n+ Item2", 0), (2, 9));
    assert_eq!(
        List::parse_item(
            r"+ item1
 + item1
 + item2",
            0
        ),
        (2, 25)
    );
    assert_eq!(
        List::parse_item(
            r"  1. item1
  + item2",
            2
        ),
        (5, 11)
    );
    assert_eq!(
        List::parse_item(
            r"+ It
  em1
+ Item2",
            0
        ),
        (2, 11)
    );
    assert_eq!(
        List::parse_item(
            r#"1) Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec sit amet
   ullamcorper ante, nec pellentesque nisi.
2) Sed pulvinar ut arcu id aliquam.Curabitur quis justo eu magna maximus sodales.
   Curabitur nisl nisi, ornare in enim id, sagittis facilisis magna.
3) Curabitur venenatis molestie eros sit amet congue. Nunc at molestie leo, vitae
   malesuada nisi."#,
            0
        ),
        (3, 119)
    );
    assert_eq!(
        List::parse_item(
            r#"- Lorem ipsum dolor sit amet, consectetur adipiscing elit.

  - Nulla et dolor vitae elit placerat sagittis. Aliquam a lobortis massa,
    aliquam efficitur arcu.

  - Lorem ipsum dolor sit amet, consectetur adipiscing elit.

  - Phasellus auctor lacus a orci imperdiet, ut facilisis neque lobortis.

  - Proin condimentum id orci vitae lobortis. Nunc sollicitudin risus neque,
    dapibus malesuada sem faucibus vitae.

- Sed vitae dolor augue. Phasellus at rhoncus arcu. Suspendisse potenti.

  - Nulla faucibus, metus ut porta hendrerit, urna lorem porta metus, in tempus
    nibh orci sed sapien.

  - Morbi tortor mi, dapibus vel faucibus a, iaculis sed turpis."#,
            0
        ),
        (2, 421)
    );
}
