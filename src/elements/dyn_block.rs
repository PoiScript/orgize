use memchr::{memchr, memchr_iter};

// return (name, parameters, offset, limit, end)
#[inline]
pub fn parse(text: &str) -> Option<(&str, Option<&str>, usize, usize, usize)> {
    debug_assert!(text.starts_with("#+"));

    if text.len() <= "#+BEGIN: ".len() || !text[2..9].eq_ignore_ascii_case("BEGIN: ") {
        return None;
    }

    let bytes = text.as_bytes();
    let mut lines = memchr_iter(b'\n', bytes);

    let (name, para, off) = lines
        .next()
        .map(|i| {
            memchr(b' ', &bytes["#+BEGIN: ".len()..i])
                .map(|x| {
                    (
                        &text["#+BEGIN: ".len().."#+BEGIN: ".len() + x],
                        Some(text["#+BEGIN: ".len() + x..i].trim()),
                        i + 1,
                    )
                })
                .unwrap_or((&text["#+BEGIN: ".len()..i], None, i + 1))
        })
        .filter(|(name, _, _)| name.as_bytes().iter().all(|&c| c.is_ascii_alphabetic()))?;

    let mut pos = off;

    for i in lines {
        if text[pos..i].trim().eq_ignore_ascii_case("#+END:") {
            return Some((name, para, off, pos, i + 1));
        }

        pos = i + 1;
    }

    if text[pos..].trim().eq_ignore_ascii_case("#+END:") {
        Some((name, para, off, pos, text.len()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        // TODO: testing
        assert_eq!(
            parse("#+BEGIN: clocktable :scope file\nCONTENTS\n#+END:\n"),
            Some((
                "clocktable",
                Some(":scope file"),
                "#+BEGIN: clocktable :scope file\n".len(),
                "#+BEGIN: clocktable :scope file\nCONTENTS\n".len(),
                "#+BEGIN: clocktable :scope file\nCONTENTS\n#+END:\n".len(),
            ))
        );
    }
}
