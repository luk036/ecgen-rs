mod gray_code;
mod perm;
mod combin;
mod set_partition;
mod set_bipart;

#[cfg(test)]
mod test {
    use crate::gray_code::brgc_gen;
    use crate::perm::ehr_gen;
    use crate::perm::sjt_gen;
    use crate::combin::emk_gen;
    use crate::set_partition::set_partition_gen;
    use crate::set_bipart::set_bipart_gen;

    #[test]
    fn test_emk() {
        let mut combin = [1, 1, 0];
        println!("{:?}", combin);
        let mut cnt = 1;
        for (i, j) in emk_gen(3, 2) {
            combin.swap(i, j);
            println!("{:?}", combin);
            cnt += 1;
        }
        assert_eq!(cnt, 3);
    }

    #[test]
    fn test_gray_code() {
        let mut lst = ["⬜"; 3];
        println!("{}", lst.concat());
        let mut cnt = 1;
        for n in brgc_gen(lst.len()) {
            lst[n] = if lst[n] == "⬜" { "⬛" } else { "⬜" };
            println!("{}", lst.concat());
            cnt += 1;
        }
        assert_eq!(cnt, 8);
    }

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
        for (x, y) in set_partition_gen(N, K) {
            let old = b[x];
            b[x] = y;
            println!("Move {} from Block {} to Block {}", x, old, y);
            cnt += 1;
        }
        assert_eq!(cnt, 25);
    }

    #[test]
    fn test_set_partition_evn() {
        const N: usize = 6;
        const K: usize = 3;

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
        assert_eq!(cnt, 90);
    }

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
