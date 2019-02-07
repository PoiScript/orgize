use memchr::memchr;

pub struct Emphasis;

impl Emphasis {
    // TODO: return usize instead of Option<usize>
    pub fn parse(src: &str, marker: u8) -> Option<usize> {
        expect!(src, 1, |c: u8| !c.is_ascii_whitespace())?;

        let bytes = src.as_bytes();
        let end = memchr(marker, &bytes[1..])
            .map(|i| i + 1)
            .filter(|&i| bytes[1..i].iter().filter(|&&c| c == b'\n').count() < 2)?;

        expect!(src, end - 1, |c: u8| !c.is_ascii_whitespace())?;

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
                || ch == b'}')?;
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
