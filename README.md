# tc_kdf

A Rust workspace for key derivation after Bouncy Castle: the contracts that a
key derivation function implements, through which a secret and optional
context become key bytes, and the functions over them. Each crate is
published separately and keeps its own README and changelog.

[![CI](https://github.com/TomiCheng/tc_kdf/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_kdf/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

## Crates

| Crate | Version | Description |
| --- | --- | --- |
| [`tc_kdf`](tc_kdf) | [![crates.io](https://img.shields.io/crates/v/tc_kdf.svg)](https://crates.io/crates/tc_kdf) [![docs.rs](https://docs.rs/tc_kdf/badge.svg)](https://docs.rs/tc_kdf) | The `Kdf` and `KdfInit` contracts after Bouncy Castle's `IDerivationFunction`, and KDF1, KDF2, the concatenation KDF, HKDF and the SP 800-108 KDFs. `no_std` with `alloc`, no `unsafe`. Constant time in the secret as far as the digest or MAC underneath is. |

## Requirements

Rust 1.85 or later, edition 2024. Every crate builds without `std`.
`tc_kdf` always uses `alloc` for the copies of the secret it keeps.

Rust 1.85 is the earliest compiler for edition 2024, and it is guaranteed for
every build. The libraries depend on `tc_*` crates alone, all of which build
on 1.85. Dev-dependencies used only by tests are exempt. The workspace lock
tracks the latest dependency releases, so CI on stable tests what a user on a
current toolchain resolves.

## Workspace checks

```text
cargo test --locked
cargo test --locked --all-features
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo doc --locked --no-deps --all-features
```

CI additionally runs these on Linux x64, i686 and ARM64, macOS ARM64, and
Windows x64, checks the `wasm32-unknown-unknown` and `aarch64-unknown-none`
targets and each crate's dependency set on each target, checks the build on
Rust 1.85.0, and verifies the package archives. See
[.github/workflows/ci.yml](.github/workflows/ci.yml).

Before a release, check each archive and run publication validation from a
committed checkout:

```text
cargo package -p <crate> --list --locked
cargo publish -p <crate> --dry-run --locked
```

The archive must include both license texts, the crate README, the
changelog, the source and the integration tests, and no `target/` or other
build artifacts.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
