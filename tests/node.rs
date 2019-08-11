use orgize::Org;
use pretty_assertions::assert_eq;

#[test]
fn set_content() {
    let mut org = Org::parse(
        r#"* title 1
section 1
** title 2
"#,
    );
    let headlines: Vec<_> = org.headlines().collect();
    for headline in headlines {
        headline.set_title_content(String::from("a *bold* title"), &mut org);
        headline.set_section_content("and a _underline_ section", &mut org);
    }
    let mut writer = Vec::new();
    org.html(&mut writer).unwrap();
    assert_eq!(
        String::from_utf8(writer).unwrap(),
        "<main><h1>a <b>bold</b> title</h1><section><p>and a <u>underline</u> section</p></section>\
         <h2>a <b>bold</b> title</h2><section><p>and a <u>underline</u> section</p></section></main>"
    );
}
