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

const ACTIVE_TAB: [u8; 6] = [b' ', b'"', b'(', b'{', b'\'', b'\n'];

#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Objects<'a> {
    text: &'a str,
    off: usize,
}

impl<'a> Objects<'a> {
    pub fn new(text: &'a str) -> Objects<'a> {
        Objects { text, off: 0 }
    }
}

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

    Bold(&'a str),
    Verbatim(&'a str),
    Italic(&'a str),
    Strike(&'a str),
    Underline(&'a str),
    Code(&'a str),

    Text(&'a str),
}

impl<'a> Object<'a> {
    pub fn parse(src: &'a str) -> (Object<'a>, usize) {
        macro_rules! parse {
            ($ty:ident) => {
                $ty::parse(src).map(|(s, l)| (Object::$ty(s), l))
            };
        }

        macro_rules! parse_emphasis {
            ($mk:tt, $ty:ident) => {
                Emphasis::parse(src, $mk).map(|(s, l)| (Object::$ty(s), l))
            };
        }

        (match src.as_bytes()[0] {
            b'@' => parse!(Snippet),
            b'[' => parse!(FnRef)
                .or_else(|| parse!(Link))
                .or_else(|| parse!(Cookie)),
            b's' => parse!(InlineSrc),
            b'c' => parse!(InlineCall),
            b'{' => parse!(Macros),
            b'<' => parse!(RadioTarget).or_else(|| parse!(Target)),
            b'*' => parse_emphasis!(b'*', Bold),
            b'=' => parse_emphasis!(b'=', Verbatim),
            b'/' => parse_emphasis!(b'/', Italic),
            b'+' => parse_emphasis!(b'+', Strike),
            b'_' => parse_emphasis!(b'_', Underline),
            b'~' => parse_emphasis!(b'~', Code),
            _ => None,
        })
        .unwrap_or((Object::Text(&src[0..1]), 1))
    }
}
