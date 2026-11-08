# LogScope

A deliberately small Rust workspace for learning production-oriented Rust engineering:
workspace design, build scripts, tests, benchmarks, feature flags, CI, coverage,
dependency auditing, cross-compilation, Miri, and release automation.

## What it does

LogScope reads lines in this format:

```text
TIMESTAMP LEVEL MESSAGE
2026-11-07T10:00:00 INFO Server started
2026-11-07T10:00:01 WARN High memory usage
2026-11-07T10:00:02 ERROR Database connection failed
```

It counts log levels and calculates warning/error rates. Invalid lines are skipped
and counted.

## Requirements

- Rust stable toolchain (see `rust-toolchain.toml`)
- Git
- Optional: `cargo-make`, `cargo-llvm-cov`, `cargo-audit`, `cargo-deny`, `cross`

## Setup

Create a sample log:

```bash
cat > sample.log <<'EOF'
2026-11-07T10:00:00 INFO Server started
2026-11-07T10:00:01 WARN High memory usage
2026-11-07T10:00:02 ERROR Database connection failed
2026-11-07T10:00:03 INFO Retrying connection
EOF
```

Run:

```bash
cargo run -p logscope -- analyze sample.log
cargo run -p logscope -- analyze sample.log --level error
cargo run -p logscope -- info
```

JSON output is an optional compile-time feature:

```bash
cargo run -p logscope --features json -- analyze sample.log
```

## Workspace

- `logscope-core`: parsing, domain types, analysis, build metadata.
- `logscope`: command-line interface and filesystem interaction.

The core crate deliberately does not know about CLI arguments or terminal output.
This separation makes the logic easy to unit-test and reuse.

## Local quality gates

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Optional task runner:

```bash
cargo install cargo-make
cargo make dev
```

Enable the repository's pre-commit hook:

```bash
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit
```

## Engineering exercises

1. Add a `--contains TEXT` filter.
2. Add support for `TRACE` or `FATAL`.
3. Add a `--top N` option for most common error messages.
4. Add property-based tests for the parser.
5. Add Criterion benchmarks for parsing a whole file.
6. Raise coverage threshold after adding tests.
7. Add a feature that exports JSON Lines.
8. Add a reproducible-build test using `SOURCE_DATE_EPOCH`.
9. Configure a scheduled nightly Miri job.
10. Configure `cargo-dist` to publish binaries on version tags.

## Tooling commands

```bash
cargo bench -p logscope-core
cargo install cargo-llvm-cov && cargo llvm-cov --workspace --all-features --html
cargo install cargo-audit && cargo audit
cargo install cargo-deny && cargo deny check
cargo install cross --locked
```

Miri requires nightly:

```bash
rustup toolchain install nightly --component miri
cargo +nightly miri setup
cargo +nightly miri test -p logscope-core
```

Cross-build examples:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release -p logscope --target x86_64-unknown-linux-musl

# For aarch64-unknown-linux-gnu, use cross:
cross build --release -p logscope --target aarch64-unknown-linux-gnu
```

## CI pipeline

Every push and pull request to `main` runs:

1. Fast feedback: formatting, compilation, Clippy.
2. Tests: Linux, macOS, Windows.
3. Coverage: `cargo-llvm-cov`, minimum 60% line coverage for the core library.
4. Security: `cargo audit` and `cargo deny`.
5. Cross-build: Linux musl x86_64 and Linux GNU aarch64.
6. Miri: interpreter-based checks for the core crate.

The expensive jobs wait for the fast check gate, then run independently. Miri runs on a weekly schedule or manual dispatch; Criterion benchmarks run on pull requests.

## Build metadata

`logscope info` reports the Git SHA and build timestamp embedded by `build.rs`.
Set `SOURCE_DATE_EPOCH` to provide a stable timestamp for reproducible builds.
The timestamp is a Unix epoch value, not a formatted date.

## License

MIT
