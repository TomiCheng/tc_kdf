# Changelog

All notable changes to `tc_kdf` are documented in this file.

## 0.1.0 - 2026-11-25

Initial release.

### Added

- `Kdf` and `KdfInit`, the contracts of a key derivation function after
  Bouncy Castle's `IDerivationFunction`, with initialization and output in
  separate traits.
- `kdf::Kdf1` and `kdf::Kdf2` over a `BaseKdf` with the counter start as a
  const parameter, Bouncy Castle's `BaseKdfBytesGenerator`, and
  `kdf::ConcatenationKdf`, its `ConcatenationKdfGenerator`, with
  `KdfParams`, `KdfParamsRef` and `KdfParamsOwned`.
- `hkdf::Hkdf`, Bouncy Castle's `HkdfBytesGenerator`, with `HkdfParams`,
  `HkdfParamsRef` and `HkdfParamsOwned`, including the expand-only mode of
  its `SkipExtractParameters`.
- `sp800_108::KdfCounter`, `KdfFeedback` and `KdfDoublePipeline`, Bouncy
  Castle's SP 800-108 generators, over any MAC of `tc_macs`, with their
  params traits and types and `KdfInitError` for a counter length other
  than 8, 16, 24 or 32 bits.
- `KdfError` for a function used before `init`, an output past the
  function's limit, and a failure of the MAC underneath.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- `no_std` with `alloc`, and contains no `unsafe` code.
- Constant time in the secret as far as the digest or MAC underneath is;
  output lengths are public.
- Where Bouncy Castle leaves a limit unchecked or lets a counter wrap, the
  functions stop at the limit of their standard: the concatenation KDF at
  `(2^32 - 1)` blocks, and the SP 800-108 counter mode at `(2^r - 1)`.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
