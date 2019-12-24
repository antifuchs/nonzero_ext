# Changes for `nonzero_ext`

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added
* Support for `NonZeroI*` types - now `nonzero_ext` should include support for all non-zero integer types that the
 standard library exports.
* Support for using `nonzero!` in a [const context](https://doc.rust-lang.org/reference/const_eval.html).
* This [CHANGELOG](./CHANGELOG.md) file. I have tried to backfill the major changes since initial release, but there
 are bound to be gaps.

## [v0.1.3] - 2019-03-10

### Added
* Ability to use the `nonzero_ext` crate in `no_std` mode; to use it without the `std` library, disable default
 features when pulling this crate into your project.
