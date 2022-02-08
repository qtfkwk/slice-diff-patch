This crate provides the [`Change`] enum as an abstraction for [`diff::Result`],
[`lcs_diff::DiffResult`], and [`wu_diff::DiffResult`]; the [`diff_changes()`], [`diff_diff()`],
[`lcs_changes()`], [`lcs_diff()`], [`wu_changes()`], and [`wu_diff()`] functions to calculate or
process diffs between `a` and `b` slices via LCS (Longest Common Subsequence) or Wu diff algorithms
into a [`Vec<Change>`], and the [`patch()`] function to reproduce `b` from the `a` slice and
[`Vec<Change>`].

See also:

* Hunt, James W; Szymanski, Thomas G. (1977). "A fast algorithm for computing longest common
  subsequences" <http://www.cs.ust.hk/mjg_lib/bibs/DPSu/DPSu.Files/HuSz77.pdf>
* Wu, Sun; Manber, Udi; Myers, Gene (1989). "An O(NP) Sequence Comparison Algorithm"
  <https://publications.mpi-cbg.de/Wu_1990_6334.pdf>
* Department of Mathematics and Computer Science. University of Southern Denmark
  (January 12, 2017). "The Hunt-Szymanski Algorithm for LCS"
  <https://imada.sdu.dk/~rolf/Edu/DM823/E16/HuntSzymanski.pdf>
* [diff crate](https://crates.io/crates/diff)
* [lcs-diff crate](https://crates.io/crates/lcs-diff)
* [wu-diff crate](https://crates.io/crates/wu-diff)
* [Wikipedia: Hunt–Szymanski algorithm](https://en.wikipedia.org/wiki/Hunt%E2%80%93Szymanski_algorithm)
* [Wikipedia: Bitap algorithm](https://en.wikipedia.org/wiki/Bitap_algorithm)
* [Practical use case analysis](https://github.com/bokuweb/wu-diff-rs/issues/7)

Changelog:

* 0.1.0: initial
* 0.2.0: add support for [diff crate](https://crates.io/crates/diff)
* 0.2.1: fix readme
* 0.2.2: fix readme
* 0.3.0: derive `Clone` on [`Change`]
* 1.0.0: add `Update` variant on [`Change`]

[`Change`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/enum.Change.html
[`diff_changes()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.lcs_changes.html
[`diff::Result`]: https://docs.rs/lcs-diff/latest/lcs_diff/enum.DiffResult.html
[`diff_diff()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.lcs_diff.html
[`lcs_changes()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.lcs_changes.html
[`lcs_diff::DiffResult`]: https://docs.rs/lcs-diff/latest/lcs_diff/enum.DiffResult.html
[`lcs_diff()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.lcs_diff.html
[`patch()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.patch.html
[`Vec<Change>`]: https://doc.rust-lang.org/1.58.1/alloc/vec/struct.Vec.html
[`wu_changes()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.wu_changes.html
[`wu_diff::DiffResult`]: https://docs.rs/wu-diff/latest/wu_diff/enum.DiffResult.html
[`wu_diff()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.wu_diff.html

