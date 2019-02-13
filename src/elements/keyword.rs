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

pub fn parse(src: &str) -> Option<(Key<'_>, &str, usize)> {
    debug_assert!(src.starts_with("#+"));

    let bytes = src.as_bytes();
    let key_end = memchr2(b':', b'[', bytes).filter(|&i| {
        bytes[2..i]
            .iter()
            .all(|&c| c.is_ascii_alphabetic() || c == b'_')
    })?;

    let option = if bytes[key_end] == b'[' {
        let option =
            memchr(b']', bytes).filter(|&i| bytes[key_end..i].iter().all(|&c| c != b'\n'))?;
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
        match src[2..key_end].to_uppercase().as_str() {
            "AUTHOR" => Key::Author,
            "CALL" => Key::Call,
            "DATE" => Key::Date,
            "HEADER" => Key::Header,
            "NAME" => Key::Name,
            "PLOT" => Key::Plot,
            "TITLE" => Key::Title,
            "RESULTS" => Key::Results {
                option: if key_end == option {
                    None
                } else {
                    Some(&src[key_end + 1..option - 1])
                },
            },
            "CAPTION" => Key::Caption {
                option: if key_end == option {
                    None
                } else {
                    Some(&src[key_end + 1..option - 1])
                },
            },
            key if key.starts_with("ATTR_") => Key::Attr {
                backend: &src["#+ATTR_".len()..key_end],
            },
            _ => Key::Custom(&src[2..key_end]),
        },
        &src[option + 1..end].trim(),
        end,
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
