use genawaiter::sync::{Gen, GenBoxed};

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

pub fn emk_neg(n: usize, k: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        if n <= k || k == 0 {
            return;
        }
        if k == 1 {
            for i in (0..n - 1).rev() {
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
    use crate::combin::comb;

    #[test]
    fn test_emk_gen_old() {
        let mut cnt = 1;
        for (_x, _y) in emk_gen(16, 5) {
            cnt += 1;
        }
        assert_eq!(cnt, comb(16, 5));
    }
}
