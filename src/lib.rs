//! This crate provides the [`Change`] enum as an abstraction for [`diff::Result`],
//! [`lcs_diff::DiffResult`], and [`wu_diff::DiffResult`]; the [`diff_changes()`], [`diff_diff()`],
//! [`lcs_changes()`], [`lcs_diff()`], [`wu_changes()`], and [`wu_diff()`] functions to calculate or
//! process diffs between `a` and `b` slices via LCS (Longest Common Subsequence) or Wu diff
//! algorithms into a [`Vec<Change>`], and the [`patch()`] function to reproduce `b` from the `a`
//! slice and [`Vec<Change>`], and the [`insert()`] and [`remove()`] functions to enable writing a
//! custom `changes` function.
//!
//! ```
//! use slice_diff_patch::*;
//!
//! let a = vec!["one", "TWO", "three", "four"];
//! let b = vec!["zero", "one", "two", "four"];
//!
//! let diff = diff_diff(&a, &b);
//! assert_eq!(
//!     diff,
//!     vec![
//!         Change::Insert((0, "zero")),
//!         Change::Remove(2),
//!         Change::Update((2, "two")),
//!     ],
//! );
//! assert_eq!(patch(&a, &diff), b);
//!
//! let lcs = lcs_diff(&a, &b);
//! assert_eq!(
//!     lcs,
//!     vec![
//!         Change::Insert((0, "zero")),
//!         Change::Update((2, "two")),
//!         Change::Remove(3),
//!     ],
//! );
//! assert_eq!(patch(&a, &lcs), b);
//!
//! let wu = wu_diff(&a, &b);
//! assert_eq!(
//!     wu,
//!     vec![
//!         Change::Insert((0, "zero")),
//!         Change::Remove(2),
//!         Change::Update((2, "two")),
//!     ],
//! );
//! assert_eq!(patch(&a, &wu), b);
//! ```
//!
//! See also:
//!
//! * Hunt, James W; Szymanski, Thomas G. (1977). "A fast algorithm for computing longest common
//!   subsequences" <http://www.cs.ust.hk/mjg_lib/bibs/DPSu/DPSu.Files/HuSz77.pdf>
//! * Wu, Sun; Manber, Udi; Myers, Gene (1989). "An O(NP) Sequence Comparison Algorithm"
//!   <https://publications.mpi-cbg.de/Wu_1990_6334.pdf>
//! * Department of Mathematics and Computer Science. University of Southern Denmark
//!   (January 12, 2017). "The Hunt-Szymanski Algorithm for LCS"
//!   <https://imada.sdu.dk/~rolf/Edu/DM823/E16/HuntSzymanski.pdf>
//! * [diff crate](https://crates.io/crates/diff)
//! * [lcs-diff crate](https://crates.io/crates/lcs-diff)
//! * [wu-diff crate](https://crates.io/crates/wu-diff)
//! * [Wikipedia: Hunt–Szymanski algorithm](https://en.wikipedia.org/wiki/Hunt%E2%80%93Szymanski_algorithm)
//! * [Wikipedia: Bitap algorithm](https://en.wikipedia.org/wiki/Bitap_algorithm)
//! * [Practical use case analysis](https://github.com/bokuweb/wu-diff-rs/issues/7)
//!
//! [`diff::Result`]: https://docs.rs/diff/latest/diff/enum.Result.html
//! [`lcs_diff::DiffResult`]: https://docs.rs/lcs-diff/latest/lcs_diff/enum.DiffResult.html
//! [`wu_diff::DiffResult`]: https://docs.rs/wu-diff/latest/wu_diff/enum.DiffResult.html

#[cfg(test)]
mod tests {
    use super::*;

    fn display<T: PartialEq + Clone + Debug>(a: &[T], b: &[T], d: &[Change<T>]) {
        println!("a = {:?}", a);
        println!("b = {:?}", b);
        for i in d {
            println!("i = {:?}", i);
        }
    }

    fn test_states<T: PartialEq + Clone + Debug>(
        states: &[&[T]],
        diff: &dyn Fn(&[T], &[T]) -> Vec<Change<T>>,
    ) {
        for i in 0..states.len() - 1 {
            let a = &states[i];
            let b = &states[i + 1];
            let d = diff(&a, &b);
            display(&a, &b, &d);
            let c = patch(&a, &d);
            assert_eq!(&c, b);
        }
    }

