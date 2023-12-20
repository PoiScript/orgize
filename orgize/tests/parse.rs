const INPUT: &[&str] = &[
    // issue 10
    "* / // a",
    "\"* / // a\"",
    "* * ** a",
    "* 2020\n** December\n*** Experiment\nType A is marked with * and type B is marked with **.\n",
    "* 2020\n:DRAWER:\n* ** a\n:END:",
    "* * ** :a:",
    "* * ** ",
    // issue 11
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
    // issue 22
    "\u{000b}\u{0085}\u{00a0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}",
    // fuzz test
    "___\n",
    "\n\n\n",
    "\n*",
    "\r-",
    "6\r\n",
    "|\n\u{b}|"
];

#[test]
fn parse() {
    for input in INPUT {
        let _ = orgize::Org::parse(input);
    }
}
