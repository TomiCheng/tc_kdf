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
`no_std`. `tc_kdf` always uses `alloc`, for the copies of the secret it keeps.

This repository holds key derivation after Bouncy Castle's
`IDerivationFunction`: functions that turn a secret with enough entropy into
key bytes. Password-based derivation, which must be slow, is not a KDF here.

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

`tc_kdf` has a `tests/documentation.rs` that requires the declarations it
lists to say "constant time" or "variable time", exactly one, matched without
case and across wrapped lines. Trait declarations, which end in `;`, are
exempt. Keep the timing contract of each item stated in its doc comment, and
extend the list when adding declarations.

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
