use genawaiter::sync::{Gen, GenBoxed};

/// Set partition gen object
///
/// # Examples
///
/// ```
/// use ecgen::set_partition_gen;
///  
/// const N: usize = 5;
/// const K: usize = 3;
///
/// // 0 0 0 1
/// let mut b = [0; N + 1];
/// let offset = N - K + 1;
/// for i in 1..K {
///     b[offset + i] = i;
/// }
/// let mut cnt = 1;
/// for (x, y) in set_partition_gen(N, K) {
///     let old = b[x];
///     b[x] = y;
///     println!("Move {} from Block {} to Block {}", x, old, y);
///     cnt += 1;
/// }
///
/// assert_eq!(cnt, 25);
/// ```
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
