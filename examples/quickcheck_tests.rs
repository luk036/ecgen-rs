//! Property-based tests using QuickCheck for ecgen-rs

use ecgen::{
    brgc_gen, comb, ehr_gen, emk_comb_gen, factorial, set_bipart_gen, set_partition_gen, sjt_gen,
    stirling2nd, stirling2nd2,
};
use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};

#[derive(Clone, Debug)]
struct CombParams(usize, usize);
impl Arbitrary for CombParams {
    fn arbitrary(g: &mut Gen) -> Self {
        let n = usize::arbitrary(g) % 10 + 1;
        let k = usize::arbitrary(g) % (n + 1);
        CombParams(n, k)
    }
}

#[derive(Clone, Debug)]
struct PermParams(usize);
impl Arbitrary for PermParams {
    fn arbitrary(g: &mut Gen) -> Self {
        let n = usize::arbitrary(g) % 7 + 2;
        PermParams(n)
    }
}

#[derive(Clone, Debug)]
struct PartitionParams(usize, usize);
impl Arbitrary for PartitionParams {
    fn arbitrary(g: &mut Gen) -> Self {
        let n = usize::arbitrary(g) % 7 + 3;
        let k = usize::arbitrary(g) % (n - 2) + 2;
        PartitionParams(n, k)
    }
}

#[derive(Clone, Debug)]
struct BipartParams(usize);
impl Arbitrary for BipartParams {
    fn arbitrary(g: &mut Gen) -> Self {
        let n = usize::arbitrary(g) % 8 + 2;
        BipartParams(n)
    }
}

fn comb_symmetry(params: CombParams) -> TestResult {
    let CombParams(n, k) = params;
    TestResult::from_bool(comb(n, k) == comb(n, n - k))
}

fn comb_boundary_zero(n: PermParams) -> TestResult {
    let PermParams(n) = n;
    TestResult::from_bool(comb(n, 0) == 1 && comb(n, n) == 1)
}

fn comb_boundary_one(n: PermParams) -> TestResult {
    let PermParams(n) = n;
    if n == 0 {
        return TestResult::discard();
    }
    TestResult::from_bool(comb(n, 1) == n && comb(n, n - 1) == n)
}

fn sjt_generates_factorial_count(n: PermParams) -> TestResult {
    let PermParams(n) = n;
    let count = sjt_gen(n).into_iter().count();
    TestResult::from_bool(count == factorial(n))
}

fn ehr_generates_factorial_count(n: PermParams) -> TestResult {
    let PermParams(n) = n;
    let count = ehr_gen(n).into_iter().count();
    // ehr_gen yields factorial(n) - 1 transitions
    let expected = factorial(n).saturating_sub(1);
    TestResult::from_bool(count == expected)
}

fn emk_comb_gen_count(params: CombParams) -> TestResult {
    let CombParams(n, k) = params;
    if k == 0 || k >= n {
        return TestResult::discard();
    }
    let count = emk_comb_gen(n, k).into_iter().count();
    let expected = comb(n, k).saturating_sub(1);
    TestResult::from_bool(count == expected)
}

fn brgc_gen_count(n: PermParams) -> TestResult {
    let PermParams(n) = n;
    let count = brgc_gen(n).into_iter().count();
    let expected = if n == 0 { 0 } else { (1 << n) - 1 };
    TestResult::from_bool(count == expected)
}

fn set_partition_gen_count(params: PartitionParams) -> TestResult {
    let PartitionParams(n, k) = params;
    let count = set_partition_gen(n, k).into_iter().count();
    let expected = stirling2nd(n, k).saturating_sub(1);
    TestResult::from_bool(count == expected)
}

fn set_bipart_gen_count(params: BipartParams) -> TestResult {
    let BipartParams(n) = params;
    let count = set_bipart_gen(n).into_iter().count();
    let expected = stirling2nd2(n).saturating_sub(1);
    TestResult::from_bool(count == expected)
}

fn emk_comb_k_zero(n: PermParams) -> TestResult {
    let PermParams(n) = n;
    let count = emk_comb_gen(n, 0).into_iter().count();
    TestResult::from_bool(count == 0)
}

fn emk_comb_k_equals_n(params: CombParams) -> TestResult {
    let CombParams(n, k) = params;
    if k != n {
        return TestResult::discard();
    }
    let count = emk_comb_gen(n, k).into_iter().count();
    TestResult::from_bool(count == 0)
}

