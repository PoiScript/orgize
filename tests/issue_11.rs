use orgize::Org;

#[test]
fn can_handle_empty_list_item() {
    let cases = &[
        "0. ",
        "* \n0. ",
        " * ",
        " 0. ",
        "\t* ",
        "- ",
        "- hello\n- ",
        "- \n- hello",
        "- hello\n- \n- world",
        "* world\n- ",
    ];

    for case in cases {
        let _ = Org::parse(case);
    }
}
