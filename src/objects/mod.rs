mod cookie;
mod emphasis;
mod entity;
mod fn_ref;
mod fragment;
mod inline_call;
mod inline_src;
mod link;
mod macros;
mod snippet;
mod target;

pub use self::cookie::Cookie;
pub use self::emphasis::Emphasis;
pub use self::fn_ref::FnRef;
pub use self::inline_call::InlineCall;
pub use self::inline_src::InlineSrc;
pub use self::link::Link;
pub use self::macros::Macros;
pub use self::snippet::Snippet;
pub use self::target::{RadioTarget, Target};

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Object<'a> {
    Cookie(Cookie<'a>),
    FnRef(FnRef<'a>),
    InlineCall(InlineCall<'a>),
    InlineSrc(InlineSrc<'a>),
    Link(Link<'a>),
    Macros(Macros<'a>),
    RadioTarget(RadioTarget<'a>),
    Snippet(Snippet<'a>),
    Target(Target<'a>),

    // `end` indicates the position of the second marker
    Bold { end: usize },
    Italic { end: usize },
    Strike { end: usize },
    Underline { end: usize },

    Verbatim(&'a str),
    Code(&'a str),
    Text(&'a str),
}

impl<'a> Object<'a> {
    pub fn next_2(src: &'a str) -> (Object<'a>, usize, Option<(Object<'a>, usize)>) {
        let bytes = src.as_bytes();

        if src.len() < 2 {
            return (Object::Text(src), src.len(), None);
        }

        // TODO: refactor with src[..].find(..)
        for pos in 0..src.len() - 2 {
            macro_rules! ret {
                ($obj:expr, $off:expr) => {
                    return if pos == 0 {
                        ($obj, $off, None)
                    } else {
                        (Object::Text(&src[0..pos]), pos, Some(($obj, $off)))
                    };
                };
            }

            let first = bytes[pos];
            let second = bytes[pos + 1];
            let third = bytes[pos + 2];

            if first == b'@' && second == b'@' {
                if let Some((snippet, off)) = Snippet::parse(&src[pos..]) {
                    ret!(Object::Snippet(snippet), off);
                }
            }

            if first == b'[' {
                if second == b'f' && third == b'n' {
                    if let Some((fn_ref, off)) = FnRef::parse(&src[pos..]) {
                        ret!(Object::FnRef(fn_ref), off);
                    }
                } else if second == b'[' {
                    if let Some((link, off)) = Link::parse(&src[pos..]) {
                        ret!(Object::Link(link), off);
                    }
                } else {
                    if let Some((cookie, off)) = Cookie::parse(&src[pos..]) {
                        ret!(Object::Cookie(cookie), off);
                    }
                    // TODO: Timestamp
                }
            }

            if first == b'{' && second == b'{' && third == b'{' {
                if let Some((macros, off)) = Macros::parse(&src[pos..]) {
                    ret!(Object::Macros(macros), off);
                }
            }

            if first == b'<' && second == b'<' {
                if third == b'<' {
                    if let Some((target, off)) = RadioTarget::parse(&src[pos..]) {
                        ret!(Object::RadioTarget(target), off);
                    }
                } else if third != b'<' && third != b'\n' {
                    if let Some((target, off)) = Target::parse(&src[pos..]) {
                        ret!(Object::Target(target), off);
                    }
                }
            }

            if pos == 0
                || bytes[pos - 1] == b' '
                || bytes[pos - 1] == b'"'
                || bytes[pos - 1] == b'('
                || bytes[pos - 1] == b','
                || bytes[pos - 1] == b'\n'
                || bytes[pos - 1] == b'{'
            {
                if (first == b'*'
                    || first == b'+'
                    || first == b'/'
                    || first == b'='
                    || first == b'_'
                    || first == b'~')
                    && !second.is_ascii_whitespace()
                {
                    if let Some(end) = Emphasis::parse(&src[pos..], first) {
                        match first {
                            b'*' => ret!(Object::Bold { end }, 1),
                            b'+' => ret!(Object::Strike { end }, 1),
                            b'/' => ret!(Object::Italic { end }, 1),
                            b'_' => ret!(Object::Underline { end }, 1),
                            b'~' => ret!(Object::Code(&src[pos + 1..pos + end]), end + 1),
                            b'=' => ret!(Object::Verbatim(&src[pos + 1..pos + end]), end + 1),
                            _ => unreachable!(),
                        }
                    }
                }

                if first == b'c' && second == b'a' && third == b'l' {
                    if let Some((call, off)) = InlineCall::parse(&src[pos..]) {
                        ret!(Object::InlineCall(call), off);
                    }
                }

                if first == b's' && second == b'r' && third == b'c' {
                    if let Some((src, off)) = InlineSrc::parse(&src[pos..]) {
                        ret!(Object::InlineSrc(src), off);
                    }
                }
            }
        }

        (Object::Text(src), src.len(), None)
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
