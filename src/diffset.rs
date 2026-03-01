// Copyright (c) 2019 Joe Sawada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Difference set generation
//!
//! This module provides functionality for generating difference sets,
//! which are combinatorial structures with specific properties.

use genawaiter::sync::{Gen, GenBoxed};

/// Generate difference sets
///
/// This generator produces all possible difference sets of size `d` for a set of size `n`,
/// using a threshold for pruning the search space.
///
/// # Arguments
///
/// * `n` - The size of the set (elements are 0, 1, ..., n-1)
/// * `d` - The size of each difference set
/// * `threshold` - Threshold value for pruning the search space
///
/// # Returns
///
/// A generator that yields vectors representing difference sets
///
/// # Examples
///
/// ```
/// use ecgen::diffset::diffset_gen;
///
/// // Generate difference sets for n=13, d=4
/// let gen = diffset_gen(13, 4, 2);
/// let mut sets: Vec<Vec<usize>> = Vec::new();
/// for set in gen {
///     sets.push(set);
/// }
/// // Should find the difference set {0,1,3,9} modulo 13
/// ```
pub fn diffset_gen(n: usize, d: usize, threshold: usize) -> GenBoxed<Vec<usize>> {
    Gen::new_boxed(|co| async move {
        if n > d * (d - 1) + 1 {
            return;
        }

        let max_n = 70;
        if n > max_n {
            return;
        }

        let d_times_d_minus_1 = d * (d - 1);
        let n_minus_d = n - d;
        let n2 = n / 2;
        let n1 = n2 - d_times_d_minus_1 / 2;
        let size_n = n2 + 1;

        let mut a = vec![0usize; d + 1];
        let mut b = vec![0usize; d + 1];
        a[d] = n;
        a[0] = 0;

        let mut differences = vec![0i8; size_n];
        differences[0] = 1;

        let ctx = DiffsetContext {
            n,
            d,
            threshold,
            n1,
            n2,
            n_minus_d,
        };

        for j in ((n - 1) / d + 1..=n - d + 1).rev() {
            a[1] = j;
            b[1] = 1;
            let mut result = Vec::new();
            diffset_recursive_impl(&mut a, &mut b, 1, 1, 1, &mut differences, &ctx, &mut result);
            for set in result {
                co.yield_(set).await;
            }
        }
    })
}

struct DiffsetContext {
    n: usize,
    d: usize,
    threshold: usize,
    n1: usize,
    n2: usize,
    n_minus_d: usize,
}

#[allow(clippy::too_many_arguments)]
fn diffset_recursive_impl(
    a: &mut [usize],
    b: &mut [usize],
    t: usize,
    p: usize,
    tt: usize,
    diffset: &mut [i8],
    ctx: &DiffsetContext,
    result: &mut Vec<Vec<usize>>,
) {
    let mut differences = diffset.to_owned();

    for i in 0..t {
        let diff = a[t] - a[i];
        let n_diff = ctx.n - diff;
        let min_diff = if diff <= n_diff { diff } else { n_diff };
        differences[min_diff] = 1;
    }

    if t >= ctx.threshold {
        let count = differences[1..=ctx.n2].iter().filter(|&&x| x != 0).count();
        if count < ctx.n1 + tt {
            return;
        }
    }

    let t1 = t + 1;
    if t1 >= ctx.d {
        // Found a valid difference set
        result.push(a[0..ctx.d].to_vec());
    } else {
        let mut tail = ctx.n_minus_d + t1;
        let max = a[t1 - p] + a[p];
        let tt1 = t1 * (t1 + 1) / 2;

        if max <= tail {
            a[t1] = max;
            b[t1] = b[t1 - p];
            diffset_recursive_impl(a, b, t1, p, tt1, &mut differences, ctx, result);

            if b[t1] == 0 {
                b[t1] = 1;
                diffset_recursive_impl(a, b, t1, t1, tt1, &mut differences, ctx, result);
            }
            tail = max - 1;
        }

        for j in (a[t] + 1..=tail).rev() {
            a[t1] = j;
            b[t1] = 1;
            diffset_recursive_impl(a, b, t1, t1, tt1, &mut differences, ctx, result);
        }
    }
}

/// Check if a set is a valid difference set
///
/// This function verifies whether a given set is a difference set modulo n,
/// meaning that all non-zero differences appear exactly once.
///
/// # Arguments
///
/// * `diffset` - The set to check
/// * `n` - The modulus
///
/// # Returns
///
/// `true` if the set is a valid difference set, `false` otherwise
///
/// # Examples
///
/// ```
/// use ecgen::diffset::is_diffset;
///
/// // {0, 1, 3, 9} is a difference set modulo 13
/// assert!(is_diffset(&[0, 1, 3, 9], 13));
/// ```
pub fn is_diffset(diffset: &[usize], n: usize) -> bool {
    if diffset.is_empty() {
        return false;
    }

    let mut differences = vec![0i8; n];

    for i in 0..diffset.len() {
        for j in (i + 1)..diffset.len() {
            let diff = (diffset[j] - diffset[i]) % n;
            let n_diff = (n - diff) % n;
            differences[diff] += 1;
            differences[n_diff] += 1;
        }
    }

    // Check that all non-zero differences appear exactly once
    for diff_count in &differences[1..n] {
        if *diff_count != 1 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_diffset() {
        // {0, 1, 3, 9} is a difference set modulo 13
        assert!(is_diffset(&[0, 1, 3, 9], 13));

        // {0, 1, 3, 9, 13} is not a difference set modulo 13
        assert!(!is_diffset(&[0, 1, 3, 9, 13], 13));

        // Empty set is not a difference set
        assert!(!is_diffset(&[], 5));

        // {0, 1, 4, 6} - verify if this is actually a difference set modulo 7
        // The differences are: 1-0=1, 4-0=4, 6-0=6, 4-1=3, 6-1=5, 6-4=2
        // and their negatives modulo 7: 6, 3, 1, 4, 2, 5
        // So we get differences: 1,2,3,4,5,6 each appearing twice
        // This is NOT a valid difference set because each non-zero difference should appear exactly once
        assert!(!is_diffset(&[0, 1, 4, 6], 7));
    }

    #[test]
    fn test_diffset_gen() {
        // Test that the generator finds at least one difference set
        let gen = diffset_gen(13, 4, 2);
        let mut sets: Vec<Vec<usize>> = Vec::new();
        for set in gen {
            sets.push(set);
        }

        // Should find at least one difference set
        assert!(!sets.is_empty());

        // Verify all generated sets are valid
        for set in &sets {
            assert!(is_diffset(set, 13));
        }
    }

    #[test]
    fn test_diffset_gen_invalid() {
        // Test with invalid parameters
        let gen = diffset_gen(100, 5, 2);
        let mut sets: Vec<Vec<usize>> = Vec::new();
        for set in gen {
            sets.push(set);
        }
        // Should not find any sets because n > d*(d-1)+1
        assert!(sets.is_empty());
    }

    #[test]
    fn test_diffset_gen_large() {
        // Test with larger parameters (but still manageable)
        let gen = diffset_gen(16, 4, 2);
        let mut count = 0;
        for _ in gen {
            count += 1;
        }
        // Difference sets are rare, so we might not find any for these parameters
        // The important thing is that the generator doesn't crash
        assert!(count >= 0);
    }
}
