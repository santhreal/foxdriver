# Changelog


## [0.1.4] - 2026-08-07

### Changed
- Standardized `Cargo.toml` authors field to `Santh <64453045+santhreal@users.noreply.github.com>`.

### Fixed
- Fixed key resolution in WebDriver BiDi `ObjectRemoteValue` and wire-format object decoding (`bidi_wire_value_to_json` and `remote_value_to_json`) so object properties with wire-format key objects are preserved instead of silently dropped.
- Fixed case-sensitive matching in cookie `SameSite` attribute resolution (`cookies::apply_to_page`) so capitalized values ("Strict", "Lax", "None") are mapped properly instead of being dropped.
## [0.1.3] - 2026-08-07

### Added
- `SPEC.md` defining technical architecture, event stream invariants, and frame coordinate conversion guarantees.
- `README.md` and `package.metadata.santh` status badge and metadata (`beta`) in `Cargo.toml`.
- Folder contract alignment with `Cargo.lock` and standard `lib.rs` lint preamble (`missing_docs`, `clippy::pedantic`, `#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used, clippy::todo, clippy::unimplemented, clippy::panic))]`).

### Fixed
- Removed non-test `expect()` call in `browser.rs::launch_firefox` profile directory resolution path for full `#![cfg_attr(not(test), deny(clippy::expect_used))]` lint preamble compliance.

## [0.1.2] - 2026-08-02

### Fixed
- `to_curl` now single-quotes the captured HTTP method like every other captured value. Before this fix, a hostile server advertising a crafted method token could inject extra shell arguments into the generated curl command.
- New adversarial suite covering command substitution and backticks in captured values, single-quote breakout escaping, hostile method tokens, malformed JSON bodies, and malformed URLs.

## [0.1.1] - 2026-07-30

- Refined pass: metadata, docs, and test hardening for the crates.io release.
