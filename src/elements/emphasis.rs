use bytecount::count;
use memchr::memchr_iter;

use crate::elements::Element;

#[derive(Debug, PartialEq)]
pub(crate) struct Emphasis<'a> {
    marker: u8,
    contents: &'a str,
}

impl<'a> Emphasis<'a> {
    pub fn parse(text: &str, marker: u8) -> Option<(&str, Emphasis)> {
        if text.len() < 3 {
            return None;
        }

        let bytes = text.as_bytes();

        if bytes[1].is_ascii_whitespace() {
            return None;
        }

        for i in memchr_iter(marker, bytes).skip(1) {
            // contains at least one character
            if i == 1 {
                continue;
            } else if count(&bytes[1..i], b'\n') >= 2 {
                break;
            } else if validate_marker(i, text) {
                return Some((
                    &text[i + 1..],
                    Emphasis {
                        marker,
                        contents: &text[1..i],
                    },
                ));
            }
        }
        None
    }

    pub fn into_element(self) -> (Element<'a>, &'a str) {
        let Emphasis { marker, contents } = self;
        let element = match marker {
            b'*' => Element::Bold,
            b'+' => Element::Strike,
            b'/' => Element::Italic,
            b'_' => Element::Underline,
            b'=' => Element::Verbatim {
                value: contents.into(),
            },
            b'~' => Element::Code {
                value: contents.into(),
            },
            _ => unreachable!(),
        };
        (element, contents)
    }
}

fn validate_marker(pos: usize, text: &str) -> bool {
    if text.as_bytes()[pos - 1].is_ascii_whitespace() {
        false
    } else if let Some(&post) = text.as_bytes().get(pos + 1) {
        match post {
            b' ' | b'-' | b'.' | b',' | b':' | b'!' | b'?' | b'\'' | b'\n' | b')' | b'}' => true,
            _ => false,
        }
    } else {
        true
    }
}

#[test]
fn parse() {
    assert_eq!(
        Emphasis::parse("*bold*", b'*'),
        Some((
            "",
            Emphasis {
                contents: "bold",
                marker: b'*'
            }
        ))
    );
    assert_eq!(
        Emphasis::parse("*bo*ld*", b'*'),
        Some((
            "",
            Emphasis {
                contents: "bo*ld",
                marker: b'*'
            }
        ))
    );
    assert_eq!(
        Emphasis::parse("*bo\nld*", b'*'),
        Some((
            "",
            Emphasis {
                contents: "bo\nld",
                marker: b'*'
            }
        ))
    );
    assert_eq!(Emphasis::parse("*bold*a", b'*'), None);
    assert_eq!(Emphasis::parse("*bold*", b'/'), None);
    assert_eq!(Emphasis::parse("*bold *", b'*'), None);
    assert_eq!(Emphasis::parse("* bold*", b'*'), None);
    assert_eq!(Emphasis::parse("*b\nol\nd*", b'*'), None);
}
