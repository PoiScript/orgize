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

pub fn parse<'a>(src: &'a str) -> (Object<'a>, usize, Option<(Object<'a>, usize)>) {
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
            b'{' | b' ' | b'"' | b',' | b'(' | b'\n' => {
                if let Some((obj, off)) = parse_text_markup(&src[pos + 1..]) {
                    brk!(obj, off, pos + 1);
                }
            }
            _ => {
                if let Some((obj, off)) = parse_text_markup(&src[pos..]) {
                    brk!(obj, off, pos);
                }
            }
        }

        if let Some(off) = bs
            .find(&bytes[pos + 1..])
            .map(|i| i + pos + 1)
            .filter(|&i| i < src.len() - 3)
        {
            pos = off;
        } else {
            break (Object::Text(src), src.len(), None);
        }
    }
}

fn parse_text_markup<'a>(src: &'a str) -> Option<(Object<'a>, usize)> {
    match src.as_bytes()[0] {
        b'*' => emphasis::parse(src, b'*').map(|end| (Object::Bold { end }, 1)),
        b'+' => emphasis::parse(src, b'+').map(|end| (Object::Strike { end }, 1)),
        b'/' => emphasis::parse(src, b'/').map(|end| (Object::Italic { end }, 1)),
        b'_' => emphasis::parse(src, b'_').map(|end| (Object::Underline { end }, 1)),
        b'=' => emphasis::parse(src, b'=').map(|end| (Object::Verbatim(&src[1..end]), end + 1)),
        b'~' => emphasis::parse(src, b'~').map(|end| (Object::Code(&src[1..end]), end + 1)),
        b's' if src.starts_with("src_") => inline_src::parse(src)
            .map(|(lang, option, body, off)| (Object::InlineSrc { lang, option, body }, off)),
        b'c' if src.starts_with("call_") => {
            inline_call::parse(src).map(|(name, args, inside_header, end_header, off)| {
                (
                    Object::InlineCall {
                        name,
                        args,
                        inside_header,
                        end_header,
                    },
                    off,
                )
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::*;

        assert_eq!(parse("*bold*"), (Object::Bold { end: 5 }, 1, None));
        assert_eq!(
            parse("Normal =verbatim="),
            (
                Object::Text("Normal "),
                "Normal ".len(),
                Some((Object::Verbatim("verbatim"), "=verbatim=".len()))
            )
        );
        // TODO: more tests
    }
}