    #[test]
    fn diff_int() {
        test_states(
            &[
                &[],
                &[2],
                &[2, 6],
                &[2, 4, 6],
                &[2, 4, 6, 8],
                &[1, 2, 4, 6, 8],
                &[1, 2, 3, 5, 8],
                &[1, 2, 3, 5, 8],
                &[2, 3, 5, 8],
                &[2, 5, 8],
                &[2, 5],
                &[],
            ],
            &diff_diff,
        );
    }

    #[test]
    fn diff_str() {
        test_states(
            &[
                &[],
                &["alpha"],
                &["alpha", "delta"],
                &["alpha", "bravo", "delta"],
                &["alpha", "bravo", "charlie", "delta"],
                &["pre-alpha", "alpha", "pre-bravo", "pre-charlie", "delta"],
                &["pre-alpha", "alpha", "pre-bravo", "pre-charlie"],
                &["pre-alpha", "pre-bravo", "pre-charlie"],
                &["pre-bravo", "pre-charlie"],
                &["pre-bravo"],
                &[],
            ],
            &diff_diff,
        );
    }

    #[test]
    fn lcs_int() {
        test_states(
            &[
                &[],
                &[2],
                &[2, 6],
                &[2, 4, 6],
                &[2, 4, 6, 8],
                &[1, 2, 4, 6, 8],
                &[1, 2, 3, 5, 8],
                &[1, 2, 3, 5, 8],
                &[2, 3, 5, 8],
                &[2, 5, 8],
                &[2, 5],
                &[],
            ],
            &lcs_diff,
        );
    }

    #[test]
    fn lcs_str() {
        test_states(
            &[
                &[],
                &["alpha"],
                &["alpha", "delta"],
                &["alpha", "bravo", "delta"],
                &["alpha", "bravo", "charlie", "delta"],
                &["pre-alpha", "alpha", "pre-bravo", "pre-charlie", "delta"],
                &["pre-alpha", "alpha", "pre-bravo", "pre-charlie"],
                &["pre-alpha", "pre-bravo", "pre-charlie"],
                &["pre-bravo", "pre-charlie"],
                &["pre-bravo"],
                &[],
            ],
            &lcs_diff,
        );
    }

    #[test]
    fn wu_int() {
        test_states(
            &[
                &[],
                &[2],
                &[2, 6],
                &[2, 4, 6],
                &[2, 4, 6, 8],
                &[1, 2, 4, 6, 8],
                &[1, 2, 3, 5, 8],
                &[1, 2, 3, 5, 8],
                &[2, 3, 5, 8],
                &[2, 5, 8],
                &[2, 5],
                &[],
            ],
            &wu_diff,
        );
    }

    #[test]
    fn wu_str() {
        test_states(
            &[
                &[],
                &["alpha"],
                &["alpha", "delta"],
                &["alpha", "bravo", "delta"],
                &["alpha", "bravo", "charlie", "delta"],
                &["pre-alpha", "alpha", "pre-bravo", "pre-charlie", "delta"],
                &["pre-alpha", "alpha", "pre-bravo", "pre-charlie"],
                &["pre-alpha", "pre-bravo", "pre-charlie"],
                &["pre-bravo", "pre-charlie"],
                &["pre-bravo"],
                &[],
            ],
            &wu_diff,
        );
    }

    fn update<T: PartialEq + Clone + Debug>(
        a: &[T],
        b: &[T],
        changes: Vec<Change<T>>,
        diff: &dyn Fn(&[T], &[T]) -> Vec<Change<T>>,
    ) {
        assert_eq!(diff(&a, &b), changes);
    }

    #[test]
    fn diff_update() {
        update(&[1], &[2], vec![Change::Update((0, 2))], &diff_diff);
        update(&[1, 2], &[1, 3], vec![Change::Update((1, 3))], &diff_diff);
        update(&[1, 2, 3], &[1, 2, 4], vec![Change::Update((2, 4))], &diff_diff);
        update(&["alpha"], &["bravo"], vec![Change::Update((0, "bravo"))], &diff_diff);
        update(
            &["alpha", "bravo"],
            &["alpha", "charlie"],
            vec![Change::Update((1, "charlie"))],
            &diff_diff,
        );
        update(
            &["alpha", "bravo", "charlie"],
            &["alpha", "bravo", "delta"],
            vec![Change::Update((2, "delta"))],
            &diff_diff,
        );
    }

