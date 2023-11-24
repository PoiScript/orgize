## Format, test, lint

```shell
cargo fmt -- --check
cargo test --all-features
cargo clippy --allow-dirty --allow-staged
```

## Update snapshot testing

```shell
cargo install cargo-insta
cargo insta test --all-features
cargo insta review
```

## Fuzz testing

```shell
cargo install cargo-fuzz
rustup default nightly
cargo fuzz run fuzz_target_1
```

## Benchmark

```shell
curl -q https://orgmode.org/worg/doc.org --output ./benches/doc.org
curl -q https://orgmode.org/worg/org-faq.org --output ./benches/org-faq.org
curl -q https://orgmode.org/worg/org-hacks.org --output ./benches/org-hacks.org
curl -q https://orgmode.org/worg/org-release-notes.org --output ./benches/org-release-notes.org
curl -q https://orgmode.org/worg/org-syntax.org --output ./benches/org-syntax.org
curl -q https://raw.githubusercontent.com/bzg/org-mode/main/doc/org-manual.org --output ./benches/org-manual.org

cargo bench --bench parse
```

## Benchmark w/ flamegraph

```shell
cargo install flamegraph
cargo flamegraph --bench parse -o baseline.svg -- --bench
# then open baseline.svg with your browser
```
