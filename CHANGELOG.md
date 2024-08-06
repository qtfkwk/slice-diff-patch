# Changelog

* 0.1.0 (2022-02-05): Initial
* 0.2.0 (2022-02-05): Add support for [diff crate](https://crates.io/crates/diff)
    * 0.2.1 (2022-02-05): Fix readme
    * 0.2.2 (2022-02-05): Fix readme
* 0.3.0 (2022-02-05): Derive `Clone` on [`Change`]
* 1.0.0 (2022-02-07): Add `Update` variant on [`Change`]
    * 1.1.0 (2022-02-08): Add doc test example
        * 1.1.1 (2022-02-08): Fix readme
    * 1.2.1 (2022-02-08): Expose [`remove()`] and [`insert()`] functions
    * 1.2.2 (2024-07-22): Update dependencies; rustfmt
    * 1.2.3 (2024-08-06): Add `Makefile.md`, `Cargo.lock`; fix changelog

[`Change`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/enum.Change.html
[`insert()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.insert.html
[`remove()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.remove.html

