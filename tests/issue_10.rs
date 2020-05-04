use orgize::Org;

#[test]
fn can_handle_empty_emphasis() {
    let cases = &[
        "* / // a",
        "\"* / // a\"",
        "* * ** a",
        "* 2020\n** December\n*** Experiment\nType A is marked with * and type B is marked with **.\n",
        "* 2020\n:DRAWER:\n* ** a\n:END:",
        "* * ** :a:",
        "* * ** "
    ];

    for case in cases {
        let _ = Org::parse(case);
    }
}
