# Changelog

All notable changes to `tc_pwhash` are documented in this file.

## 0.1.0 - 2026-11-26

Initial release.

### Added

- `argon2::Argon2`, Bouncy Castle's `Argon2BytesGenerator`, with
  `Argon2ParamsRef`, `Argon2Type`, `Argon2Version` and `Argon2Error`: Argon2d,
  Argon2i and Argon2id of RFC 9106, versions 0x10 and 0x13, with a secret and
  associated data.
- `bcrypt::Bcrypt`, Bouncy Castle's `BCrypt`, and `bcrypt::OpenBsdBcrypt`, its
  `OpenBsdBCrypt`, with `BcryptVersion` and `BcryptError`: the raw function
  and the `$2a$`, `$2b$` and `$2y$` hash strings, generated and checked.
- `scrypt::Scrypt`, Bouncy Castle's `SCrypt`, with `ScryptError`: scrypt of
  RFC 7914 over `tc_pbe`'s PBKDF2-HMAC-SHA256.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- `no_std` with `alloc`, and contains no `unsafe` code.
- Argon2d, Argon2id past the first half of its first pass, scrypt and bcrypt
  are variable time (cache timing) in the password; Argon2i's memory accesses
  depend on the parameters alone. Every working buffer is wiped before a call
  returns.
- `OpenBsdBcrypt::check_password` compares the hashes without branching on
  their bytes, where Bouncy Castle compares strings.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
