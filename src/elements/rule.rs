#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Rule;

impl Rule {
    pub fn parse(src: &str) -> Option<usize> {
        let end = eol!(src);
        let leading = until_while!(src, 0, b'-', |c| c == b' ' || c == b'\t')?;
        if src[leading..end].chars().all(|c| c == '-') && end - leading > 4 {
            Some(end)
        } else {
            None
        }
    }
}

#[test]
fn parse() {
    assert_eq!(Rule::parse("-----").unwrap(), "-----".len());
    assert_eq!(Rule::parse("--------").unwrap(), "--------".len());
    assert_eq!(Rule::parse("   -----").unwrap(), "   -----".len());
    assert_eq!(Rule::parse("\t\t-----").unwrap(), "\t\t-----".len());

    assert!(Rule::parse("").is_none());
    assert!(Rule::parse("----").is_none());
    assert!(Rule::parse("   ----").is_none());
    assert!(Rule::parse("  0----").is_none());
    assert!(Rule::parse("0  ----").is_none());
    assert!(Rule::parse("0------").is_none());
    assert!(Rule::parse("----0----").is_none());
    assert!(Rule::parse("\t\t----").is_none());
    assert!(Rule::parse("------0").is_none());
    assert!(Rule::parse("----- 0").is_none());
}
