#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Rule;

impl Rule {
    pub fn parse(src: &str) -> usize {
        let end = memchr::memchr(b'\n', src.as_bytes())
            .map(|i| i + 1)
            .unwrap_or_else(|| src.len());
        let rules = &src[0..end].trim();
        if rules.len() >= 5 && rules.chars().all(|c| c == '-') {
            end
        } else {
            0
        }
    }
}

#[test]
fn parse() {
    assert_eq!(Rule::parse("-----"), "-----".len());
    assert_eq!(Rule::parse("--------"), "--------".len());
    assert_eq!(Rule::parse("   -----"), "   -----".len());
    assert_eq!(Rule::parse("\t\t-----"), "\t\t-----".len());
    assert_eq!(Rule::parse("\t\t-----\n"), "\t\t-----\n".len());
    assert_eq!(Rule::parse("\t\t-----  \n"), "\t\t-----  \n".len());
    assert_eq!(Rule::parse(""), 0);
    assert_eq!(Rule::parse("----"), 0);
    assert_eq!(Rule::parse("   ----"), 0);
    assert_eq!(Rule::parse("  0----"), 0);
    assert_eq!(Rule::parse("0  ----"), 0);
    assert_eq!(Rule::parse("0------"), 0);
    assert_eq!(Rule::parse("----0----"), 0);
    assert_eq!(Rule::parse("\t\t----"), 0);
    assert_eq!(Rule::parse("------0"), 0);
    assert_eq!(Rule::parse("----- 0"), 0);
}
