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
            macro_rules! parse {
                ($obj:ident) => {
                    if let Some((obj, off)) = $obj::parse(&src[pos..]) {
                        return if pos == 0 {
                            (Object::$obj(obj), off, None)
                        } else {
                            (
                                Object::Text(&src[0..pos]),
                                pos,
                                Some((Object::$obj(obj), off)),
                            )
                        };
                    }
                };
            }

            let first = bytes[pos];
            let second = bytes[pos + 1];
            let third = bytes[pos + 2];

            if first == b'@' && second == b'@' {
                parse!(Snippet);
            }

            if first == b'[' {
                if second == b'f' && third == b'n' {
                    parse!(FnRef);
                } else if second == b'[' {
                    parse!(Link);
                } else {
                    parse!(Cookie);
                    // TODO: Timestamp
                }
            }

            if first == b'{' && second == b'{' && third == b'{' {
                parse!(Macros);
            }

            if first == b'<' && second == b'<' {
                if third == b'<' {
                    parse!(RadioTarget);
                } else if third != b'<' && third != b'\n' {
                    parse!(Target);
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
                    if let Some(end) = Emphasis::parse(&src[pos..], first).map(|i| i + pos) {
                        macro_rules! emph {
                            ($obj:ident) => {
                                return if pos == 0 {
                                    (Object::$obj { end }, 1, None)
                                } else {
                                    (
                                        Object::Text(&src[0..pos]),
                                        pos,
                                        Some((Object::$obj { end }, end)),
                                    )
                                };
                            };
                        }

                        match first {
                            b'*' => emph!(Bold),
                            b'+' => emph!(Strike),
                            b'/' => emph!(Italic),
                            b'_' => emph!(Underline),
                            b'~' => {
                                return if pos == 0 {
                                    (Object::Code(&src[1..end + 1]), end + 2, None)
                                } else {
                                    (
                                        Object::Text(&src[0..pos]),
                                        pos,
                                        Some((Object::Code(&src[pos + 1..end + 1]), end - pos + 2)),
                                    )
                                };
                            }
                            b'=' => {
                                return if pos == 0 {
                                    (Object::Verbatim(&src[1..end + 1]), end + 2, None)
                                } else {
                                    (
                                        Object::Text(&src[0..pos]),
                                        pos,
                                        Some((
                                            Object::Verbatim(&src[pos + 1..end + 1]),
                                            end - pos + 2,
                                        )),
                                    )
                                };
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                if first == b'c' && second == b'a' && third == b'l' {
                    parse!(InlineCall);
                }

                if first == b's' && second == b'r' && third == b'c' {
                    parse!(InlineSrc);
                }
            }
        }

        (Object::Text(src), src.len(), None)
    }
}

#[test]
fn next_2() {
    // TODO: more tests
    assert_eq!(Object::next_2("*bold*"), (Object::Bold { end: 4 }, 1, None));
    assert_eq!(
        Object::next_2("Normal =verbatim="),
        (
            Object::Text("Normal "),
            "Normal ".len(),
            Some((Object::Verbatim("verbatim"), "=verbatim=".len()))
        )
    );
}
