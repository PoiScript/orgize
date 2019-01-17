use regex::Regex;

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"^[ \t]*-{5,}[ \t]*\n?$").unwrap();
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Rule;

impl Rule {
    pub fn parse(src: &str) -> usize {
        RULE_REGEX.find(src).map(|m| m.end()).unwrap_or(0)
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
