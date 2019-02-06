#![feature(test)]

extern crate orgize;
extern crate test;

use orgize::Parser;
use test::Bencher;

#[bench]
fn org_syntax(b: &mut Bencher) {
    // wget https://orgmode.org/worg/sources/dev/org-syntax.org
    b.iter(|| {
        let _ = Parser::new(include_str!("org-syntax.org")).collect::<Vec<_>>();
    })
}

#[bench]
fn doc(b: &mut Bencher) {
    // wget https://orgmode.org/worg/sources/doc.org
    b.iter(|| {
        let _ = Parser::new(include_str!("doc.org")).collect::<Vec<_>>();
    })
}

#[bench]
fn org_faq(b: &mut Bencher) {
    // wget https://orgmode.org/worg/sources/org-faq.org
    b.iter(|| {
        let _ = Parser::new(include_str!("org-faq.org")).collect::<Vec<_>>();
    })
}
