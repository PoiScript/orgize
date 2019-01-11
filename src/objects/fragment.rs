#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Fragment<'a> {
    value: &'a str,
}

impl<'a> Fragment<'a> {
    pub fn parse(src: &'a str) -> Option<(Fragment<'a>, usize)> {
        None
    }
}
