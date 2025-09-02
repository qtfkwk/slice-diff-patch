# Features

* [`Change`] enum as an abstraction for [`diff::Result`], [`lcs_diff::DiffResult`], and
  [`wu_diff::DiffResult`];
* [`diff_changes()`], [`diff_diff()`], [`lcs_changes()`], [`lcs_diff()`], [`wu_changes()`], and
  [`wu_diff()`] functions to calculate or process diffs between `a` and `b` slices via LCS (Longest
  Common Subsequence) or Wu diff algorithms into a [`Vec<Change>`]
* [`patch()`] function to reproduce `b` from the `a` slice and [`Vec<Change>`]
* [`insert()`] and [`remove()`] functions to enable writing a custom `changes` function

# Example

```Rust
use slice_diff_patch::*;

let a = vec!["one", "TWO", "three", "four"];
let b = vec!["zero", "one", "two", "four"];

let diff = diff_diff(&a, &b);
assert_eq!(
    diff,
    vec![
        Change::Insert((0, "zero")),
        Change::Remove(2),
        Change::Update((2, "two")),
    ],
);
assert_eq!(patch(&a, &diff), b);

let lcs = lcs_diff(&a, &b);
assert_eq!(
    lcs,
    vec![
        Change::Insert((0, "zero")),
        Change::Update((2, "two")),
        Change::Remove(3),
    ],
);
assert_eq!(patch(&a, &lcs), b);

let wu = wu_diff(&a, &b);
assert_eq!(
    wu,
    vec![
        Change::Insert((0, "zero")),
        Change::Remove(2),
        Change::Update((2, "two")),
    ],
);
assert_eq!(patch(&a, &wu), b);
```

# References

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

[`Change`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/enum.Change.html
[`diff_changes()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.lcs_changes.html
[`diff::Result`]: https://docs.rs/diff/latest/diff/enum.Result.html
[`diff_diff()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.lcs_diff.html
[`insert()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.insert.html
[`lcs_changes()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.lcs_changes.html
[`lcs_diff::DiffResult`]: https://docs.rs/lcs-diff/latest/lcs_diff/enum.DiffResult.html
[`lcs_diff()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.lcs_diff.html
[`patch()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.patch.html
[`remove()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.remove.html
[`Vec<Change>`]: https://doc.rust-lang.org/1.58.1/alloc/vec/struct.Vec.html
[`wu_changes()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.wu_changes.html
[`wu_diff::DiffResult`]: https://docs.rs/wu-diff/latest/wu_diff/enum.DiffResult.html
[`wu_diff()`]: https://docs.rs/slice-diff-patch/latest/slice_diff_patch/fn.wu_diff.html

