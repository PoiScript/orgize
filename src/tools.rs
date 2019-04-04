use crate::elements::{fn_def, keyword, Key};
use crate::headline::{Headline, DEFAULT_KEYWORDS};
use memchr::memchr;

type Headlines<'a> = Vec<Headline<'a>>;
type Keywords<'a> = Vec<(Key<'a>, &'a str)>;
type Footnotes<'a> = Vec<&'a str>;

pub fn metadata(src: &str) -> (Headlines<'_>, Keywords<'_>, Footnotes<'_>) {
    let mut headlines = Vec::new();
    let mut keywords = Vec::new();
    let mut footnotes = Vec::new();

    for line in src.lines().filter(|l| !l.is_empty()) {
        if line.starts_with('*') {
            let level = memchr(b' ', line.as_bytes()).unwrap_or_else(|| line.len());
            if line.as_bytes()[0..level].iter().all(|&c| c == b'*') {
                headlines.push(Headline::parse(line, DEFAULT_KEYWORDS).0)
            }
        } else if line.starts_with("#+") {
            if let Some((key, value, _)) = keyword::parse(line) {
                keywords.push((key, value))
            }
        } else if line.starts_with("[fn:") {
            if let Some((label, _, _)) = fn_def::parse(line) {
                footnotes.push(label)
            }
        }
    }

    (headlines, keywords, footnotes)
}

pub fn toc(src: &str) -> Headlines<'_> {
    metadata(src).0
}

pub fn keywords(src: &str) -> Keywords<'_> {
    metadata(src).1
}

pub fn fn_def(src: &str) -> Footnotes<'_> {
    metadata(src).2
}
