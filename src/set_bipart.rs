use genawaiter::sync::{Gen, GenBoxed};

/// The `stirling2nd2` function calculates the Stirling number of the second kind specifically for k =
/// 2.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the number of elements in a set. The `stirling2nd2` function
///        calculates the Stirling number of the second kind specifically for `k = 2`, which represents the
///        number of non-empty subsets that can be formed from a set of `n` elements
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
/// * `n`: The parameter `n` represents the number of elements in the set that you want to bipart.
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
/// The function `gen0_even` generates a sequence of even numbers starting from `n` and yielding the
/// previous number, followed by the even numbers from `gen1_even(n-1)` and the negative even numbers
/// from `neg1_even(n-1)`.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the upper limit for generating even numbers. The function
///        `gen0_even` generates a sequence of even numbers starting from `n` and going down to 2.
///
/// Returns:
///
/// The function `gen0_even` returns a boxed generator (`GenBoxed<usize>`).
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

/// The function `gen1_even` generates a sequence of even numbers from 2 to n, where n is an input
/// parameter.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the upper limit of the range of numbers to generate.
///
/// Returns:
///
/// The function `gen1_even` returns a boxed generator that yields even numbers from 2 to `n`,
/// inclusive, in a specific pattern.
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
/// The function `neg1_even` generates a sequence of even numbers from 2 to n, where n is an input
/// parameter.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the upper limit of the range of numbers to generate.
///
/// Returns:
///
/// The function `neg1_even` returns a boxed generator that yields even numbers from 2 to `n`,
/// inclusive, in a specific pattern.
/// S(n,k,1) even k
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_bipart_odd() {
        const N: usize = 11;

        // 0 0 0 0 0 1
        let mut b = [0; N + 1];
        b[N] = 1; // b[0] is unused
        let mut cnt = 1;
        for x in set_bipart_gen(N) {
            let old = b[x];
            b[x] = 1 - b[x];
            println!("Move {} from Block {} to Block {}", x, old, b[x]);
            cnt += 1;
        }
        assert_eq!(cnt, stirling2nd2(N));
    }

    #[test]
    fn test_set_bipart_even() {
        const N: usize = 10;

        // 0 0 0 0 0 1
        let mut b = [0; N + 1];
        b[N] = 1; // b[0] is unused
        let mut cnt = 1;
        for x in set_bipart_gen(N) {
            let old = b[x];
            b[x] = 1 - b[x];
            println!("Move {} from Block {} to Block {}", x, old, b[x]);
            cnt += 1;
        }
        assert_eq!(cnt, stirling2nd2(N));
    }

    /// The function `test_set_bipart_special` tests the correctness of the `set_bipart_gen` function by
    /// comparing its output with the `stirling2nd2` function.
    #[test]
    fn test_set_bipart_special() {
        const N: usize = 2;

        let mut cnt = 1;
        for _x in set_bipart_gen(N) {
            cnt += 1;
        }
        assert_eq!(cnt, stirling2nd2(N));
    }
}
