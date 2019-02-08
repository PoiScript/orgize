mod cookie;
mod emphasis;
mod fn_ref;
mod inline_call;
mod inline_src;
mod link;
mod macros;
mod radio_target;
mod snippet;
mod target;

pub use self::cookie::Cookie;
use jetscii::bytes;

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Object<'a> {
    Cookie(Cookie<'a>),
    FnRef {
        label: Option<&'a str>,
        def: Option<&'a str>,
    },
    InlineCall {
        name: &'a str,
        args: &'a str,
        inside_header: Option<&'a str>,
        end_header: Option<&'a str>,
    },
    InlineSrc {
        lang: &'a str,
        option: Option<&'a str>,
        body: &'a str,
    },
    Link {
        path: &'a str,
        desc: Option<&'a str>,
    },
    Macros {
        name: &'a str,
        args: Option<&'a str>,
    },
    RadioTarget {
        target: &'a str,
    },
    Snippet {
        name: &'a str,
        value: &'a str,
    },
    Target {
        target: &'a str,
    },

    // `end` indicates the position of the second marker
    Bold {
        end: usize,
    },
    Italic {
        end: usize,
    },
    Strike {
        end: usize,
    },
    Underline {
        end: usize,
    },

    Verbatim(&'a str),
    Code(&'a str),
    Text(&'a str),
}

impl<'a> Object<'a> {
    pub fn next_2(src: &'a str) -> (Object<'a>, usize, Option<(Object<'a>, usize)>) {
        let bytes = src.as_bytes();

        if src.len() <= 2 {
            return (Object::Text(src), src.len(), None);
        }

        let bs = bytes!(b'@', b' ', b'"', b'(', b'\n', b'{', b'<', b'[');

        let mut pos = 0;
        loop {
            macro_rules! brk {
                ($obj:expr, $off:expr, $pos:expr) => {
                    break if $pos == 0 {
                        ($obj, $off, None)
                    } else {
                        (Object::Text(&src[0..$pos]), $pos, Some(($obj, $off)))
                    };
                };
            }

            let mut pre = pos;

            match bytes[pos] {
                b'@' if bytes[pos + 1] == b'@' => {
                    if let Some((name, value, off)) = snippet::parse(&src[pos..]) {
                        brk!(Object::Snippet { name, value }, off, pos);
                    }
                }
                b'{' if bytes[pos + 1] == b'{' && bytes[pos + 2] == b'{' => {
                    if let Some((name, args, off)) = macros::parse(&src[pos..]) {
                        brk!(Object::Macros { name, args }, off, pos);
                    }
                }
                b'<' if bytes[pos + 1] == b'<' => {
                    if bytes[pos + 2] == b'<' {
                        if let Some((target, off)) = radio_target::parse(&src[pos..]) {
                            brk!(Object::RadioTarget { target }, off, pos);
                        }
                    } else if bytes[pos + 2] != b'\n' {
                        if let Some((target, off)) = target::parse(&src[pos..]) {
                            brk!(Object::Target { target }, off, pos);
                        }
                    }
                }
                b'[' => {
                    if bytes[pos + 1..].starts_with(b"fn:") {
                        if let Some((label, def, off)) = fn_ref::parse(&src[pos..]) {
                            brk!(Object::FnRef { label, def }, off, pos);
                        }
                    }

                    if bytes[pos + 1] == b'[' {
                        if let Some((path, desc, off)) = link::parse(&src[pos..]) {
                            brk!(Object::Link { path, desc }, off, pos);
                        }
                    }

                    if let Some((cookie, off)) = cookie::parse(&src[pos..]) {
                        brk!(Object::Cookie(cookie), off, pos);
                    }
                    // TODO: Timestamp
                }
                b'{' | b' ' | b'"' | b',' | b'(' | b'\n' => pre += 1,
                _ => (),
            }

            match bytes[pre] {
                b'*' => {
                    if let Some(end) = emphasis::parse(&src[pre..], b'*') {
                        brk!(Object::Bold { end }, 1, pre);
                    }
                }
                b'+' => {
                    if let Some(end) = emphasis::parse(&src[pre..], b'+') {
                        brk!(Object::Strike { end }, 1, pre);
                    }
                }
                b'/' => {
                    if let Some(end) = emphasis::parse(&src[pre..], b'/') {
                        brk!(Object::Italic { end }, 1, pre);
                    }
                }
                b'_' => {
                    if let Some(end) = emphasis::parse(&src[pre..], b'_') {
                        brk!(Object::Underline { end }, 1, pre);
                    }
                }
                b'=' => {
                    if let Some(end) = emphasis::parse(&src[pre..], b'=') {
                        brk!(Object::Verbatim(&src[pre + 1..pre + end]), end + 1, pre);
                    }
                }
                b'~' => {
                    if let Some(end) = emphasis::parse(&src[pre..], b'~') {
                        brk!(Object::Code(&src[pre + 1..pre + end]), end + 1, pre);
                    }
                }
                b'c' if src[pre..].starts_with("call_") => {
                    if let Some((name, args, inside_header, end_header, off)) =
                        inline_call::parse(&src[pre..])
                    {
                        brk!(
                            Object::InlineCall {
                                name,
                                args,
                                inside_header,
                                end_header,
                            },
                            off,
                            pre
                        );
                    }
                }
                b's' if src[pre..].starts_with("src_") => {
                    if let Some((lang, option, body, off)) = inline_src::parse(&src[pre..]) {
                        brk!(Object::InlineSrc { lang, option, body }, off, pre);
                    }
                }
                _ => (),
            }

            if let Some(off) = bs
                .find(&bytes[pos + 1..])
                .map(|i| i + pos + 1)
                .filter(|&i| i < src.len() - 2)
            {
                pos = off;
            } else {
                break (Object::Text(src), src.len(), None);
            }
        }
    }
}

#[test]
fn next_2() {
    // TODO: more tests
    assert_eq!(Object::next_2("*bold*"), (Object::Bold { end: 5 }, 1, None));
    assert_eq!(
        Object::next_2("Normal =verbatim="),
        (
            Object::Text("Normal "),
            "Normal ".len(),
            Some((Object::Verbatim("verbatim"), "=verbatim=".len()))
        )
    );
}
