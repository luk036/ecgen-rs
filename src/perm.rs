use genawaiter::sync::{Gen, GenBoxed};

/**
 * @brief Generate all permutations by adjacent transposition
 *
 * @param n
 */
pub fn sjt_gen(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| {
        async move {
            /* Generate the swaps for the Steinhaus-Johnson-Trotter algorithm.*/
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

/**
 * @brief Generate all permutations by star transposition
 *
 * @param n
 * @return cppcoro::generator<int>
 */
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

                // for (auto i = 1, j = k - 1; i < j; ++i, --j) {
                //    std::swap(b[i], b[j]);
                // }
                let mut i = 1;
                let mut j = k - 1;
                while i < j {
                    b.swap(i, j);
                    i += 1;
                    j -= 1;
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::{ehr_gen, sjt_gen};

    #[test]
    fn test_sjt() {
        let mut perm = ["🧧", "🥭", "🐍", "🦠"];
        let mut cnt = 0;
        for n in sjt_gen(perm.len()) {
            println!("{}", perm.concat());
            cnt += 1;
            perm.swap(n, n + 1);
        }
        assert_eq!(cnt, 24);
    }

    #[test]
    fn test_ehr() {
        let mut perm = ["🧧", "🥭", "🐍", "🦠"];
        let mut cnt = 1;
        println!("{}", perm.concat());
        for n in ehr_gen(perm.len()) {
            perm.swap(0, n);
            println!("{}", perm.concat());
            cnt += 1;
        }
        assert_eq!(cnt, 24);
    }
}
