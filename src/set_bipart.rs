/**
 Set Partition

 A set partition of the set [n] = {1,2,3,...,n} is a collection B0,
 B1, ... Bj of disjoint subsets of [n] whose union is [n]. Each Bj
 is called a block. Below we show the partitions of [4]. The periods
 separtate the individual sets so that, for example, 1.23.4 is the
 partition {{1},{2,3},{4}}.
   1 block:  1234
   2 blocks: 123.4   124.3   134.2   1.234   12.34   13.24   14.23
   3 blocks: 1.2.34  1.24.3  14.2.3  13.2.4  12.3.4
   4 blocks: 1.2.3.4

 Each partition above has its blocks listed in increasing order of
 smallest element; thus block 0 contains element 1, block1 contains
 the smallest element not in block 0, and so on. A Restricted Growth
 string (or RG string) is a sring a[1..n] where a[i] is the block in
 whcih element i occurs. Restricted Growth strings are often called
 restricted growth functions. Here are the RG strings corresponding
 to the partitions shown above.

   1 block:  0000
   2 blocks: 0001, 0010, 0100, 0111, 0011, 0101, 0110
   3 blocks: 0122, 0121, 0112, 0120, 0102,

 ...more

 Reference:
 Frank Ruskey. Simple combinatorial Gray codes constructed by
 reversing sublists. Lecture Notes in Computer Science, #762,
 201-208. Also downloadable from
 http://webhome.cs.uvic.ca/~ruskey/Publications/SimpleGray/SimpleGray.html
*/
use genawaiter::sync::{Gen, GenBoxed};

// The lists S(n,k,0) and S(n,k,1) satisfy the following properties.
// 1. Successive RG sequences differ in exactly one position.
// 2. first(S(n,k,0)) = first(S(n,k,1)) = 0^{n-k}0123...(k-1)
// 3. last(S(n,k,0)) = 0^{n-k}12...(k-1)0
// 4. last(S(n,k,1)) = 012...(k-1)0^{n-k}
// Note that first(S'(n,k,p)) = last(S(n,k,p))

/**
 * @brief Set the partition gen object
 *
 * @param n
 * @param k
 * @return GenBoxed<usize>
 */
pub fn set_bipart_gen(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| async move {
        for i in gen0_even(n) {
            co.yield_(i).await;
        }
    })
}

/**
 * @brief S(n,k,0) even k
 *
 * @param n
 * @param k
 * @return GenBoxed<usize>
 */
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

/**
 * @brief S(n, k, 1) even k
 *
 * @param n
 * @param k
 * @return GenBoxed<usize>
 */
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

/**
 * @brief S'(n, k, 1) even k
 *
 * @param n
 * @param k
 * @return GenBoxed<usize>
 */
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
mod test {
    use super::set_bipart_gen;

    #[test]
    fn test_set_bipart() {
        const N: usize = 5;

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
        assert_eq!(cnt, 15);
    }
}