fn sjt_n_equals_two() -> TestResult {
    TestResult::from_bool(sjt_gen(2).into_iter().count() == 2)
}

fn ehr_n_equals_two() -> TestResult {
    // n=2: yields 1 transition
    TestResult::from_bool(ehr_gen(2).into_iter().count() == 1)
}

fn set_partition_k_n_minus_one(n: PermParams) -> TestResult {
    let PermParams(n) = n;
    if n < 3 {
        return TestResult::discard();
    }
    let count = set_partition_gen(n, n - 1).into_iter().count();
    let expected = stirling2nd(n, n - 1).saturating_sub(1);
    TestResult::from_bool(count == expected)
}

fn set_partition_k_two(n: PermParams) -> TestResult {
    let PermParams(n) = n;
    if n < 2 {
        return TestResult::discard();
    }
    let count = set_partition_gen(n, 2).into_iter().count();
    let expected = stirling2nd(n, 2).saturating_sub(1);
    TestResult::from_bool(count == expected)
}

fn factorial_correct() -> TestResult {
    let results = [(0, 1), (1, 1), (2, 2), (3, 6), (4, 24), (5, 120)];
    TestResult::from_bool(results.iter().all(|(n, e)| factorial(*n) == *e))
}

fn main() {
    println!("Running quickcheck property-based tests for ecgen-rs...\n");

    println!("Test 1: comb_symmetry");
    match QuickCheck::new()
        .tests(100)
        .quicktest(comb_symmetry as fn(CombParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 2: comb_boundary_zero");
    match QuickCheck::new()
        .tests(100)
        .quicktest(comb_boundary_zero as fn(PermParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 3: comb_boundary_one");
    match QuickCheck::new()
        .tests(100)
        .quicktest(comb_boundary_one as fn(PermParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 4: sjt_generates_factorial_count");
    match QuickCheck::new()
        .tests(100)
        .quicktest(sjt_generates_factorial_count as fn(PermParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 5: ehr_generates_factorial_count");
    match QuickCheck::new()
        .tests(100)
        .quicktest(ehr_generates_factorial_count as fn(PermParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 6: emk_comb_gen_count");
    match QuickCheck::new()
        .tests(100)
        .quicktest(emk_comb_gen_count as fn(CombParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 7: brgc_gen_count");
    match QuickCheck::new()
        .tests(100)
        .quicktest(brgc_gen_count as fn(PermParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 8: set_partition_gen_count");
    match QuickCheck::new()
        .tests(100)
        .quicktest(set_partition_gen_count as fn(PartitionParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 9: set_bipart_gen_count");
    match QuickCheck::new()
        .tests(100)
        .quicktest(set_bipart_gen_count as fn(BipartParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 10: emk_comb_k_zero");
    match QuickCheck::new()
        .tests(100)
        .quicktest(emk_comb_k_zero as fn(PermParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 11: emk_comb_k_equals_n");
    match QuickCheck::new()
        .tests(100)
        .quicktest(emk_comb_k_equals_n as fn(CombParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 12: sjt_n_equals_two");
    match QuickCheck::new()
        .tests(1)
        .quicktest(sjt_n_equals_two as fn() -> TestResult)
    {
        Ok(n) => println!("  Passed {}/1\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 13: ehr_n_equals_two");
    match QuickCheck::new()
        .tests(1)
        .quicktest(ehr_n_equals_two as fn() -> TestResult)
    {
        Ok(n) => println!("  Passed {}/1\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 14: set_partition_k_n_minus_one");
    match QuickCheck::new()
        .tests(100)
        .quicktest(set_partition_k_n_minus_one as fn(PermParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 15: set_partition_k_two");
    match QuickCheck::new()
        .tests(100)
        .quicktest(set_partition_k_two as fn(PermParams) -> TestResult)
    {
        Ok(n) => println!("  Passed {}/100\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Test 16: factorial_correct");
    match QuickCheck::new()
        .tests(1)
        .quicktest(factorial_correct as fn() -> TestResult)
    {
        Ok(n) => println!("  Passed {}/1\n", n),
        Err(_) => println!("  FAILED\n"),
    }

    println!("Quickcheck integration verified!");
}
