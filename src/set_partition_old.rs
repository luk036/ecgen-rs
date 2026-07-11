use genawaiter::sync::{Gen, GenBoxed};

pub fn set_partition_gen_old(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
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

fn gen0_even(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if k > 0 && k < n {
            for (i, j) in gen0_odd(n - 1, k - 1) {
                co.yield_((i, j)).await;
            }
            co.yield_((n - 1, k - 1)).await;
            for (i, j) in gen1_even(n - 1, k) {
                co.yield_((i, j)).await;
            }
            co.yield_((n, k - 2)).await;
            for (i, j) in neg1_even(n - 1, k) {
                co.yield_((i, j)).await;
            }
            let mut i: isize = (k as isize) - 2;
            while i > 1 {
                let ui = i as usize;
                co.yield_((n, ui - 1)).await;
                for (i2, j) in gen1_even(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui - 2)).await;
                for (i2, j) in neg1_even(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                i -= 2;
            }
        }
    })
}

fn neg0_even(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if k > 0 && k < n {
            let mut i = 1;
            while i < k - 2 {
                let ui = i;
                for (i2, j) in gen1_even(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui)).await;
                for (i2, j) in neg1_even(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui + 1)).await;
                i += 2;
            }
            for (i2, j) in gen1_even(n - 1, k) {
                co.yield_((i2, j)).await;
            }
            co.yield_((n, k - 1)).await;
            for (i2, j) in neg1_even(n - 1, k) {
                co.yield_((i2, j)).await;
            }
            co.yield_((n - 1, 0)).await;
            for (i2, j) in neg0_odd(n - 1, k - 1) {
                co.yield_((i2, j)).await;
            }
        }
    })
}

fn gen1_even(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if k > 0 && k < n {
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
            let mut i: isize = (k as isize) - 2;
            while i > 1 {
                let ui = i as usize;
                co.yield_((n, ui - 1)).await;
                for (i2, j) in neg1_even(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui - 2)).await;
                for (i2, j) in gen1_even(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                i -= 2;
            }
        }
    })
}

fn neg1_even(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if k > 0 && k < n {
            let mut i = 1;
            while i < k - 2 {
                let ui = i;
                for (i2, j) in neg1_even(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui)).await;
                for (i2, j) in gen1_even(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui + 1)).await;
                i += 2;
            }
            for (i2, j) in neg1_even(n - 1, k) {
                co.yield_((i2, j)).await;
            }
            co.yield_((n, k - 1)).await;
            for (i2, j) in gen1_even(n - 1, k) {
                co.yield_((i2, j)).await;
            }
            co.yield_((k, 0)).await;
            for (i2, j) in neg1_odd(n - 1, k - 1) {
                co.yield_((i2, j)).await;
            }
        }
    })
}

fn gen0_odd(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if k > 1 && k < n {
            for (i, j) in gen1_even(n - 1, k - 1) {
                co.yield_((i, j)).await;
            }
            co.yield_((k, k - 1)).await;
            for (i, j) in neg1_odd(n - 1, k) {
                co.yield_((i, j)).await;
            }
            let mut i: isize = (k as isize) - 1;
            while i > 1 {
                let ui = i as usize;
                co.yield_((n, ui - 1)).await;
                for (i2, j) in gen1_odd(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui - 2)).await;
                for (i2, j) in neg1_odd(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                i -= 2;
            }
        }
    })
}

fn neg0_odd(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if k > 1 && k < n {
            let mut i = 1;
            while i < k - 1 {
                let ui = i;
                for (i2, j) in gen1_odd(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui)).await;
                for (i2, j) in neg1_odd(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui + 1)).await;
                i += 2;
            }
            for (i2, j) in gen1_odd(n - 1, k) {
                co.yield_((i2, j)).await;
            }
            co.yield_((k, 0)).await;
            for (i2, j) in neg1_even(n - 1, k - 1) {
                co.yield_((i2, j)).await;
            }
        }
    })
}

fn gen1_odd(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if k > 1 && k < n {
            for (i, j) in gen0_even(n - 1, k - 1) {
                co.yield_((i, j)).await;
            }
            co.yield_((n - 1, k - 1)).await;
            for (i, j) in gen1_odd(n - 1, k) {
                co.yield_((i, j)).await;
            }
            let mut i: isize = (k as isize) - 1;
            while i > 1 {
                let ui = i as usize;
                co.yield_((n, ui - 1)).await;
                for (i2, j) in neg1_odd(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui - 2)).await;
                for (i2, j) in gen1_odd(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                i -= 2;
            }
        }
    })
}

fn neg1_odd(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if k > 1 && k < n {
            let mut i = 1;
            while i < k - 1 {
                let ui = i;
                for (i2, j) in neg1_odd(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui)).await;
                for (i2, j) in gen1_odd(n - 1, k) {
                    co.yield_((i2, j)).await;
                }
                co.yield_((n, ui + 1)).await;
                i += 2;
            }
            for (i2, j) in neg1_odd(n - 1, k) {
                co.yield_((i2, j)).await;
            }
            co.yield_((n - 1, 0)).await;
            for (i2, j) in neg0_even(n - 1, k - 1) {
                co.yield_((i2, j)).await;
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set_partition::stirling2nd;

    #[test]
    fn test_set_partition_old_odd_odd() {
        const N: usize = 11;
        const K: usize = 5;
        let mut cnt = 1;
        for (_x, _y) in set_partition_gen_old(N, K) {
            cnt += 1;
        }
        assert_eq!(cnt, stirling2nd(N, K));
    }

    #[test]
    fn test_set_partition_old_even_odd() {
        const N: usize = 10;
        const K: usize = 5;
        let mut cnt = 1;
        for (_x, _y) in set_partition_gen_old(N, K) {
            cnt += 1;
        }
        assert_eq!(cnt, stirling2nd(N, K));
    }
}
