use orgize::elements::Title;
use orgize::Org;
use pretty_assertions::assert_eq;
use serde_json::to_string;

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

#[test]
fn insert() {
    let mut org = Org::new();
    let document = org.document();

    let h1 = org.new_headline(Title {
        level: 1,
        raw: "title".into(),
        ..Default::default()
    });
    h1.set_section_content("section", &mut org);
    document.prepend(h1, &mut org).unwrap();
    dbg!(to_string(&org).unwrap());

    let h3 = org.new_headline(Title {
        level: 3,
        raw: "title".into(),
        ..Default::default()
    });
    h3.set_section_content("section", &mut org);
    document.prepend(h3, &mut org).unwrap();
    dbg!(to_string(&org).unwrap());

    let h2 = org.new_headline(Title {
        level: 2,
        raw: "title".into(),
        ..Default::default()
    });
    h2.set_section_content("section", &mut org);
    h1.insert_before(h2, &mut org).unwrap();
    dbg!(to_string(&org).unwrap());

    let mut writer = Vec::new();
    org.html(&mut writer).unwrap();
    assert_eq!(
        String::from_utf8(writer).unwrap(),
        "<main><h3>title</h3><section><p>section</p></section>\
         <h2>title</h2><section><p>section</p></section>\
         <h1>title</h1><section><p>section</p></section></main>"
    );
}
