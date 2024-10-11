use genawaiter::sync::{Gen, GenBoxed};

// /// For future rust version:
// const fn factorial2<const N: usize>() -> usize {
//     if N < 2 {
//         1
//     } else {
//         const M: usize = N - 1;
//         N * factorial2::<M>()
//     }
// }

/// The `factorial` function calculates the factorial of a given number.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the number for which we want to calculate the factorial.
///
/// Returns:
///
/// The `factorial` function returns the factorial of the input number `n`.
///
/// # Examples
///
/// ```
/// use ecgen::factorial;
///  
/// assert_eq!(factorial(5), 120);
/// assert_eq!(factorial(1), 1);
/// ```
pub const fn factorial(n: usize) -> usize {
    if n < 2 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// Generate all permutations by adjacent transposition
///
/// The `sjt_gen` function in Rust generates all permutations of a given length using the
/// Steinhaus-Johnson-Trotter algorithm.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the number of elements in the permutation.
///
/// Returns:
///
/// The function `sjt_gen` returns a boxed generator that yields permutations of indices.
///
/// # Examples
///
/// ```
/// use ecgen::sjt_gen;
///  
/// let mut perm = ["🍉", "🍌", "🍇", "🍏"];
/// let mut cnt = 0;
/// for n in sjt_gen(perm.len()) {
///     println!("{}", perm.concat());
///     cnt += 1;
///     perm.swap(n, n + 1);
/// }
///
/// assert_eq!(cnt, 24);
/// assert_eq!(perm, ["🍉", "🍌", "🍇", "🍏"]); // Hamilton cycle
/// ```
pub fn sjt_gen(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| {
        async move {
            /* Generate the swaps for the Steinhaus-Johnson-Trotter algorithm. */
            if n == 2 {
                co.yield_(0).await;
                co.yield_(0).await; // tricky part: return to the original list
                return;
            }
            let mut even = true;
            for j in sjt_gen(n - 1) {
                if even {
                    // downward
                    for i in (0..n - 1).rev() {
                        co.yield_(i).await;
                    }
                    co.yield_(1 + j).await;
                    even = false;
                } else {
                    // upward
                    for i in 0..n - 1 {
                        co.yield_(i).await;
                    }
                    co.yield_(j).await;
                    even = true;
                }
            }
        }
    })
}

/// Generate all permutations by star transposition
///
/// The `ehr_gen` function generates all permutations of a given length using the star transposition
/// algorithm.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the number of elements in the permutation. In the given example,
///        `n` is set to 4, so it generates permutations of 4 elements.
///
/// Returns:
///
/// The function `ehr_gen` returns a `GenBoxed<usize>`, which is a boxed generator that yields `usize`
/// values.
///
/// # Examples
///
/// ```
/// use ecgen::ehr_gen;
///  
/// let mut perm = ["🍉", "🍌", "🍇", "🍏"];
/// let mut cnt = 1;
/// println!("{}", perm.concat());
/// for n in ehr_gen(perm.len()) {
///     perm.swap(0, n);
///     println!("{}", perm.concat());
///     cnt += 1;
/// }
///
/// assert_eq!(cnt, 24);
/// assert_eq!(perm, ["🍏", "🍌", "🍇", "🍉"]);
/// ```
pub fn ehr_gen(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| {
        async move {
            let mut c = vec![0; n + 1]; // c[0] is never used
            let mut b: Vec<usize> = (0..n).collect(); // 0, ... n-1
            loop {
                let mut k: usize = 1;
                loop {
                    if c[k] == k {
                        c[k] = 0;
                        k += 1;
                    }
                    if c[k] < k {
                        break;
                    }
                }
                if k == n {
                    break;
                }
                c[k] += 1;
                co.yield_(b[k]).await;
                b[1..k].reverse();
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sjt() {
        let mut cnt = 0;
        for _n in sjt_gen(4) {
            cnt += 1;
        }
        assert_eq!(cnt, factorial(4));
    }

    #[test]
    fn test_ehr() {
        let mut cnt = 1;
        for _n in ehr_gen(4) {
            cnt += 1;
        }
        assert_eq!(cnt, factorial(4));
    }
}
