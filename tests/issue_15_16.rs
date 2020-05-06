use orgize::Org;

#[test]
fn bad_headline_tags() {
    contains_no_tag(Org::parse("* a ::"));

    contains_no_tag(Org::parse("* a :(:"));

    contains_one_tag(Org::parse("* a \t:_:"), "_");

    contains_one_tag(Org::parse("* a \t :@:"), "@");

    contains_one_tag(Org::parse("* a :#:"), "#");

    contains_one_tag(Org::parse("* a\t :%:"), "%");

    contains_one_tag(Org::parse("* a :余:"), "余");
}

fn contains_no_tag(org: Org) {
    assert!(org.headlines().next().unwrap().title(&org).tags.is_empty());
}

fn contains_one_tag(org: Org, tag: &str) {
    assert_eq!(vec![tag], org.headlines().next().unwrap().title(&org).tags);
}
