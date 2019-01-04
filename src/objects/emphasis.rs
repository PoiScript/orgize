pub struct Emphasis;

impl Emphasis {
    pub fn parse(src: &str, marker: u8) -> Option<(&'_ str, usize)> {
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
                || ch == b')'
                || ch == b'}');
        }

        Some((&src[1..end], end + 1))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Emphasis::parse("*bold*", b'*').unwrap(),
        ("bold", "*bold*".len())
    );
    assert_eq!(
        Emphasis::parse("*bo\nld*", b'*').unwrap(),
        ("bo\nld", "*bo\nld*".len())
    );
    assert!(Emphasis::parse("*bold*a", b'*').is_none());
    assert!(Emphasis::parse("*bold*", b'/').is_none());
    assert!(Emphasis::parse("*bold *", b'*').is_none());
    assert!(Emphasis::parse("* bold*", b'*').is_none());
    assert!(Emphasis::parse("*b\nol\nd*", b'*').is_none());
}
