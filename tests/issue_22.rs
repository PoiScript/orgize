use orgize::Org;

#[test]
fn whitespaces() {
    let org = Org::parse("       ");

    assert(&org);

    let org = Org::parse("\t \t  \n \t \t \n  \t");

    assert(&org);

    let org = Org::parse("\u{000b}\u{0085}\u{00a0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}");

    assert(&org);
}

fn assert(org: &Org) {
    assert_eq!(
        org.iter().count(),
        2,
        "should contains only one element - document"
    );
}
