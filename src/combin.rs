use genawaiter::sync::{Gen, GenBoxed};

/// The `comb` function calculates the number of combinations of `k` elements from a set of `n`
/// elements.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the total number of items or elements available for selection.
/// * `k`: The parameter `k` represents the number of elements chosen from a set. It is used to
/// calculate the number of combinations.
///
/// Returns:
///
/// The function `comb` returns the number of combinations of `k` elements that can be selected from a
/// set of `n` elements.
/// Number of combinations.
///
/// # Examples
///
/// ```
/// use ecgen::comb;
///  
/// assert_eq!(comb(3, 2), 3);
/// assert_eq!(comb(6, 4), comb(6, 2));
/// ```
pub const fn comb(n: usize, k: usize) -> usize {
    if k >= n || k == 0 {
        1
    } else {
        comb(n - 1, k - 1) + comb(n - 1, k)
    }
}

/// Generate all combinations by homogeneous revolving-door
///
/// The `emk_gen` function generates all combinations by using a homogeneous revolving-door algorithm.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the total number of elements in the combination set. It
/// determines the range of indices that can be used in the combinations.
/// * `k`: The parameter `k` represents the number of elements that will be swapped in each combination.
///
/// Returns:
///
/// The function `emk_gen` returns a `GenBoxed<(usize, usize)>`, which is a boxed generator that yields
/// tuples of two `usize` values.
///
/// # Examples
///
/// ```
/// use ecgen::emk_gen;
///  
/// let mut combin = [1, 1, 0];
/// println!("{:?}", combin);
/// let mut cnt = 1;
/// for (i, j) in emk_gen(3, 2) {
///     combin.swap(i, j);
///     println!("{:?}", combin);
///     cnt += 1;
/// }
///
/// assert_eq!(cnt, 3);
/// ```
pub fn emk_gen(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if n <= k || k == 0 {
            return;
        }
        if k == 1 {
            for i in 0..(n - 1) {
                co.yield_((i, i + 1)).await;
            }
        } else {
            for (i, j) in emk_gen(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n - 2, n - 1)).await;
            for (i, j) in emk_neg(n - 2, k - 1) {
                co.yield_((i, j)).await;
            }
            co.yield_((k - 2, n - 2)).await;
            for (i, j) in emk_gen(n - 2, k - 2) {
                co.yield_((i, j)).await;
            }
        }
    })
}

/// Generate all combinations by homogeneous revolving-door
///
/// The function `emk_neg` generates all combinations by using a homogeneous revolving-door algorithm.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the total number of elements in the set from which combinations
/// are generated.
/// * `k`: The parameter `k` represents the number of elements in each combination.
///
/// Returns:
///
/// The function `emk_neg` returns a generator that yields all combinations by homogeneous
/// revolving-door. The combinations are represented as tuples of two usize values.
fn emk_neg(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if n <= k || k == 0 {
            return;
        }
        if k == 1 {
            for i in (0..(n - 1)).rev() {
                co.yield_((i + 1, i)).await;
            }
        } else {
            for (i, j) in emk_neg(n - 2, k - 2) {
                co.yield_((i, j)).await;
            }
            co.yield_((n - 2, k - 2)).await;
            for (i, j) in emk_gen(n - 2, k - 1) {
                co.yield_((i, j)).await;
            }
            co.yield_((n - 1, n - 2)).await;
            for (i, j) in emk_neg(n - 1, k) {
                co.yield_((i, j)).await;
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comb_test() {
        assert_eq!(comb(3, 2), 3);
        assert_eq!(comb(6, 4), comb(6, 2));
        assert_eq!(comb(6, 6), 1);
        assert_eq!(comb(0, 0), 1);
        assert_eq!(comb(0, 1), 1);
        assert_eq!(comb(1, 1), 1);
        assert_eq!(comb(1, 0), 1);
        assert_eq!(comb(2, 2), 1);
        assert_eq!(comb(2, 1), 2);
        assert_eq!(comb(2, 0), 1);
        assert_eq!(comb(3, 3), 1);
        assert_eq!(comb(3, 2), 3);
    }
}
