use orgize::Org;

#[test]
fn headline_in_drawer() {
    // https://github.com/PoiScript/orgize/issues/24
    // A drawer may not contain a headline.
    const STARS: &str = "****";
    for h1 in 1..STARS.len() {
        for h2 in 1..STARS.len() {
            let org = crate::Org::parse_string(format!(
                "{} Hello\n:PROPERTIES:\n{} World\n:END:",
                &STARS[..h1],
                &STARS[..h2]
            ));
            assert_eq!(org.headlines().count(), 2);
        }
    }
}
