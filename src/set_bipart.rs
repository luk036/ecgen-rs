use genawaiter::sync::{Gen, GenBoxed};

/// Stirling number of second kind (k = 2).
///
/// # Examples
///
/// ```
/// use ecgen::stirling2nd2;
///  
/// assert_eq!(stirling2nd2(5), 15);
/// ```
pub const fn stirling2nd2(n: usize) -> usize {
    if n <= 2 {
        1
    } else {
        1 + 2 * stirling2nd2(n - 1)
    }
}

/// Set partition gen object
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
