use memchr::{memchr, memchr2};

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

pub fn parse(text: &str) -> Option<(Key<'_>, &str, usize)> {
    debug_assert!(text.starts_with("#+"));

    let bytes = text.as_bytes();

    let (key, off) = memchr2(b':', b'[', bytes)
        .filter(|&i| {
            bytes[2..i]
                .iter()
                .all(|&c| c.is_ascii_alphabetic() || c == b'_')
        })
        .map(|i| (&text[2..i], i + 1))?;

    let (option, off) = if bytes[off - 1] == b'[' {
        memchr(b']', bytes)
            .filter(|&i| {
                bytes[off..i].iter().all(|&c| c != b'\n') && i < text.len() && bytes[i + 1] == b':'
            })
            .map(|i| (Some(&text[off..i]), i + 2 /* ]: */))?
    } else {
        (None, off)
    };

    let (value, off) = memchr(b'\n', bytes)
        .map(|i| (&text[off..i], i + 1))
        .unwrap_or_else(|| (&text[off..], text.len()));

    Some((
        match &*key.to_uppercase() {
            "AUTHOR" => Key::Author,
            "CALL" => Key::Call,
            "DATE" => Key::Date,
            "HEADER" => Key::Header,
            "NAME" => Key::Name,
            "PLOT" => Key::Plot,
            "TITLE" => Key::Title,
            "RESULTS" => Key::Results { option },
            "CAPTION" => Key::Caption { option },
            k if k.starts_with("ATTR_") => Key::Attr {
                backend: &key["ATTR_".len()..],
            },
            _ => Key::Custom(key),
        },
        value.trim(),
        off,
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::*;

        assert_eq!(
            parse("#+KEY:"),
            Some((Key::Custom("KEY"), "", "#+KEY:".len()))
        );
        assert_eq!(
            parse("#+KEY: VALUE"),
            Some((Key::Custom("KEY"), "VALUE", "#+KEY: VALUE".len()))
        );
        assert_eq!(
            parse("#+K_E_Y: VALUE"),
            Some((Key::Custom("K_E_Y"), "VALUE", "#+K_E_Y: VALUE".len()))
        );
        assert_eq!(
            parse("#+KEY:VALUE\n"),
            Some((Key::Custom("KEY"), "VALUE", "#+KEY:VALUE\n".len()))
        );
        assert_eq!(parse("#+KE Y: VALUE"), None);
        assert_eq!(parse("#+ KEY: VALUE"), None);

        assert_eq!(
            parse("#+RESULTS:"),
            Some((Key::Results { option: None }, "", "#+RESULTS:".len()))
        );

        assert_eq!(
            parse("#+ATTR_LATEX: :width 5cm"),
            Some((
                Key::Attr { backend: "LATEX" },
                ":width 5cm",
                "#+ATTR_LATEX: :width 5cm".len()
            ))
        );

        assert_eq!(
            parse("#+CALL: double(n=4)"),
            Some((Key::Call, "double(n=4)", "#+CALL: double(n=4)".len()))
        );

        assert_eq!(
            parse("#+CAPTION[Short caption]: Longer caption."),
            Some((
                Key::Caption {
                    option: Some("Short caption")
                },
                "Longer caption.",
                "#+CAPTION[Short caption]: Longer caption.".len()
            ))
        );
    }
}
