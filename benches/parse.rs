#![feature(test)]

extern crate test;

use orgize::Org;
use test::Bencher;

#[bench]
fn org_syntax(b: &mut Bencher) {
    // wget https://orgmode.org/worg/sources/dev/org-syntax.org
    b.iter(|| {
        Org::new(include_str!("org-syntax.org")).parse();
    })
}

#[bench]
fn doc(b: &mut Bencher) {
    // wget https://orgmode.org/worg/sources/doc.org
    b.iter(|| {
        Org::new(include_str!("doc.org")).parse();
    })
}

#[bench]
fn org_faq(b: &mut Bencher) {
    // wget https://orgmode.org/worg/sources/org-faq.org
    b.iter(|| {
        Org::new(include_str!("org-faq.org")).parse();
    })
}
