pub struct Rule;

impl Rule {
    #[inline]
    // return offset
    pub(crate) fn parse(text: &str) -> Option<usize> {
        let (text, off) = memchr::memchr(b'\n', text.as_bytes())
            .map(|i| (text[..i].trim(), i + 1))
            .unwrap_or_else(|| (text.trim(), text.len()));

        if text.len() >= 5 && text.as_bytes().iter().all(|&c| c == b'-') {
            Some(off)
        } else {
            None
        }
    }
}

#[test]
fn parse() {
    assert_eq!(Rule::parse("-----"), Some("-----".len()));
    assert_eq!(Rule::parse("--------"), Some("--------".len()));
    assert_eq!(Rule::parse("   -----"), Some("   -----".len()));
    assert_eq!(Rule::parse("\t\t-----"), Some("\t\t-----".len()));
    assert_eq!(Rule::parse("\t\t-----\n"), Some("\t\t-----\n".len()));
    assert_eq!(Rule::parse("\t\t-----  \n"), Some("\t\t-----  \n".len()));
    assert_eq!(Rule::parse(""), None);
    assert_eq!(Rule::parse("----"), None);
    assert_eq!(Rule::parse("   ----"), None);
    assert_eq!(Rule::parse("  None----"), None);
    assert_eq!(Rule::parse("None  ----"), None);
    assert_eq!(Rule::parse("None------"), None);
    assert_eq!(Rule::parse("----None----"), None);
    assert_eq!(Rule::parse("\t\t----"), None);
    assert_eq!(Rule::parse("------None"), None);
    assert_eq!(Rule::parse("----- None"), None);
}
