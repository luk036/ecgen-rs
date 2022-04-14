/// # Set Partition
///
/// A set partition of the set [n] = {1,2,3,...,n} is a collection B0,
/// B1, ... Bj of disjoint subsets of [n] whose union is [n]. Each Bj
/// is called a block. Below we show the partitions of [4]. The periods
/// separtate the individual sets so that, for example, 1.23.4 is the
/// partition {{1},{2,3},{4}}.
///   1. block:  1234
///   2. blocks: 123.4   124.3   134.2   1.234   12.34   13.24   14.23
///   3. blocks: 1.2.34  1.24.3  14.2.3  13.2.4  12.3.4
///   4. blocks: 1.2.3.4
///
/// Each partition above has its blocks listed in increasing order of
/// smallest element; thus block 0 contains element 1, block1 contains
/// the smallest element not in block 0, and so on. A Restricted Growth
/// string (or RG string) is a sring a[1..n] where a[i] is the block in
/// whcih element i occurs. Restricted Growth strings are often called
/// restricted growth functions. Here are the RG strings corresponding
/// to the partitions shown above.
///
///   1. block:  0000
///   2. blocks: 0001, 0010, 0100, 0111, 0011, 0101, 0110
///   3. blocks: 0122, 0121, 0112, 0120, 0102,
///
/// ...more
///
/// # Reference
///
/// Frank Ruskey. Simple combinatorial Gray codes constructed by
/// reversing sublists. Lecture Notes in Computer Science, #762,
/// 201-208. Also downloadable from
/// <http://webhome.cs.uvic.ca/~ruskey/Publications/SimpleGray/SimpleGray.html>
use genawaiter::sync::{Gen, GenBoxed};

/// The lists S(n,k,0) and S(n,k,1) satisfy the following properties.
/// 1. Successive RG sequences differ in exactly one position.
/// 2. first(S(n,k,0)) = first(S(n,k,1)) = 0^{n-k}0123...(k-1)
/// 3. last(S(n,k,0)) = 0^{n-k}12...(k-1)0
/// 4. last(S(n,k,1)) = 012...(k-1)0^{n-k}
/// Note that first(S'(n,k,p)) = last(S(n,k,p))

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
/// // 0 0 0 1 2
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
/// use ecgen::set_partition_gen;
///  
/// const N: usize = 6;
/// const K: usize = 3;
///
/// // 0 0 0 0 1 2
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
/// assert_eq!(cnt, 90);
/// ```
pub fn set_partition_gen(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if k % 2 == 0 {
            for (i, j) in gen0_even(n, k) {
                co.yield_((i, j)).await;
            }
        } else {
            for (i, j) in gen0_odd(n, k) {
                co.yield_((i, j)).await;
            }
        }
    })
}

/// S(n,k,0) even k
fn gen0_even(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| {
        async move {
            if !(k > 0 && k < n) {
                return;
            }
            for (i, j) in gen0_odd(n - 1, k - 1) {
                co.yield_((i, j)).await;
            } // S(n-1, k-1, 0).(k-1)
            co.yield_((n - 1, k - 1)).await;
            for (i, j) in gen1_even(n - 1, k) {
                co.yield_((i, j)).await;
            } // S(n-1, k, 1).(k-1)
            co.yield_((n, k - 2)).await;
            for (i, j) in neg1_even(n - 1, k) {
                co.yield_((i, j)).await;
            } // S'(n-1, k, 1).(k-2)

            for i in (1..k - 2).step_by(2).rev() {
                co.yield_((n, i)).await;
                for (i, j) in gen1_even(n - 1, k) {
                    co.yield_((i, j)).await;
                } // S(n-1, k, 1).i
                co.yield_((n, i - 1)).await;
                for (i, j) in neg1_even(n - 1, k) {
                    co.yield_((i, j)).await;
                } // S'(n-1, k, 1).(i-1)
            }
        }
    })
}

/// S'(n,k,0) even k
fn neg0_even(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| {
        async move {
            if !(k > 0 && k < n) {
                return;
            }
            for i in (1..k - 2).step_by(2) {
                for (i, j) in gen1_even(n - 1, k) {
                    co.yield_((i, j)).await;
                } // S(n-1, k, 1).(i-1)
                co.yield_((n, i)).await;
                for (i, j) in neg1_even(n - 1, k) {
                    co.yield_((i, j)).await;
                } // S'(n-1, k, 1).i
                co.yield_((n, i + 1)).await;
            }

            for (i, j) in gen1_even(n - 1, k) {
                co.yield_((i, j)).await;
            } // S(n-1, k, 1).(k-2)
            co.yield_((n, k - 1)).await;
            for (i, j) in neg1_even(n - 1, k) {
                co.yield_((i, j)).await;
            } // S(n-1, k, 1).(k-1)
            co.yield_((n - 1, 0)).await;
            for (i, j) in neg0_odd(n - 1, k - 1) {
                co.yield_((i, j)).await;
            } // S(n-1, k-1, 1).(k-1)
        }
    })
}

