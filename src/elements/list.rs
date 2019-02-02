use lines::Lines;

pub struct List;

impl List {
    #[inline]
    pub fn is_item(src: &str) -> (bool, bool) {
        if src.is_empty() {
            return (false, false);
        }
        let bytes = src.as_bytes();
        let (i, ordered) = match bytes[0] {
            b'*' | b'-' | b'+' => (1, false),
            b'0'...b'9' => {
                let i = bytes
                    .iter()
                    .position(|&c| !c.is_ascii_digit())
                    .unwrap_or_else(|| src.len());
                let c = bytes[i];
                if !(c == b'.' || c == b')') {
                    return (false, false);
                }
                (i + 1, true)
            }
            _ => return (false, false),
        };

        if i < src.len() {
            // bullet is follwed by a space or line ending
            (bytes[i] == b' ' || bytes[i] == b'\n', ordered)
        } else {
            (false, false)
        }
    }

    // returns (bullets, contents begin, contents end, end, has more)
    pub fn parse(src: &str, ident: usize) -> (&str, usize, usize, usize, bool) {
        debug_assert!(Self::is_item(&src[ident..]).0);
        debug_assert!(
            src[..ident].chars().all(|c| c == ' ' || c == '\t'),
            "{:?} doesn't starts with indentation {}",
            src,
            ident
        );

        let mut lines = Lines::new(src);
        let (mut pre_cont_end, mut pre_end, first_line) = lines.next().unwrap();
        let beg = match memchr::memchr(b' ', &first_line.as_bytes()[ident..]) {
            Some(i) => i + ident + 1,
            None => {
                let len = first_line.len();
                return (
                    &first_line,
                    len,
                    len,
                    len,
                    Self::is_item(lines.next().unwrap().2).0,
                );
            }
        };
        let bullet = &src[0..beg];

        while let Some((mut cont_end, mut end, mut line)) = lines.next() {
            // this line is emtpy
            if line.is_empty() {
                if let Some((next_cont_end, next_end, next_line)) = lines.next() {
                    // next line is emtpy, too
                    if next_line.is_empty() {
                        return (bullet, beg, pre_cont_end, next_end, false);
                    } else {
                        // move to next line
                        pre_end = end;
                        cont_end = next_cont_end;
                        end = next_end;
                        line = next_line;
                    }
                } else {
                    return (bullet, beg, pre_cont_end, end, false);
                }
            }

            let line_ident = Self::ident(line);

            if line_ident < ident {
                return (bullet, beg, pre_cont_end, pre_end, false);
            } else if line_ident == ident {
                return (
                    bullet,
                    beg,
                    pre_cont_end,
                    pre_end,
                    Self::is_item(&line[ident..]).0,
                );
            }

            pre_end = end;
            pre_cont_end = cont_end;
        }

        (bullet, beg, src.len(), src.len(), false)
    }

    fn ident(src: &str) -> usize {
        src.as_bytes()
            .iter()
            .position(|&c| c != b' ' && c != b'\t')
            .unwrap_or(0)
    }
}

#[test]
fn is_item() {
    assert_eq!(List::is_item("+ item"), (true, false));
    assert_eq!(List::is_item("- item"), (true, false));
    assert_eq!(List::is_item("10. item"), (true, true));
    assert_eq!(List::is_item("10) item"), (true, true));
    assert_eq!(List::is_item("1. item"), (true, true));
    assert_eq!(List::is_item("1) item"), (true, true));
    assert_eq!(List::is_item("10. "), (true, true));
    assert_eq!(List::is_item("10.\n"), (true, true));
    assert_eq!(List::is_item("10."), (false, false));
    assert_eq!(List::is_item("+"), (false, false));
    assert_eq!(List::is_item("-item"), (false, false));
    assert_eq!(List::is_item("+item"), (false, false));
}

#[test]
fn parse() {
    assert_eq!(
        List::parse("+ item1\n+ item2\n+ item3", 0),
        ("+ ", 2, 7, 8, true)
    );
    assert_eq!(
        List::parse("* item1\n\n* item2\n* item3", 0),
        ("* ", 2, 7, 9, true)
    );
    assert_eq!(
        List::parse("- item1\n\n\n- item2\n- item3", 0),
        ("- ", 2, 7, 10, false)
    );
    assert_eq!(
        List::parse("1. item1\n\n\n\n2. item2\n3. item3", 0),
        ("1. ", 3, 8, 11, false)
    );
    assert_eq!(
        List::parse("  + item1\n    + item2\n+ item3", 2),
        ("  + ", 4, 21, 22, false)
    );
    assert_eq!(
        List::parse("  + item1\n  + item2\n  + item3", 2),
        ("  + ", 4, 9, 10, true)
    );
    assert_eq!(List::parse("+\n", 0), ("+", 1, 1, 1, false));
    assert_eq!(List::parse("+\n+ item2\n+ item3", 0), ("+", 1, 1, 1, true));
    assert_eq!(List::parse("1) item1", 0), ("1) ", 3, 8, 8, false));
    assert_eq!(List::parse("1) item1\n", 0), ("1) ", 3, 8, 9, false));
}
