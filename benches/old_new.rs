use std::time::Instant;
use ecgen::combin::emk_comb_gen;
use ecgen::combin_old::emk_gen;
use ecgen::set_partition::set_partition_gen;
use ecgen::set_partition_old::set_partition_gen_old;

macro_rules! bench {
    ($name:expr, $body:expr) => {{
        let start = Instant::now();
        let result = $body;
        let dur = start.elapsed();
        println!("{:<30} {:>10}  {:>8.3?}", $name, result, dur);
    }};
}

fn main() {
    println!("{:<30} {:>10}  {:>10}", "Operation", "Count", "Time");
    println!("{:-<55}", "");

    // EMK old vs new
    println!("\n--- EMK combinations n=16,k=5 ---");
    let expected = ecgen::comb(16, 5);
    bench!("new (4-helper)", {
        let mut cnt = 1;
        for _ in emk_comb_gen(16, 5) { cnt += 1; }
        cnt
    });
    assert_eq!(expected, ecgen::comb(16, 5));
    bench!("old (2-helper)", {
        let mut cnt = 1;
        for _ in emk_gen(16, 5) { cnt += 1; }
        cnt
    });

    println!("\n--- EMK combinations n=18,k=7 ---");
    bench!("new (4-helper)", {
        let mut cnt = 1;
        for _ in emk_comb_gen(18, 7) { cnt += 1; }
        cnt
    });
    bench!("old (2-helper)", {
        let mut cnt = 1;
        for _ in emk_gen(18, 7) { cnt += 1; }
        cnt
    });

    // Set partition old vs new
    println!("\n--- Set partition n=11,k=5 ---");
    bench!("new (8-function)", {
        let mut cnt = 1;
        for _ in set_partition_gen(11, 5) { cnt += 1; }
        cnt
    });
    bench!("old (8-function)", {
        let mut cnt = 1;
        for _ in set_partition_gen_old(11, 5) { cnt += 1; }
        cnt
    });

    println!("\n--- Set partition n=14,k=3 ---");
    bench!("new (8-function)", {
        let mut cnt = 1;
        for _ in set_partition_gen(14, 3) { cnt += 1; }
        cnt
    });
    bench!("old (8-function)", {
        let mut cnt = 1;
        for _ in set_partition_gen_old(14, 3) { cnt += 1; }
        cnt
    });
}
