# Changelog

## [1.0.1](https://github.com/qeeqez/snowid-postgres/compare/v1.0.0...v1.0.1) (2026-01-29)


### Bug Fixes

* **release:** proper version set ([8421cf0](https://github.com/qeeqez/snowid-postgres/commit/8421cf0ad8ad494bfdb65963f14e9dd10ee3da1e))

## [1.0.0](https://github.com/qeeqez/snowid-postgres/compare/v0.7.0...v1.0.0) (2026-01-30)

### ⚠ BREAKING CHANGES

*   **deps:** Upgraded internal `snowid` generator to v1.0.1. This brings massive performance improvements/optimizations but updates internal dependencies.

### Features

*   **performance:** Leveraging `snowid` v1.0.1 (Rust optimized) for ~20x faster time component generation and zero-allocation Base62 encoding within Postgres.
*   **ci:** Migrated to `release-please` for fully automated semantic releases and changelog management.
*   **ci:** Implemented robust release workflow that triggers Docker builds only when a release is officially created.

### Bug Fixes

*   **ci:** Fixed boolean input validation for Docker push actions in CI workflows.
*   **ci:** Resolved workflow triggers to prevent accidental tag-based builds.

### Miscellaneous

*   **deps:** Updated `heapless`, `pgrx` and other internal dependencies for better stability and compatibility with latest Postgres versions.
*   **docs:** Updated documentation to reflect 1.0.0 status.