/// S(n,k,1) even k
fn gen1_even(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if !(k > 0 && k < n) {
            return;
        }
        for (i, j) in gen1_odd(n - 1, k - 1) {
            co.yield_((i, j)).await;
        }
        co.yield_((k, k - 1)).await;
        for (i, j) in neg1_even(n - 1, k) {
            co.yield_((i, j)).await;
        }
        co.yield_((n, k - 2)).await;
        for (i, j) in gen1_even(n - 1, k) {
            co.yield_((i, j)).await;
        }

        for i in (1..k - 2).step_by(2).rev() {
            co.yield_((n, i)).await;
            for (i, j) in neg1_even(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n, i - 1)).await;
            for (i, j) in gen1_even(n - 1, k) {
                co.yield_((i, j)).await;
            }
        }
    })
}

/// S'(n,k,1) even k
fn neg1_even(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if !(k > 0 && k < n) {
            return;
        }
        for i in (1..k - 2).step_by(2) {
            for (i, j) in neg1_even(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n, i)).await;
            for (i, j) in gen1_even(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n, i + 1)).await;
        }

        for (i, j) in neg1_even(n - 1, k) {
            co.yield_((i, j)).await;
        }
        co.yield_((n, k - 1)).await;
        for (i, j) in gen1_even(n - 1, k) {
            co.yield_((i, j)).await;
        }
        co.yield_((k, 0)).await;
        for (i, j) in neg1_odd(n - 1, k - 1) {
            co.yield_((i, j)).await;
        }
    })
}

/// S(n,k,0) odd k
fn gen0_odd(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if !(k > 1 && k < n) {
            return;
        }
        for (i, j) in gen1_even(n - 1, k - 1) {
            co.yield_((i, j)).await;
        }
        co.yield_((k, k - 1)).await;
        for (i, j) in neg1_odd(n - 1, k) {
            co.yield_((i, j)).await;
        }

        for i in (1..k - 1).step_by(2).rev() {
            co.yield_((n, i)).await;
            for (i, j) in gen1_odd(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n, i - 1)).await;
            for (i, j) in neg1_odd(n - 1, k) {
                co.yield_((i, j)).await;
            }
        }
    })
}

/// S'(n,k,0) odd k
fn neg0_odd(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if !(k > 1 && k < n) {
            return;
        }
        for i in (1..k - 1).step_by(2) {
            for (i, j) in gen1_odd(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n, i)).await;
            for (i, j) in neg1_odd(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n, i + 1)).await;
        }

        for (i, j) in gen1_odd(n - 1, k) {
            co.yield_((i, j)).await;
        }
        co.yield_((k, 0)).await;
        for (i, j) in neg1_even(n - 1, k - 1) {
            co.yield_((i, j)).await;
        }
    })
}

/// S(n,k,1) odd k
fn gen1_odd(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if !(k > 1 && k < n) {
            return;
        }
        for (i, j) in gen0_even(n - 1, k - 1) {
            co.yield_((i, j)).await;
        }
        co.yield_((n - 1, k - 1)).await;
        for (i, j) in gen1_odd(n - 1, k) {
            co.yield_((i, j)).await;
        }

        for i in (1..k - 1).step_by(2).rev() {
            co.yield_((n, i)).await;
            for (i, j) in neg1_odd(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n, i - 1)).await;
            for (i, j) in gen1_odd(n - 1, k) {
                co.yield_((i, j)).await;
            }
        }
    })
}

/// S'(n,k,1) odd k
fn neg1_odd(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if !(k > 1 && k < n) {
            return;
        }
        for i in (1..k - 1).step_by(2) {
            for (i, j) in neg1_odd(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n, i)).await;
            for (i, j) in gen1_odd(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n, i + 1)).await;
        }

        for (i, j) in neg1_odd(n - 1, k) {
            co.yield_((i, j)).await;
        }
        co.yield_((n - 1, 0)).await;
        for (i, j) in neg0_even(n - 1, k - 1) {
            co.yield_((i, j)).await;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_partition_odd() {
        const N: usize = 5;
        const K: usize = 3;

        // 0 0 0 1 2
        let mut b = [0; N + 1];
        let offset = N - K + 1;
        for i in 1..K {
            b[offset + i] = i;
        }
        let mut cnt = 1;
        for (x, y) in gen0_odd(N, K) {
            let old = b[x];
            b[x] = y;
            println!("Move {} from Block {} to Block {}", x, old, y);
            cnt += 1;
        }
        assert_eq!(cnt, 25);
    }

    #[test]
    fn test_set_partition_even() {
        const N: usize = 5;
        const K: usize = 2;

        // 0 0 0 1 2
        let mut b = [0; N + 1];
        let offset = N - K + 1;
        for i in 1..K {
            b[offset + i] = i;
        }
        let mut cnt = 1;
        for (x, y) in gen0_even(N, K) {
            let old = b[x];
            b[x] = y;
            println!("Move {} from Block {} to Block {}", x, old, y);
            cnt += 1;
        }
        assert_eq!(cnt, 15);
    }
}
