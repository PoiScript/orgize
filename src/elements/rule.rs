#[inline]
pub fn parse(text: &str) -> usize {
    let (text, off) = memchr::memchr(b'\n', text.as_bytes())
        .map(|i| (text[..i].trim(), i + 1))
        .unwrap_or_else(|| (text.trim(), text.len()));

    if text.len() >= 5 && text.as_bytes().iter().all(|&c| c == b'-') {
        off
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(parse("-----"), "-----".len());
        assert_eq!(parse("--------"), "--------".len());
        assert_eq!(parse("   -----"), "   -----".len());
        assert_eq!(parse("\t\t-----"), "\t\t-----".len());
        assert_eq!(parse("\t\t-----\n"), "\t\t-----\n".len());
        assert_eq!(parse("\t\t-----  \n"), "\t\t-----  \n".len());
        assert_eq!(parse(""), 0);
        assert_eq!(parse("----"), 0);
        assert_eq!(parse("   ----"), 0);
        assert_eq!(parse("  0----"), 0);
        assert_eq!(parse("0  ----"), 0);
        assert_eq!(parse("0------"), 0);
        assert_eq!(parse("----0----"), 0);
        assert_eq!(parse("\t\t----"), 0);
        assert_eq!(parse("------0"), 0);
        assert_eq!(parse("----- 0"), 0);
    }
}
