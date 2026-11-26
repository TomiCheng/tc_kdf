# tc_pwhash

[![crates.io](https://img.shields.io/crates/v/tc_pwhash.svg)](https://crates.io/crates/tc_pwhash)
[![docs.rs](https://docs.rs/tc_pwhash/badge.svg)](https://docs.rs/tc_pwhash)
[![CI](https://github.com/TomiCheng/tc_kdf/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_kdf/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

Password hashing after Bouncy Castle: deliberately slow functions that turn a
password and a salt into a hash to store and check a password against, or
into a key. `no_std` with `alloc`, no `unsafe`, Rust 1.85 or later.

## Types

- `argon2::Argon2` — Argon2d, Argon2i and Argon2id of RFC 9106, memory-hard.
- `argon2::Argon2ParamsRef` — the type, salt, passes, memory, lanes, secret and associated data.
- `bcrypt::OpenBsdBcrypt` — bcrypt as OpenBSD's `$2y$` hash strings, with the check.
- `bcrypt::Bcrypt` — the raw 24-byte bcrypt function.
- `scrypt::Scrypt` — scrypt of RFC 7914, memory-hard.
- `Argon2Error`, `BcryptError`, `ScryptError` — what each scheme turns down.

## Features

None; the crate always uses `alloc`.

## Usage

```rust
use tc_pwhash::bcrypt::OpenBsdBcrypt;

// Sign-up: a fresh random salt, and the string goes into the database.
let stored = OpenBsdBcrypt::generate(b"correct horse", &salt, 12)?;

// Login: the string holds the salt and cost, so the check needs nothing else.
assert!(OpenBsdBcrypt::check_password(&stored, b"correct horse")?);
```

## Security

Prefer Argon2id for new designs. Each guess costs time, and for Argon2 and
scrypt memory too, but a weak password still falls to a short list. Argon2d,
Argon2id after its first half pass, scrypt and bcrypt index memory or
S-boxes with values that depend on the password, so where a cache is shared
their timing can leak about it; each function's documentation states this.
bcrypt ignores password bytes past the 72nd. The working memory is wiped
before each call returns.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
