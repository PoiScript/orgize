#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Time<'a> {
    pub date: &'a str,
}

pub enum Timestamp<'a> {
    ActiveRange,
}
