use orgize::Org;

const ORG_STR: &str = r#"

#+TITLE: org

#+BEGIN_QUOTE

CONTENTS

#+END_QUOTE

* Headline 1
SCHEDULED: <2019-10-28 Mon>
:PROPERTIES:
:ID: headline-1
:END:

:LOGBOOK:

CLOCK: [2019-10-28 Mon 08:53]

CLOCK: [2019-10-28 Mon 08:53]--[2019-10-28 Mon 08:53] => 0:00

:END:

-----

#+CALL: VALUE

#
# Comment
#

#+BEGIN: NAME PARAMETERS

CONTENTS

#+END:

:
: Fixed width
:

#+BEGIN_COMMENT

COMMENT

#+END_COMMENT

#+BEGIN_EXAMPLE
#+END_EXAMPLE

"#;

#[test]
fn blank() {
    let org = Org::parse(ORG_STR);

    let mut writer = Vec::new();
    org.org(&mut writer).unwrap();

    // eprintln!("{}", serde_json::to_string_pretty(&org).unwrap());

    assert_eq!(String::from_utf8(writer).unwrap(), ORG_STR);
}
