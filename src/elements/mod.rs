pub mod fn_def;
pub mod keyword;
pub mod rule;

pub use self::fn_def::FnDef;
pub use self::keyword::Keyword;
pub use self::rule::Rule;

pub enum Element<'a> {
    Paragraph(&'a str),
}

impl<'a> Element<'a> {
    pub fn find_elem(src: &'a str) -> (Element<'a>, usize) {
        // TODO
        (Element::Paragraph(src), src.len())
    }
}
