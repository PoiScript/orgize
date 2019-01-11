pub struct Emphasis;

impl Emphasis {
    // TODO: return usize instead of Option<usize>
    pub fn parse(src: &str, marker: u8) -> Option<usize> {
        expect!(src, 1, |c: u8| !c.is_ascii_whitespace());

        let mut lines = 0;
        let end = until_while!(src, 1, marker, |c| {
            if c == b'\n' {
                lines += 1;
            }
            lines < 2
        });

        expect!(src, end - 1, |c: u8| !c.is_ascii_whitespace());

        if end < src.len() - 1 {
            expect!(src, end + 1, |ch| ch == b' '
                || ch == b'-'
                || ch == b'.'
                || ch == b','
                || ch == b':'
                || ch == b'!'
                || ch == b'?'
                || ch == b'\''
                || ch == b'\n'
                || ch == b')'
                || ch == b'}');
        }

        Some(end)
    }
}

#[test]
fn parse() {
    assert_eq!(Emphasis::parse("*bold*", b'*').unwrap(), "*bold".len());
    assert_eq!(Emphasis::parse("*bo\nld*", b'*').unwrap(), "*bo\nld".len());
    assert!(Emphasis::parse("*bold*a", b'*').is_none());
    assert!(Emphasis::parse("*bold*", b'/').is_none());
    assert!(Emphasis::parse("*bold *", b'*').is_none());
    assert!(Emphasis::parse("* bold*", b'*').is_none());
    assert!(Emphasis::parse("*b\nol\nd*", b'*').is_none());
}
