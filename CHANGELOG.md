# Changelog

## [2.3.6](https://github.com/qeeqez/snowid-postgres/compare/v2.3.5...v2.3.6) (2026-04-21)


### Bug Fixes

* strip false in the release profile ([0aac18f](https://github.com/qeeqez/snowid-postgres/commit/0aac18f79da12032b5f03a6d3c164506c192fa52))

## [2.3.5](https://github.com/qeeqez/snowid-postgres/compare/v2.3.4...v2.3.5) (2026-04-21)


### Bug Fixes

* proper release build command ([334cfab](https://github.com/qeeqez/snowid-postgres/commit/334cfab8e1f4de6eed853aa910fb05f7c0eccf18))

## [2.3.4](https://github.com/qeeqez/snowid-postgres/compare/v2.3.3...v2.3.4) (2026-04-21)


### Bug Fixes

* use release profile during build ([0587606](https://github.com/qeeqez/snowid-postgres/commit/0587606417fb5e97eacaa469b01953c79a093881))

## [2.3.3](https://github.com/qeeqez/snowid-postgres/compare/v2.3.2...v2.3.3) (2026-04-21)


### Bug Fixes

* do not allow rust compiler to strip sql migration scripts ([430f5f8](https://github.com/qeeqez/snowid-postgres/commit/430f5f877968ed57d198afd74a89a8d9f9ea739a))

## [2.3.2](https://github.com/qeeqez/snowid-postgres/compare/v2.3.1...v2.3.2) (2026-04-21)


### Bug Fixes

* missing sql files in the final extension ([8d44e64](https://github.com/qeeqez/snowid-postgres/commit/8d44e64c6da3b1687f3f5f45e48a3c097ff8d873))

## [2.3.1](https://github.com/qeeqez/snowid-postgres/compare/v2.3.0...v2.3.1) (2026-04-21)


### Bug Fixes

* bundle manual SQL migration scripts in Docker images and CI artifacts ([51f6a0a](https://github.com/qeeqez/snowid-postgres/commit/51f6a0a2a0fef5e64c44c31c15f2423762b5f140))

## [2.3.0](https://github.com/qeeqez/snowid-postgres/compare/v2.2.0...v2.3.0) (2026-04-21)


### Features

* optimize docker build for multi-arch and native performance ([3a8f6a9](https://github.com/qeeqez/snowid-postgres/commit/3a8f6a9cb74533770e6decf83dc7355d4ed4cca9))


### Bug Fixes

* add postgresql server package for pgrx initialization ([b0a18f2](https://github.com/qeeqez/snowid-postgres/commit/b0a18f216d9c521772f2cd3871a38a5452e893a8))


### Performance Improvements

* add caching for apt packages and cargo-pgrx binary ([2455c18](https://github.com/qeeqez/snowid-postgres/commit/2455c1834e4bb0bb0d84410f5996098caa3113d9))

## [2.2.0](https://github.com/qeeqez/snowid-postgres/compare/v2.1.2...v2.2.0) (2026-04-21)


### Features

* upgrade to pgrx 0.18.0 and update dependencies ([f7b7cae](https://github.com/qeeqez/snowid-postgres/commit/f7b7cae924e2273c895b96588bcefe0188a06491))


### Bug Fixes

* cleanup dev dockerfile and format code ([03aaabc](https://github.com/qeeqez/snowid-postgres/commit/03aaabc9c3ea9fcf1e4502fc1bd783a1e91ea8cf))
* resolve multi-arch build cache race and add missing ARGs ([29ca3b5](https://github.com/qeeqez/snowid-postgres/commit/29ca3b578d090743cd6b6d66aad34a7f54de2fc8))


### Performance Improvements

* use dummy source trick to fix docker caching ([e5c0728](https://github.com/qeeqez/snowid-postgres/commit/e5c0728f19e142ce7102f7a1b6e020fc649ffd28))

## [2.1.2](https://github.com/qeeqez/snowid-postgres/compare/v2.1.1...v2.1.2) (2026-03-31)


### Bug Fixes

* faster base62 ([183f100](https://github.com/qeeqez/snowid-postgres/commit/183f1003aed302cc1376c70e1f2fdf608751d852))

## [2.1.1](https://github.com/qeeqez/snowid-postgres/compare/v2.1.0...v2.1.1) (2026-02-26)


### Bug Fixes

* bump dependencies ([e488b21](https://github.com/qeeqez/snowid-postgres/commit/e488b21089f848b0b84c5d4a59a9495bc45ab82d))
* provide builds for multiple Postgres versions ([aedd8c0](https://github.com/qeeqez/snowid-postgres/commit/aedd8c032ae0297ba01d46ef96daeadfaaa9951a))
* release 2.1.1 ([6f6dbb9](https://github.com/qeeqez/snowid-postgres/commit/6f6dbb90b4a4421ab34d27be93baa5b1fa4d96d9))
* speedup builds with cache utilization ([77fd90d](https://github.com/qeeqez/snowid-postgres/commit/77fd90db29d2100cc22efe7e2df52f5fd9815cdf))

## [2.1.1](https://github.com/qeeqez/snowid-postgres/compare/v2.1.0...v2.1.1) (2026-02-26)


### Bug Fixes

* bump dependencies ([e488b21](https://github.com/qeeqez/snowid-postgres/commit/e488b21089f848b0b84c5d4a59a9495bc45ab82d))
* provide builds for multiple Postgres versions ([aedd8c0](https://github.com/qeeqez/snowid-postgres/commit/aedd8c032ae0297ba01d46ef96daeadfaaa9951a))
* release 2.1.1 ([6f6dbb9](https://github.com/qeeqez/snowid-postgres/commit/6f6dbb90b4a4421ab34d27be93baa5b1fa4d96d9))

## [2.1.0](https://github.com/qeeqez/snowid-postgres/compare/v2.0.0...v2.1.0) (2026-02-11)


### Features

* migrate to pgrx 0.17.0 ([00eaf2d](https://github.com/qeeqez/snowid-postgres/commit/00eaf2dc6c78508c0420adaa6c751c8c98166bde))

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
