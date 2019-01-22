pub struct Keyword;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum Key<'a> {
    // Affiliated Keywords
    // Only "CAPTION" and "RESULTS" keywords can have an optional value.
    Caption { option: Option<&'a str> },
    Header,
    Name,
    Plot,
    Results { option: Option<&'a str> },
    Attr { backend: &'a str },

    // Keywords
    Author,
    Date,
    Title,
    Custom(&'a str),

    // Babel Call
    Call,
}

impl Keyword {
    // return (key, value, offset)
    pub fn parse(src: &str) -> Option<(Key<'_>, &str, usize)> {
        if cfg!(test) {
            starts_with!(src, "#+");
        }

        let key_end = until_while!(src, 2, |c| c == b':' || c == b'[', |c: u8| c
            .is_ascii_alphabetic()
            || c == b'_')?;

        let option = if src.as_bytes()[key_end] == b'[' {
            let option = until_while!(src, key_end, b']', |c: u8| c != b'\n')?;
            expect!(src, option + 1, b':')?;
            option + 1
        } else {
            key_end
        };

        // includes the eol character
        let end = memchr::memchr(b'\n', src.as_bytes())
            .map(|i| i + 1)
            .unwrap_or_else(|| src.len());

        Some((
            match &src[2..key_end] {
                key if key.eq_ignore_ascii_case("CAPTION") => Key::Caption {
                    option: if key_end == option {
                        None
                    } else {
                        Some(&src[key_end + 1..option - 1])
                    },
                },
                key if key.eq_ignore_ascii_case("HEADER") => Key::Header,
                key if key.eq_ignore_ascii_case("NAME") => Key::Name,
                key if key.eq_ignore_ascii_case("PLOT") => Key::Plot,
                key if key.eq_ignore_ascii_case("RESULTS") => Key::Results {
                    option: if key_end == option {
                        None
                    } else {
                        Some(&src[key_end + 1..option - 1])
                    },
                },
                key if key.eq_ignore_ascii_case("AUTHOR") => Key::Author,
                key if key.eq_ignore_ascii_case("DATE") => Key::Date,
                key if key.eq_ignore_ascii_case("TITLE") => Key::Title,
                key if key.eq_ignore_ascii_case("CALL") => Key::Call,
                key if key.starts_with("ATTR_") => Key::Attr {
                    backend: &src["#+ATTR_".len()..key_end],
                },
                key => Key::Custom(key),
            },
            &src[option + 1..end].trim(),
            end,
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Keyword::parse("#+KEY:"),
        Some((Key::Custom("KEY"), "", "#+KEY:".len()))
    );
    assert_eq!(
        Keyword::parse("#+KEY: VALUE"),
        Some((Key::Custom("KEY"), "VALUE", "#+KEY: VALUE".len()))
    );
    assert_eq!(
        Keyword::parse("#+K_E_Y: VALUE"),
        Some((Key::Custom("K_E_Y"), "VALUE", "#+K_E_Y: VALUE".len()))
    );
    assert_eq!(
        Keyword::parse("#+KEY:VALUE\n"),
        Some((Key::Custom("KEY"), "VALUE", "#+KEY:VALUE\n".len()))
    );
    assert!(Keyword::parse("#+KE Y: VALUE").is_none());
    assert!(Keyword::parse("#+ KEY: VALUE").is_none());
    assert!(Keyword::parse("# +KEY: VALUE").is_none());
    assert!(Keyword::parse(" #+KEY: VALUE").is_none());

    assert_eq!(
        Keyword::parse("#+RESULTS:"),
        Some((Key::Results { option: None }, "", "#+RESULTS:".len()))
    );

    assert_eq!(
        Keyword::parse("#+ATTR_LATEX: :width 5cm"),
        Some((
            Key::Attr { backend: "LATEX" },
            ":width 5cm",
            "#+ATTR_LATEX: :width 5cm".len()
        ))
    );

    assert_eq!(
        Keyword::parse("#+CALL: double(n=4)"),
        Some((Key::Call, "double(n=4)", "#+CALL: double(n=4)".len()))
    );

    assert_eq!(
        Keyword::parse("#+CAPTION[Short caption]: Longer caption."),
        Some((
            Key::Caption {
                option: Some("Short caption")
            },
            "Longer caption.",
            "#+CAPTION[Short caption]: Longer caption.".len()
        ))
    );
}
