mod gray_code;
use gray_code::brgc_gen;
mod perm;
use crate::perm::ehr_gen;
use crate::perm::sjt_gen;
mod combin;
use crate::combin::emk_gen;
mod set_partition;
use crate::set_partition::set_partition_gen;
mod set_bipart;
use crate::set_bipart::set_bipart_gen;

fn print_emk() {
    let mut combin = [1, 1, 0];
    println!("{:?}", combin);
    for (i, j) in emk_gen(3, 2) {
        combin.swap(i, j);
        println!("{:?}", combin);
    }
}

fn print_gray_code() {
    let mut lst = ["⬜"; 3];
    println!("{}", lst.concat());
    for n in brgc_gen(lst.len()) {
        lst[n] = if lst[n] == "⬜" { "⬛" } else { "⬜" };
        println!("{}", lst.concat());
    }
}

fn print_sjt() {
    let mut perm = ["🧧", "🥭", "🫐", "🦠"];
    for n in sjt_gen(perm.len()) {
        println!("{}", perm.concat());
        perm.swap(n, n + 1);
    }
}

fn print_ehr() {
    let mut perm = ["🧧", "🥭", "🫐", "🦠"];
    for n in ehr_gen(perm.len()) {
        println!("{}", perm.concat());
        perm.swap(0, n);
    }
}

fn print_set_partition() {
    const N: usize = 5;
    const K: usize = 2;

    // 0 0 0 1 2
    let mut b = [0; N + 1];
    let offset = N - K + 1;
    for i in 1..K {
        b[offset + i] = i;
    }
    for (x, y) in set_partition_gen(N, K) {
        let old = b[x];
        b[x] = y;
        println!("Move {} from Block {} to Block {}", x, old, y);
    }
}

fn print_set_bipart() {
    const N: usize = 5;

    // 0 0 0 0 0 1
    let mut b = [0; N + 1];
    b[N] = 1; // b[0] is unused
    for x in set_bipart_gen(N) {
        let old = b[x];
        b[x] = 1 - b[x];
        println!("Move {} from Block {} to Block {}", x, old, b[x]);
    }
}

fn main() {
    println!("EMK-----------------");
    print_emk();
    println!("Gray Code-----------");
    print_gray_code();
    println!("SJT-----------------");
    print_sjt();
    println!("EHR-----------------");
    print_ehr();
    println!("Set Partition-------");
    print_set_partition();
    println!("Set Bi-Partition----");
    print_set_bipart();
}
