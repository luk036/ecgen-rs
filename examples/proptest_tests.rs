//! Property-based tests using proptest for ecgen-rs
//!
//! Run with: `cargo run --example proptest_tests`

// Silence dead_code warnings for the four strategy functions below
// (comb_params, perm_params, partition_params, bipar_params). They ARE
// referenced inside the `proptest!` macro (e.g. `fn comb_symmetry((n, k)
// in comb_params())`), but the macro's procedural expansion generates code
// that rustc's dead_code analysis cannot trace back to these definitions,
// so the lint fires a false positive.  Once proptest emits explicit calls
// (or rustc improves cross-macro reachability) this allow can be removed.
#![allow(dead_code)]

use proptest::prelude::*;

fn comb_params() -> impl Strategy<Value = (usize, usize)> {
    (1..11usize).prop_flat_map(|n| (Just(n), 0..=n))
}

fn perm_params() -> impl Strategy<Value = usize> {
    2..9usize
}

fn partition_params() -> impl Strategy<Value = (usize, usize)> {
    (3..10usize).prop_flat_map(|n| (Just(n), 2..=(n - 1)))
}

fn bipar_params() -> impl Strategy<Value = usize> {
    2..10usize
}

proptest! {
    #[test]
    fn comb_symmetry((n, k) in comb_params()) {
        assert_eq!(comb(n, k), comb(n, n - k));
    }

    #[test]
    fn comb_boundary_zero(n in perm_params()) {
        assert!(comb(n, 0) == 1 && comb(n, n) == 1);
    }

    #[test]
    fn comb_boundary_one(n in 1..9usize) {
        assert!(comb(n, 1) == n && comb(n, n - 1) == n);
    }

    #[test]
    fn sjt_generates_factorial_count(n in perm_params()) {
        let count = sjt_gen(n).into_iter().count();
        assert_eq!(count, factorial(n));
    }

    #[test]
    fn ehr_generates_factorial_count(n in perm_params()) {
        let count = ehr_gen(n).into_iter().count();
        // ehr_gen yields factorial(n) - 1 transitions
        let expected = factorial(n).saturating_sub(1);
        assert_eq!(count, expected);
    }

    #[test]
    fn emk_comb_gen_count((n, k) in comb_params()) {
        prop_assume!(k != 0 && k < n);
        let count = emk_comb_gen(n, k).into_iter().count();
        let expected = comb(n, k).saturating_sub(1);
        assert_eq!(count, expected);
    }

    #[test]
    fn brgc_gen_count(n in perm_params()) {
        let count = brgc_gen(n).into_iter().count();
        let expected = if n == 0 { 0 } else { (1 << n) - 1 };
        assert_eq!(count, expected);
    }

    #[test]
    fn set_partition_gen_count((n, k) in partition_params()) {
        let count = set_partition_gen(n, k).into_iter().count();
        let expected = stirling2nd(n, k).saturating_sub(1);
        assert_eq!(count, expected);
    }

    #[test]
    fn set_bipart_gen_count(n in bipar_params()) {
        let count = set_bipart_gen(n).into_iter().count();
        let expected = stirling2nd2(n).saturating_sub(1);
        assert_eq!(count, expected);
    }

    #[test]
    fn emk_comb_k_zero(n in perm_params()) {
        let count = emk_comb_gen(n, 0).into_iter().count();
        assert_eq!(count, 0);
    }

    #[test]
    fn emk_comb_k_equals_n((n, k) in comb_params()) {
        prop_assume!(k == n);
        let count = emk_comb_gen(n, k).into_iter().count();
        assert_eq!(count, 0);
    }
}

fn main() {}
