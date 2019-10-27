use orgize::{elements::Title, Headline, Org};

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
    for mut headline in headlines {
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
    let org = &mut Org::new();
    let mut document = org.document();

    let mut h1 = Headline::new(
        Title {
            level: 1,
            raw: "title".into(),
            ..Default::default()
        },
        org,
    );
    h1.set_section_content("section", org);
    document.prepend(h1, org).unwrap();

    let mut h3 = Headline::new(
        Title {
            level: 3,
            raw: "title".into(),
            ..Default::default()
        },
        org,
    );
    h3.set_section_content("section", org);
    document.prepend(h3, org).unwrap();

    let mut h2 = Headline::new(
        Title {
            level: 2,
            raw: "title".into(),
            ..Default::default()
        },
        org,
    );
    h2.set_section_content("section", org);
    h1.insert_before(h2, org).unwrap();

    document.set_section_content("section", org);

    let mut writer = Vec::new();
    org.html(&mut writer).unwrap();
    assert_eq!(
        String::from_utf8(writer).unwrap(),
        "<main><section><p>section</p></section>\
         <h3>title</h3><section><p>section</p></section>\
         <h2>title</h2><section><p>section</p></section>\
         <h1>title</h1><section><p>section</p></section></main>"
    );
}
