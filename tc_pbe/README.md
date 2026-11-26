# tc_pbe

[![crates.io](https://img.shields.io/crates/v/tc_pbe.svg)](https://crates.io/crates/tc_pbe)
[![docs.rs](https://docs.rs/tc_pbe/badge.svg)](https://docs.rs/tc_pbe)
[![CI](https://github.com/TomiCheng/tc_kdf/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_kdf/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

Password-based key derivation for PBE after Bouncy Castle's
`PbeParametersGenerator`: a password, a salt and an iteration count give a
cipher key, optionally with an IV, or a MAC key. `no_std` with `alloc`, no
`unsafe`, Rust 1.85 or later.

## Types

- `Pkcs5S2` — PBKDF2 of RFC 8018 over HMAC, the scheme for new designs.
- `Pkcs5S1` — PBKDF1 of RFC 8018, for PBES1.
- `Pkcs12` — the key derivation of RFC 7292 appendix B, for `.p12` files.
- `OpenSslPbe` — OpenSSL's `EVP_BytesToKey`, for old OpenSSL-encrypted PEM.
- `PbeParamsRef`, `PbeParamsOwned` — the password, salt and iteration count.
- `PbeError` — what the generators turn down.

## Traits

- `PbeGenerator` — derives a key, a key and IV, or a MAC key.
- `PbeGeneratorInit` — initializes a generator from its params.
- `PbeParams` — the params a generator reads.

## Features

None; the crate always uses `alloc`.

## Usage

```rust
use tc_pbe::{PbeGenerator, PbeGeneratorInit, PbeParamsRef, Pkcs5S2};

let mut pbkdf2 = Pkcs5S2::new(Sha256Digest::new());
pbkdf2.init(&PbeParamsRef::new(&password, &salt, 600_000))?;
let mut key = [0u8; 32];
let mut iv = [0u8; 16];
pbkdf2.derive_key_and_iv(&mut key, &mut iv)?;
```

## Security

The salt and the iteration count are public and travel with the ciphertext;
only the password is secret. Use a fresh random salt for each piece of data.
The iteration count slows each guess but does not make a weak password
strong, and for storing password hashes a memory-hard function such as
Argon2 resists parallel guessing better. The generators are constant time in
the password and salt as far as the digest is; the PKCS#5 and PKCS#12
password encoders are variable time, as their documentation states.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
