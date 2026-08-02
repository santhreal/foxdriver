# Changelog

## [0.1.2] - 2026-08-02

### Fixed
- `to_curl` now single-quotes the captured HTTP method like every other captured value. Before this fix, a hostile server advertising a crafted method token could inject extra shell arguments into the generated curl command.
- New adversarial suite covering command substitution and backticks in captured values, single-quote breakout escaping, hostile method tokens, malformed JSON bodies, and malformed URLs.

## [0.1.1] - 2026-07-30

- Refined pass: metadata, docs, and test hardening for the crates.io release.
