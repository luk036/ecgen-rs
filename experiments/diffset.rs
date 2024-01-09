// use std::mem;
use std::process::exit;

const MAX: usize = 100;
const MAX_N: usize = 100;

struct DiffCover {
    n: i32,
    d: i32,
    a: [i32; MAX],
    b: [i32; MAX],
    threshold: i32,
    n_minus_d: i32,
    n1: i32,
    n2: i32,
    size_n: usize,
}

fn print_d(diff_cover: &DiffCover) {
    for i in 1..=diff_cover.d {
        print!("{} ", diff_cover.a[i as usize]);
    }
    println!();
    exit(0);
}

fn gen_d(diff_cover: &mut DiffCover, t: i32, p: i32, tt: i32, diffset: &mut [i8; MAX_N]) {
    // let mut differences = [0; MAX_N];
    // differences.copy_from_slice(&diffset[..diff_cover.size_n]);
    let mut differences = diffset.to_owned();
    for i in 0..t {
        let diff = diff_cover.a[t as usize] - diff_cover.a[i as usize];
        differences[std::cmp::min(diff, diff_cover.n - diff) as usize] = 1;
    }
    if t >= diff_cover.threshold {
        let mut count = 0;
        for i in 1..=diff_cover.n2 {
            if differences[i as usize] != 0 {
                count += 1;
            }
        }
        if count < diff_cover.n1 + tt {
            return;
        }
    }
    let t1 = t + 1;
    if t1 >= diff_cover.d {
        print_d(diff_cover);
    } else {
        let mut tail = diff_cover.n_minus_d + t1;
        let max = diff_cover.a[t1 as usize - p as usize] + diff_cover.a[p as usize];
        let tt1 = t1 * (t1 + 1) / 2;
        if max <= tail {
            diff_cover.a[t1 as usize] = max;
            diff_cover.b[t1 as usize] = diff_cover.b[t1 as usize - p as usize];
            gen_d(diff_cover, t1, p, tt1, &mut differences);
            if diff_cover.b[t1 as usize] == 0 {
                diff_cover.b[t1 as usize] = 1;
                gen_d(diff_cover, t1, t1, tt1, &mut differences);
            }
            tail = max - 1;
        }
        for j in (diff_cover.a[t as usize] + 1..=tail).rev() {
            diff_cover.a[t1 as usize] = j;
            diff_cover.b[t1 as usize] = 1;
            gen_d(diff_cover, t1, t1, tt1, &mut differences);
        }
    }
}

fn init(diff_cover: &mut DiffCover) {
    // for j in 0..=diff_cover.d {
    //     diff_cover.a[j as usize] = 0;
    // }
    diff_cover.a[diff_cover.d as usize] = diff_cover.n;
    diff_cover.a[0] = 0;
    let mut differences = [0; MAX_N];
    differences[0] = 1;
    for j in (((diff_cover.n - 1) / diff_cover.d + 1)..=(diff_cover.n_minus_d + 1)).rev() {
        diff_cover.a[1] = j;
        diff_cover.b[1] = 1;
        gen_d(diff_cover, 1, 1, 1, &mut differences);
    }
    println!("No solution is found.");
}

fn usage() {
    println!("Usage: necklace [n] [density] [threshold]");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        usage();
        return;
    }
    let n: i32 = args[1].parse().unwrap();
    let d: i32 = args[2].parse().unwrap();
    let threshold: i32 = args[3].parse().unwrap();
    if n > d * (d - 1) + 1 {
        println!("Error: N must be less than D*(D-1)+1");
        return;
    }
    let mut diff_cover = DiffCover {
        n,
        d,
        a: [0; MAX],
        b: [0; MAX],
        threshold,
        n_minus_d: n - d,
        n1: n / 2 - d * (d - 1) / 2,
        n2: n / 2,
        size_n: (n / 2 + 1) as usize,
    };
    diff_cover.a[d as usize] = n;
    diff_cover.a[0] = 0;
    init(&mut diff_cover);
}
