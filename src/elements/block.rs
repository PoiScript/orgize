use memchr::{memchr, memchr_iter};

// return (name, args, contents-begin, contents-end, end)
#[inline]
pub fn parse(text: &str) -> Option<(&str, Option<&str>, usize, usize, usize)> {
    debug_assert!(text.starts_with("#+"));

    if text.len() <= 8 || text[2..8].to_uppercase() != "BEGIN_" {
        return None;
    }

    let mut lines = memchr_iter(b'\n', text.as_bytes());

    let (name, para, off) = lines
        .next()
        .map(|i| {
            memchr(b' ', &text.as_bytes()[8..i])
                .map(|x| (&text[8..8 + x], Some(text[8 + x..i].trim()), i + 1))
                .unwrap_or((&text[8..i], None, i + 1))
        })
        .filter(|(name, _, _)| name.as_bytes().iter().all(|&c| c.is_ascii_alphabetic()))?;

    let mut pos = off;
    let end = format!(r"#+END_{}", name.to_uppercase());

    for i in lines {
        if text[pos..i].trim().eq_ignore_ascii_case(&end) {
            return Some((name, para, off, pos, i + 1));
        }

        pos = i + 1;
    }

    if text[pos..].trim().eq_ignore_ascii_case(&end) {
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

        assert_eq!(
            parse("#+BEGIN_SRC\n#+END_SRC"),
            Some((
                "SRC",
                None,
                "#+BEGIN_SRC\n".len(),
                "#+BEGIN_SRC\n".len(),
                "#+BEGIN_SRC\n#+END_SRC".len()
            ))
        );
        assert_eq!(
            parse("#+BEGIN_SRC javascript  \nconsole.log('Hello World!');\n#+END_SRC\n"),
            Some((
                "SRC",
                Some("javascript"),
                "#+BEGIN_SRC javascript  \n".len(),
                "#+BEGIN_SRC javascript  \nconsole.log('Hello World!');\n".len(),
                "#+BEGIN_SRC javascript  \nconsole.log('Hello World!');\n#+END_SRC\n".len()
            ))
        );
        // TODO: more testing
    }
}
