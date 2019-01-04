#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct InlineCall<'a> {
    pub name: &'a str,
    pub args: &'a str,
    // header args for block
    pub inside_header: Option<&'a str>,
    // header args for call line
    pub end_header: Option<&'a str>,
}

impl<'a> InlineCall<'a> {
    pub fn parse(src: &'a str) -> Option<(InlineCall, usize)> {
        starts_with!(src, "call_");

        let mut pos = until_while!(src, 5, |c| c == b'[' || c == b'(', |c: u8| c
            .is_ascii_graphic());
        let mut pos_;

        let name = &src[5..pos];

        let inside_header = if src.as_bytes()[pos] == b'[' {
            pos_ = pos;
            pos = until_while!(src, pos, b']', |c: u8| c != b'\n') + 1;
            expect!(src, pos, b'(');
            Some(&src[pos_ + 1..pos - 1])
        } else {
            None
        };

        pos_ = pos;
        pos = until_while!(src, pos, b')', |c| c != b'\n');
        let args = &src[pos_ + 1..pos];

        let end_header = if src.len() > pos + 1 && src.as_bytes()[pos + 1] == b'[' {
            pos_ = pos;
            pos = until_while!(src, pos_ + 1, |c| c == b']', |c: u8| c != b'\n'
                && c != b')');
            Some(&src[pos_ + 2..pos])
        } else {
            None
        };

        Some((
            InlineCall {
                name,
                inside_header,
                args,
                end_header,
            },
            pos + 1,
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        InlineCall::parse("call_square(4)").unwrap(),
        (
            InlineCall {
                name: "square",
                args: "4",
                inside_header: None,
                end_header: None,
            },
            "call_square(4)".len()
        )
    );
    assert_eq!(
        InlineCall::parse("call_square[:results output](4)").unwrap(),
        (
            InlineCall {
                name: "square",
                args: "4",
                inside_header: Some(":results output"),
                end_header: None,
            },
            "call_square[:results output](4)".len()
        )
    );
    assert_eq!(
        InlineCall::parse("call_square(4)[:results html]").unwrap(),
        (
            InlineCall {
                name: "square",
                args: "4",
                inside_header: None,
                end_header: Some(":results html"),
            },
            "call_square(4)[:results html]".len()
        )
    );
    assert_eq!(
        InlineCall::parse("call_square[:results output](4)[:results html]").unwrap(),
        (
            InlineCall {
                name: "square",
                args: "4",
                inside_header: Some(":results output"),
                end_header: Some(":results html"),
            },
            "call_square[:results output](4)[:results html]".len()
        )
    );
}
