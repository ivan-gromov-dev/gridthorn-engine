# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Cargo workspace with Rust 1.85.0 as the MSRV.
- Initial `gridthorn` SDK facade and version API.
- `gridthorn` CLI with `new`, `run`, `check`, and `--version`.
- Embedded minimal project template and generation/build tests.
- Formatting, lint, test, dependency-boundary, and CI checks.
- Scoped `AGENTS.md` guidance and domain-owned source/test organization.
- Matching Windows and macOS/Linux verification scripts plus three-platform CI
  with an aggregate `CI Success` gate.
- Repository-scoped `$implement` and `$release` skills for tested feature work
  and consistent release preparation.
