use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use orgize::Org;

const INPUT: &[(&str, &str)] = &[
    ("doc.org", include_str!("./doc.org")),
    ("org-faq.org", include_str!("./org-faq.org")),
    ("org-hacks.org", include_str!("./org-hacks.org")),
    (
        "org-release-notes.org",
        include_str!("./org-release-notes.org"),
    ),
    ("org-syntax.org", include_str!("./org-syntax.org")),
];

pub fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("Org::parse");

    for (id, org) in INPUT {
        group.throughput(Throughput::Bytes(org.len() as u64));
        group.bench_with_input(*id, org, |b, i| b.iter(|| Org::parse(i)));
    }

    group.finish();
}

pub fn bench_to_html(c: &mut Criterion) {
    let mut group = c.benchmark_group("Org::to_html");

    for (id, org) in INPUT {
        group.throughput(Throughput::Bytes(org.len() as u64));
        group.bench_with_input(*id, &Org::parse(org), |b, i| b.iter(|| i.to_html()));
    }

    group.finish();
}

criterion_group!(benches, bench_parse, bench_to_html);
criterion_main!(benches);
