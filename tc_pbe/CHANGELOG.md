# Changelog

All notable changes to `tc_pbe` are documented in this file.

## 0.1.0 - Unreleased

Initial release.

### Added

- `PbeGenerator`, `PbeGeneratorInit` and `PbeParams`, the contracts of a
  password-based generator after Bouncy Castle's `PbeParametersGenerator`,
  with the lengths taken from the caller's buffers.
- `Pkcs5S1`, `Pkcs5S2`, `Pkcs12` and `OpenSslPbe`, Bouncy Castle's
  `Pkcs5S1ParametersGenerator`, `Pkcs5S2ParametersGenerator`,
  `Pkcs12ParametersGenerator` and `OpenSslPbeParametersGenerator`.
- `PbeParamsRef` and `PbeParamsOwned` for the password, salt and iteration
  count.
- `pkcs5_password_to_bytes`, `pkcs5_password_to_utf8_bytes` and
  `pkcs12_password_to_bytes`, Bouncy Castle's password encoders.
- `PbeError` for a generator used before `init`, an iteration count the
  scheme does not take, an output past its limit, and a failure of the HMAC
  under PBKDF2.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- `no_std` with `alloc`, and contains no `unsafe` code.
- Constant time in the password and salt as far as the digest is; the
  iteration count and lengths are public. The UTF-16 password encoders are
  variable time.
- `Pkcs5S1`, `Pkcs5S2` and `Pkcs12` refuse an iteration count of 0, which
  Bouncy Castle runs as 1. `OpenSslPbe` ignores the count and runs once, as
  Bouncy Castle and `openssl enc` without `-iter` do.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