    #[test]
    fn lcs_update() {
        update(&[1], &[2], vec![Change::Update((0, 2))], &lcs_diff);
        update(&[1, 2], &[1, 3], vec![Change::Update((1, 3))], &lcs_diff);
        update(&[1, 2, 3], &[1, 2, 4], vec![Change::Update((2, 4))], &lcs_diff);
        update(&["alpha"], &["bravo"], vec![Change::Update((0, "bravo"))], &lcs_diff);
        update(
            &["alpha", "bravo"],
            &["alpha", "charlie"],
            vec![Change::Update((1, "charlie"))],
            &lcs_diff,
        );
        update(
            &["alpha", "bravo", "charlie"],
            &["alpha", "bravo", "delta"],
            vec![Change::Update((2, "delta"))],
            &lcs_diff,
        );
    }

    #[test]
    fn wu_update() {
        update(&[1], &[2], vec![Change::Update((0, 2))], &wu_diff);
        update(&[1, 2], &[1, 3], vec![Change::Update((1, 3))], &wu_diff);
        update(&[1, 2, 3], &[1, 2, 4], vec![Change::Update((2, 4))], &wu_diff);
        update(&["alpha"], &["bravo"], vec![Change::Update((0, "bravo"))], &wu_diff);
        update(
            &["alpha", "bravo"],
            &["alpha", "charlie"],
            vec![Change::Update((1, "charlie"))],
            &wu_diff,
        );
        update(
            &["alpha", "bravo", "charlie"],
            &["alpha", "bravo", "delta"],
            vec![Change::Update((2, "delta"))],
            &wu_diff,
        );
    }
}

use std::fmt::Debug;

/// Process an insert.
///
/// * Upgrade `Change::Remove(n), Change::Insert((n, item))` to `Change::Update((n, item))`.
pub fn insert<T: PartialEq + Clone + Debug>(n: usize, item: &T, changes: &mut Vec<Change<T>>) {
    if let Some(prev_change) = changes.pop() {
        if let Change::Remove(prev_n) = prev_change {
            if n == prev_n {
                changes.push(Change::Update((n, (*item).clone())));
                return;
            }
        }
        changes.push(prev_change);
    }
    changes.push(Change::Insert((n, (*item).clone())));
}

/// Process a remove.
///
/// * Upgrade `Change::Insert((n, item)), Change::Remove(n+1)` to `Change::Update((n, item))`.
pub fn remove<T: PartialEq + Clone + Debug>(n: usize, changes: &mut Vec<Change<T>>) {
    if let Some(prev_change) = changes.pop() {
        if let Change::Insert((prev_n, ref item)) = prev_change {
            if n == prev_n + 1 {
                changes.push(Change::Update((prev_n, item.clone())));
                return;
            }
        }
        changes.push(prev_change);
    }
    changes.push(Change::Remove(n));
}

/// Abstraction for [`diff::Result`], [`lcs_diff::DiffResult`], and [`wu_diff::DiffResult`] that
/// excludes a variant for common sequence, stores a clone of inserted items, and indices relate
/// iteratively to `a`.
///
/// [`diff::Result`]: https://docs.rs/diff/latest/diff/enum.Result.html
/// [`lcs_diff::DiffResult`]: https://docs.rs/lcs-diff/latest/lcs_diff/enum.DiffResult.html
/// [`wu_diff::DiffResult`]: https://docs.rs/wu-diff/latest/wu_diff/enum.DiffResult.html
#[derive(Debug, Clone, PartialEq)]
pub enum Change<T: PartialEq + Clone + Debug> {
    Remove(usize),
    Insert((usize, T)),
    Update((usize, T)),
}

/// Convert a slice of [`diff::Result`] into a [`Vec<Change>`].
///
/// Note that unlike [`wu_changes`], `b` is not needed to clone inserted items because they are
/// included in the [`diff::Result`].
///
/// [`diff::Result`]: https://docs.rs/diff/latest/diff/enum.Result.html
pub fn diff_changes<T: PartialEq + Clone + Debug>(d: &[diff::Result<&T>]) -> Vec<Change<T>> {
    let mut changes = vec![];
    let mut removed = 0;
    for (i, j) in d.iter().enumerate() {
        let n = i - removed;
        match j {
            diff::Result::Left(_) => {
                remove(n, &mut changes);
                removed += 1;
            }
            diff::Result::Right(r) => {
                insert(n, *r, &mut changes);
            }
            _ => {}
        }
    }
    changes
}

