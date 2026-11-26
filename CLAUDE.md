# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Rules

- Comments and documentation are written in English only — doc comments,
  README, changelog, and inline comments alike.
- Never wire a README into rustdoc (`#![doc = include_str!("../README.md")]`).
  The README's badges and relative links do not survive rustdoc rendering.
  Crate documentation lives in `//!` and `///` comments; the README repeats what
  a reader on crates.io needs.
- No breaking changes. Public API work is additive: adding items, trait
  implementations, or default methods. If a change cannot be made additively,
  stop and raise it rather than altering an existing signature or behavior.
- Every crate README opens with the badge block: crates.io, docs.rs, CI,
  license, and rustc.
- Crate READMEs are short and written for crates.io: the badge block, a
  sentence or two on what the crate is, the "Types", "Traits" and "Features"
  lists, a usage example of ten lines or fewer, a short security note and the
  license. Behaviour belongs in the rustdoc, and validation and release
  commands in the root README.
- Crate READMEs use no Markdown tables; crates.io renders them badly. Traits,
  types and features are flat one-line bullets (`` `Item` — what it does. ``).

The crate list and workspace-wide checks live in the root
[README.md](README.md); read it rather than restating it here. Every crate is
`no_std`. `tc_kdf`, `tc_pbe` and `tc_pwhash` always use `alloc`, for the
copies of the secret they keep and, in `tc_pwhash`, the working memory.

This repository holds key derivation after Bouncy Castle: `IDerivationFunction`
for secrets with enough entropy, and `PbeParametersGenerator` for passwords,
which must be slow. The two families stay apart: neither crate depends on the
other.

`tc_kdf` holds the contracts and the general functions. `Kdf`, after
`IDerivationFunction`, fills a caller-provided buffer; `KdfInit<P>` takes the
parameters by reference. Initialization and output are separate traits, and
the traits make no timing promise; each implementation states its own. The
functions come in three public modules: `kdf` (KDF1, KDF2 and the
concatenation KDF over a digest), `hkdf` (HKDF over HMAC) and `sp800_108`
(counter, feedback and double-pipeline modes over any MAC). Each family's
parameters are a trait with a borrowed `…Ref` and an owned `…Owned` type, and
`KdfInit<P>` is implemented for every `P` of that trait. As in Bouncy Castle,
KDF1, KDF2 and the concatenation KDF start over on each `generate_bytes`
call, while HKDF and SP 800-108 continue. Where Bouncy Castle leaves a limit
unchecked or lets a counter wrap, the functions stop at the limit of their
standard.

`tc_pbe` holds password-based derivation for PBE: `PbeGenerator` derives a
key, a key and IV, or a MAC key into caller-provided buffers, and
`PbeGeneratorInit<P>` takes the password, salt and iteration count. The
schemes are `Pkcs5S1` (PBKDF1), `Pkcs5S2` (PBKDF2 over HMAC), `Pkcs12` (RFC
7292 appendix B) and `OpenSslPbe` (`EVP_BytesToKey`), with Bouncy Castle's
password encoders as free functions. Every call derives from the start, as in
Bouncy Castle. An iteration count of 0 is refused, except by `OpenSslPbe`,
which ignores the count.

`tc_pwhash` holds password hashing: Argon2, bcrypt with OpenBSD's hash
strings, and scrypt, one public module each, with no shared trait, as Bouncy
Castle has none. scrypt runs on `tc_pbe`'s PBKDF2, the one sibling dependency
in the workspace. Argon2d, Argon2id past its first half pass, scrypt and
bcrypt read memory or S-boxes at indices that depend on the password; that is
how they are memory-hard or how Blowfish works, so it is disclosed as variable
time (cache timing) rather than avoided.

`tc_kdf`, `tc_pbe` and `tc_pwhash` each have a `tests/documentation.rs` that
requires the declarations it lists to say "constant time" or "variable time",
exactly one, matched without case and across wrapped lines; `tc_pwhash`
checks every function, private helpers included, since they sit on the
password's path. Trait declarations, which end in `;`, are exempt. Keep the
timing contract of each item stated in its doc comment, and extend the list
when adding declarations.

`unsafe` code is forbidden at every crate root.

Rust 1.85 is guaranteed for every build. The libraries depend on `tc_*`
crates alone; dev-dependencies are exempt, so tests run on stable.
`.cargo/config.toml` sets `incompatible-rust-versions = "allow"` so
`Cargo.lock` tracks the latest releases. Adding a third-party dependency to a
library hands the 1.85 guarantee to that crate; raise it before doing so.

A crate depends on a workspace sibling through `path` plus `version`, so the
workspace builds and tests against the local crate while the published package
requires the release. When a change needs a sibling API that is not released
yet, raise the `version` requirement to the release that adds it; that release
has to be published first.

## Conventions

Each crate ships its own `README.md`, `CHANGELOG.md`, `LICENSE-MIT`,
`LICENSE-APACHE`, and an explicit `include` list in `Cargo.toml`. Changelog
entries are written as `## <version> - Unreleased` and dated in a separate
commit at release, with `### Added` and `### Compatibility` sections.

Adding a crate to the workspace means five edits beyond the crate itself: the
`members` list, a `-p <crate>` on the single `cargo package --locked` step in
the CI `quality` job (the only place package archives are verified; packaging
the crates in one invocation checks each against its siblings' local sources
rather than their releases), a `cargo tree` check of its dependency set in the
CI `portable` job, a row in the root `README.md`, and
workspace inheritance for `edition`, `rust-version`, `license`, and
`repository`. A missing `rust-version` also leaves clippy suggesting APIs newer
than 1.85.

Documentation is part of the contract: crates use `#![deny(missing_docs)]`,
doctests carry the executable examples, and CI runs `cargo doc` with
`RUSTDOCFLAGS: -D warnings`, with and without `--all-features`. Doc links to
feature-gated items break the build without that feature, so name them in plain
code spans. An additive public API change belongs in the crate README's
contract lists — "Types", "Traits" and "Features" in
each crate's `README.md` — and in the changelog, not only in the code.

Work happens on `feat/*` branches off `develop`; pull requests target `develop`,
which merges to `main`. Commit messages use an imperative subject and a wrapped
body that explains the reasoning, not a bullet list of the diff.
