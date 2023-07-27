use genawaiter::sync::{Gen, GenBoxed};

/// The `stirling2nd2` function calculates the Stirling number of the second kind specifically for k =
/// 2.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the number of elements in a set. The `stirling2nd2` function
/// calculates the Stirling number of the second kind specifically for `k = 2`, which represents the
/// number of non-empty subsets that can be formed from a set of `n` elements
///
/// Returns:
///
/// The `stirling2nd2` function returns the Stirling number of the second kind specifically for k = 2.
///
/// # Examples
///
/// ```
/// use ecgen::stirling2nd2;
///  
/// assert_eq!(stirling2nd2(5), 15);
/// ```
#[inline]
pub const fn stirling2nd2(n: usize) -> usize {
    if n <= 2 {
        1
    } else {
        1 + 2 * stirling2nd2(n - 1)
    }
}

/// The `set_bipart_gen` function generates a sequence of numbers representing moves between two blocks.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the number of elements in the set that you want to partition.
///
/// Returns:
///
/// The function `set_bipart_gen` returns a boxed generator of type `GenBoxed<usize>`.
///
/// # Examples
///
/// ```
/// use ecgen::set_bipart_gen;
///  
/// const N: usize = 5;
///
/// // 0 0 0 0 0 1
/// let mut b = [0; N + 1];
/// b[N] = 1; // b[0] is unused
/// let mut cnt = 1;
/// for x in set_bipart_gen(N) {
///     let old = b[x];
///     b[x] = 1 - b[x];
///     println!("Move {} from Block {} to Block {}", x, old, b[x]);
///     cnt += 1;
/// }
///
/// assert_eq!(cnt, 15);
/// ```
pub fn set_bipart_gen(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| async move {
        for i in gen0_even(n) {
            co.yield_(i).await;
        }
    })
}

/// S(n,k,0) even k
#[inline]
fn gen0_even(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| async move {
        if n < 3 {
            return;
        }
        co.yield_(n - 1).await;
        for i in gen1_even(n - 1) {
            co.yield_(i).await;
        } // S(n-1, k, 1).(k-1)
        co.yield_(n).await;
        for i in neg1_even(n - 1) {
            co.yield_(i).await;
        } // S'(n-1, k, 1).(k-2)
    })
}

/// S(n,k,1) even k
#[inline]
fn gen1_even(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| async move {
        if n < 3 {
            return;
        }
        co.yield_(2).await;
        for i in neg1_even(n - 1) {
            co.yield_(i).await;
        }
        co.yield_(n).await;
        for i in gen1_even(n - 1) {
            co.yield_(i).await;
        }
    })
}

/// S'(n,k,1) even k
#[inline]
fn neg1_even(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| async move {
        if n < 3 {
            return;
        }
        for i in neg1_even(n - 1) {
            co.yield_(i).await;
        }
        co.yield_(n).await;
        for i in gen1_even(n - 1) {
            co.yield_(i).await;
        }
        co.yield_(2).await;
    })
}
