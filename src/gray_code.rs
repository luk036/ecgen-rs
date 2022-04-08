use genawaiter::sync::{Gen, GenBoxed};

/**
 * @brief Binary Reflexed Gray Code Generator
 *
 * @param n
 * @return recursive_generator<int>
 */
pub fn brgc_gen(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| async move {
        if n < 1 {
            return;
        }
        for i in brgc_gen(n - 1) {
            co.yield_(i).await;
        }
        co.yield_(n - 1).await;
        for i in brgc_gen(n - 1) {
            co.yield_(i).await;
        }
    })
}
