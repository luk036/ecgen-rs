use std::time::Instant;

use ecgen::{
    brgc_gen, comb, ehr_gen, emk_comb_gen, factorial, set_bipart_gen, set_partition_gen, sjt_gen,
    stirling2nd, stirling2nd2,
};

macro_rules! bench {
    ($name:expr, $body:expr) => {{
        let start = Instant::now();
        let result = $body;
        let dur = start.elapsed();
        println!(
            "{:<30} {:>10}  {:>8.3?}  {:>10}",
            $name,
            result,
            dur,
            stringify!($body)
        );
    }};
}

fn main() {
    println!(
        "{:<30} {:>10}  {:>10}  {}",
        "Operation", "Count", "Time", ""
    );
    println!("{:-<70}", "");

    // EMK combinations
    bench!("emk_comb_gen(16,5)", {
        let mut cnt = 1;
        for _ in emk_comb_gen(16, 5) {
            cnt += 1;
        }
        cnt
    });
    bench!("emk_comb_gen(10,5)", {
        let mut cnt = 1;
        for _ in emk_comb_gen(10, 5) {
            cnt += 1;
        }
        cnt
    });

    // SJT permutations
    bench!("sjt_gen(8)", {
        let mut cnt = 0;
        for _ in sjt_gen(8) {
            cnt += 1;
        }
        cnt
    });
    bench!("sjt_gen(7)", {
        let mut cnt = 0;
        for _ in sjt_gen(7) {
            cnt += 1;
        }
        cnt
    });
    bench!("sjt_gen(5)", {
        let mut cnt = 0;
        for _ in sjt_gen(5) {
            cnt += 1;
        }
        cnt
    });

    // Ehrlich permutations
    bench!("ehr_gen(8)", {
        let mut cnt = 1;
        for _ in ehr_gen(8) {
            cnt += 1;
        }
        cnt
    });

    // BRGC
    bench!("brgc_gen(12)", {
        let mut cnt = 0;
        for _ in brgc_gen(12) {
            cnt += 1;
        }
        cnt
    });
    bench!("brgc_gen(8)", {
        let mut cnt = 0;
        for _ in brgc_gen(8) {
            cnt += 1;
        }
        cnt
    });

    // Set partition
    bench!("set_partition_gen(11,5)", {
        let mut cnt = 1;
        for _ in set_partition_gen(11, 5) {
            cnt += 1;
        }
        cnt
    });
    bench!("set_partition_gen(8,4)", {
        let mut cnt = 1;
        for _ in set_partition_gen(8, 4) {
            cnt += 1;
        }
        cnt
    });

    // Set bipartition
    bench!("set_bipart_gen(15)", {
        let mut cnt = 1;
        for _ in set_bipart_gen(15) {
            cnt += 1;
        }
        cnt
    });
    bench!("set_bipart_gen(10)", {
        let mut cnt = 1;
        for _ in set_bipart_gen(10) {
            cnt += 1;
        }
        cnt
    });

    // Math
    bench!("comb(100,50)", { comb(100, 50) });
    bench!("factorial(20)", { factorial(20) });
    bench!("stirling2nd(30,10)", { stirling2nd(30, 10) });
    bench!("stirling2nd2(20)", { stirling2nd2(20) });
}