/// Calculate the diff between `a` and `b` via [`diff::slice`] and convert to a [`Vec<Change>`].
///
/// [`diff::slice`]: https://docs.rs/diff/latest/diff/fn.diff.html
pub fn diff_diff<T: PartialEq + Clone + Debug>(a: &[T], b: &[T]) -> Vec<Change<T>> {
    diff_changes(&diff::slice(a, b))
}

/// Convert a slice of [`lcs_diff::DiffResult`] into a [`Vec<Change>`].
///
/// Note that unlike [`wu_changes`], `b` is not needed to clone inserted items because they are
/// included in the [`lcs_diff::DiffResult`].
///
/// [`lcs_diff::DiffResult`]: https://docs.rs/lcs-diff/latest/lcs_diff/enum.DiffResult.html
pub fn lcs_changes<T: PartialEq + Clone + Debug>(d: &[lcs_diff::DiffResult<T>]) -> Vec<Change<T>> {
    let mut changes = vec![];
    let mut removed = 0;
    let mut added = 0;
    for i in d {
        match i {
            lcs_diff::DiffResult::Removed(r) => {
                let n = r.old_index.unwrap() + added - removed;
                remove(n, &mut changes);
                removed += 1;
            }
            lcs_diff::DiffResult::Added(r) => {
                let n = r.new_index.unwrap();
                insert(n, &r.data, &mut changes);
                added += 1;
            }
            _ => {}
        }
    }
    changes
}

/// Calculate the diff between `a` and `b` via [`lcs_diff::diff`] and convert to a [`Vec<Change>`].
///
/// [`lcs_diff::diff`]: https://docs.rs/lcs-diff/latest/lcs_diff/fn.diff.html
pub fn lcs_diff<T: PartialEq + Clone + Debug>(a: &[T], b: &[T]) -> Vec<Change<T>> {
    lcs_changes(lcs_diff::diff(a, b).as_slice())
}

/// Convert a slice of [`wu_diff::DiffResult`] into a [`Vec<Change>`].
///
/// Note that unlike [`lcs_changes()`], `b` is needed to clone inserted items because they are not
/// included in the [`wu_diff::DiffResult`].
///
/// [`wu_diff::DiffResult`]: https://docs.rs/wu-diff/latest/wu_diff/enum.DiffResult.html
pub fn wu_changes<T: PartialEq + Clone + Debug>(
    d: &[wu_diff::DiffResult],
    b: &[T],
) -> Vec<Change<T>> {
    let mut changes = vec![];
    let mut removed = 0;
    let mut added = 0;
    for i in d {
        match i {
            wu_diff::DiffResult::Removed(r) => {
                let n = r.old_index.unwrap() + added - removed;
                remove(n, &mut changes);
                removed += 1;
            }
            wu_diff::DiffResult::Added(r) => {
                let n = r.new_index.unwrap();
                insert(n, &b[n], &mut changes);
                added += 1;
            }
            _ => {}
        }
    }
    changes
}

/// Calculate the diff between `a` and `b` via [`wu_diff::diff`] and convert to a [`Vec<Change>`].
///
/// [`wu_diff::diff`]: https://docs.rs/wu-diff/latest/wu_diff/fn.diff.html
pub fn wu_diff<T: PartialEq + Clone + Debug>(a: &[T], b: &[T]) -> Vec<Change<T>> {
    wu_changes(&wu_diff::diff(a, b), b)
}

/// Reproduce `b` from the `a` slice and [`Vec<Change>`].
pub fn patch<T: PartialEq + Clone + Debug>(a: &[T], changes: &[Change<T>]) -> Vec<T> {
    let mut a = a.to_vec();
    for i in changes {
        match i {
            Change::Remove(n) => {
                a.remove(*n);
            }
            Change::Insert((n, item)) => {
                a.insert(*n, item.clone());
            }
            Change::Update((n, item)) => {
                a.remove(*n);
                a.insert(*n, item.clone());
            }
        }
    }
    a
}
