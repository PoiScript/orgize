#[cfg_attr(test, derive(PartialEq, Debug))]
// TODO: text-markup, entities, latex-fragments, subscript and superscript
pub struct RadioTarget<'a>(&'a str);

impl<'a> RadioTarget<'a> {
    pub fn parse(src: &'a str) -> Option<(RadioTarget<'a>, usize)> {
        starts_with!(src, "<<<");
        expect!(src, 3, |c| c != b' ');

        let end = until_while!(src, 3, b'>', |c| c != b'<' && c != b'\n');

        expect!(src, end - 1, |c| c != b' ');
        expect!(src, end + 1, b'>')?;
        expect!(src, end + 2, b'>')?;

        Some((RadioTarget(&src[3..end]), end + 3))
    }
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Target<'a>(&'a str);

impl<'a> Target<'a> {
    pub fn parse(src: &'a str) -> Option<(Target<'a>, usize)> {
        starts_with!(src, "<<");
        expect!(src, 2, |c| c != b' ');

        let end = until_while!(src, 2, b'>', |c| c != b'<' && c != b'\n');

        expect!(src, end - 1, |c| c != b' ');
        expect!(src, end + 1, b'>')?;

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
    parse_fail!(RadioTarget, "<<<target >>>");
    parse_fail!(RadioTarget, "<<< target>>>");
    parse_fail!(RadioTarget, "<<<ta<get>>>");
    parse_fail!(RadioTarget, "<<<ta>get>>>");
    parse_fail!(RadioTarget, "<<<ta\nget>>>");
    parse_fail!(RadioTarget, "<<target>>>");
    parse_fail!(RadioTarget, "<<<target>>");

    assert_eq!(
        Target::parse("<<target>>").unwrap(),
        (Target("target"), "<<target>>".len())
    );
    assert_eq!(
        Target::parse("<<tar get>>").unwrap(),
        (Target("tar get"), "<<tar get>>".len())
    );
    parse_fail!(Target, "<<target >>");
    parse_fail!(Target, "<< target>>");
    parse_fail!(Target, "<<ta<get>>");
    parse_fail!(Target, "<<ta>get>>");
    parse_fail!(Target, "<<ta\nget>>");
    parse_fail!(Target, "<target>>");
    parse_fail!(Target, "<<target>");
}
