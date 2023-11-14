use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use orgize::Org;

const INPUT: &[(&str, &str)] = &[
    ("doc.org", include_str!("./doc.org")),
    ("org-faq.org", include_str!("./org-faq.org")),
    ("org-syntax.org", include_str!("./org-syntax.org")),
];

pub fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse");

    for (id, json) in INPUT.iter() {
        group.bench_with_input(BenchmarkId::new("Rowan", id), json, |b, i| {
            b.iter(|| Org::parse(i))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
