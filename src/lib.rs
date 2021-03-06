pub mod combin;
pub mod gray_code;
pub mod perm;
pub mod set_bipart;
pub mod set_partition;

pub use crate::combin::emk_gen;
pub use crate::gray_code::brgc_gen;
pub use crate::perm::{ehr_gen, sjt_gen};
pub use crate::set_bipart::set_bipart_gen;
pub use crate::set_partition::set_partition_gen;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_brgc() {
        let mut cnt = 1;
        for _n in brgc_gen(3) {
            cnt += 1;
        }
        assert_eq!(cnt, 8);
    }

    #[test]
    fn test_sjt() {
        let mut cnt = 0;
        for _n in sjt_gen(4) {
            cnt += 1;
        }
        assert_eq!(cnt, 24);
    }

    #[test]
    fn test_ehr() {
        let mut cnt = 1;
        for _n in ehr_gen(4) {
            cnt += 1;
        }
        assert_eq!(cnt, 24);
    }

    #[test]
    fn test_emk() {
        let mut cnt = 1;
        for (_x, _y) in emk_gen(6, 3) {
            cnt += 1;
        }
        assert_eq!(cnt, 20);
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
        for (x, y) in set_partition_gen(N, K) {
            let old = b[x];
            b[x] = y;
            println!("Move {} from Block {} to Block {}", x, old, y);
            cnt += 1;
        }
        assert_eq!(cnt, 15);
    }

    #[test]
    fn test_set_bipart_even() {
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
