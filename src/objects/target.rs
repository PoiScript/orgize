use jetscii::Substring;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
// TODO: text-markup, entities, latex-fragments, subscript and superscript
pub struct RadioTarget<'a>(&'a str);

impl<'a> RadioTarget<'a> {
    pub fn parse(src: &'a str) -> Option<(RadioTarget<'a>, usize)> {
        debug_assert!(src.starts_with("<<<"));

        expect!(src, 3, |c| c != b' ')?;

        let end = Substring::new(">>>").find(src).filter(|&i| {
            src.as_bytes()[3..i]
                .iter()
                .all(|&c| c != b'<' && c != b'\n' && c != b'>')
        })?;

        expect!(src, end - 1, |c| c != b' ')?;

        Some((RadioTarget(&src[3..end]), end + 3))
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Target<'a>(&'a str);

impl<'a> Target<'a> {
    pub fn parse(src: &'a str) -> Option<(Target<'a>, usize)> {
        debug_assert!(src.starts_with("<<"));

        expect!(src, 2, |c| c != b' ')?;

        let end = Substring::new(">>").find(src).filter(|&i| {
            src.as_bytes()[2..i]
                .iter()
                .all(|&c| c != b'<' && c != b'\n' && c != b'>')
        })?;

        expect!(src, end - 1, |c| c != b' ')?;

        Some((Target(&src[2..end]), end + 2))
    }
}

#[test]
fn parse() {
    assert_eq!(
        RadioTarget::parse("<<<target>>>").unwrap(),
        (RadioTarget("target"), "<<<target>>>".len())
    );
    assert_eq!(
        RadioTarget::parse("<<<tar get>>>").unwrap(),
        (RadioTarget("tar get"), "<<<tar get>>>".len())
    );
    assert_eq!(RadioTarget::parse("<<<target >>>"), None);
    assert_eq!(RadioTarget::parse("<<< target>>>"), None);
    assert_eq!(RadioTarget::parse("<<<ta<get>>>"), None);
    assert_eq!(RadioTarget::parse("<<<ta>get>>>"), None);
    assert_eq!(RadioTarget::parse("<<<ta\nget>>>"), None);
    assert_eq!(RadioTarget::parse("<<<target>>"), None);

    assert_eq!(
        Target::parse("<<target>>").unwrap(),
        (Target("target"), "<<target>>".len())
    );
    assert_eq!(
        Target::parse("<<tar get>>").unwrap(),
        (Target("tar get"), "<<tar get>>".len())
    );
    assert_eq!(Target::parse("<<target >>"), None);
    assert_eq!(Target::parse("<< target>>"), None);
    assert_eq!(Target::parse("<<ta<get>>"), None);
    assert_eq!(Target::parse("<<ta>get>>"), None);
    assert_eq!(Target::parse("<<ta\nget>>"), None);
    assert_eq!(Target::parse("<<target>"), None);
}
