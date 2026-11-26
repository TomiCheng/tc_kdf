# tc_kdf

A Rust workspace for key derivation after Bouncy Castle: the contracts that a
key derivation function implements, through which a secret and optional
context become key bytes, the functions over them, password-based
derivation for PBE, and password hashing. Each crate is
published separately and keeps its own README and changelog.

[![CI](https://github.com/TomiCheng/tc_kdf/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_kdf/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

## Crates

| Crate | Version | Description |
| --- | --- | --- |
| [`tc_kdf`](tc_kdf) | [![crates.io](https://img.shields.io/crates/v/tc_kdf.svg)](https://crates.io/crates/tc_kdf) [![docs.rs](https://docs.rs/tc_kdf/badge.svg)](https://docs.rs/tc_kdf) | The `Kdf` and `KdfInit` contracts after Bouncy Castle's `IDerivationFunction`, and KDF1, KDF2, the concatenation KDF, HKDF and the SP 800-108 KDFs. `no_std` with `alloc`, no `unsafe`. Constant time in the secret as far as the digest or MAC underneath is. |
| [`tc_pbe`](tc_pbe) | [![crates.io](https://img.shields.io/crates/v/tc_pbe.svg)](https://crates.io/crates/tc_pbe) [![docs.rs](https://docs.rs/tc_pbe/badge.svg)](https://docs.rs/tc_pbe) | Password-based key derivation for PBE after Bouncy Castle's `PbeParametersGenerator`: PBKDF1, PBKDF2, PKCS#12 and OpenSSL's `EVP_BytesToKey`, with the PKCS#5 and PKCS#12 password encoders. `no_std` with `alloc`, no `unsafe`. Constant time in the password and salt as far as the digest is. |
| [`tc_pwhash`](tc_pwhash) | [![crates.io](https://img.shields.io/crates/v/tc_pwhash.svg)](https://crates.io/crates/tc_pwhash) [![docs.rs](https://docs.rs/tc_pwhash/badge.svg)](https://docs.rs/tc_pwhash) | Password hashing after Bouncy Castle: Argon2 of RFC 9106, bcrypt with OpenBSD's hash strings, and scrypt of RFC 7914. `no_std` with `alloc`, no `unsafe`. Argon2d, Argon2id past its first half pass, scrypt and bcrypt are variable time (cache timing) in the password, as each states. |

## Requirements

Rust 1.85 or later, edition 2024. Every crate builds without `std`.
`tc_kdf`, `tc_pbe` and `tc_pwhash` always use `alloc`, for the copies of the
secret they keep and, in `tc_pwhash`, the working memory.

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

Publish `tc_pbe` before `tc_pwhash`, which depends on it. The archive must
include both license texts, the crate README, the
changelog, the source and the integration tests, and no `target/` or other
build artifacts.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
