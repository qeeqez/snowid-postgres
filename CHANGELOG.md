# Changelog

## [2.0.0](https://github.com/qeeqez/snowid-postgres/compare/v1.0.1...v2.0.0) (2026-01-31)


### ⚠ BREAKING CHANGES

* **release:** snowid-rust 2.0.0
* **deps:** bump snowid to v1.0.1

### Features

* **deps:** bump snowid to v1.0.1 ([de4d0d6](https://github.com/qeeqez/snowid-postgres/commit/de4d0d699c1e2266cbf9fa533152aa8cdb44e4d5))
* **release:** snowid-rust 2.0.0 ([883c00e](https://github.com/qeeqez/snowid-postgres/commit/883c00e6a4dbf0d77d1546869e16c5e0577e2505))
* setup release-please for snowid-postgres ([ac307db](https://github.com/qeeqez/snowid-postgres/commit/ac307dbf0ebaa1b7442c25ef536371670ee9f173))


### Bug Fixes

* **ci:** ensure string inputs for docker push action ([2509bbf](https://github.com/qeeqez/snowid-postgres/commit/2509bbf6c6a03f627cccdeea934f9a8b24f90f58))
* **ci:** non triggering job on release ([f50b454](https://github.com/qeeqez/snowid-postgres/commit/f50b454d3034b022843532cd1ba25b7ef9015e91))
* **release:** proper version set ([8421cf0](https://github.com/qeeqez/snowid-postgres/commit/8421cf0ad8ad494bfdb65963f14e9dd10ee3da1e))


### Performance Improvements

* shared lock for reads, exclusive for writes ([fabc2c9](https://github.com/qeeqez/snowid-postgres/commit/fabc2c92cd9efac41bdb782ee2d4ca2aee93d28d))

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
