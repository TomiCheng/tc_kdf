# tc_kdf

[![crates.io](https://img.shields.io/crates/v/tc_kdf.svg)](https://crates.io/crates/tc_kdf)
[![docs.rs](https://docs.rs/tc_kdf/badge.svg)](https://docs.rs/tc_kdf)
[![CI](https://github.com/TomiCheng/tc_kdf/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_kdf/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

Key derivation functions after Bouncy Castle's `IDerivationFunction`: a KDF
turns a shared secret and optional context into as many key bytes as asked
for. `no_std` with `alloc`, no `unsafe`, Rust 1.85 or later.

## Types

- `kdf::Kdf1`, `kdf::Kdf2` — KDF1 and KDF2 of ISO 18033-2; KDF2 is the ANSI X9.63 KDF.
- `kdf::ConcatenationKdf` — the one-step concatenation KDF of SP 800-56C.
- `hkdf::Hkdf` — HKDF of RFC 5869, over HMAC.
- `sp800_108::KdfCounter`, `KdfFeedback`, `KdfDoublePipeline` — the SP 800-108 KDFs, over any MAC.
- Each family's params as a trait with a borrowed `…Ref` and an owned `…Owned` type.
- `KdfError` and `sp800_108::KdfInitError` — what the functions turn down.

## Traits

- `Kdf` — fills a buffer with derived bytes.
- `KdfInit` — initializes a function from its params.

## Features

None; the crate always uses `alloc`.

## Usage

```rust
use tc_kdf::hkdf::{Hkdf, HkdfParamsRef};
use tc_kdf::{Kdf, KdfInit};

let mut hkdf = Hkdf::new(Sha256Digest::new());
hkdf.init(&HkdfParamsRef::new(&shared_secret, &salt, b"protocol v1: key"))?;
let mut key = [0u8; 32];
hkdf.generate_bytes(&mut key)?;
```

## Security

A KDF runs once over its input, so it is for secrets with enough entropy,
such as a shared secret or a master key, not for passwords. The functions
copy the secret at `init` and wipe their copies when replaced and on drop.
They are constant time in the secret as far as the digest or MAC underneath
is; output lengths are public.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
