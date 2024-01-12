// use std::mem;
use std::process::exit;

const MAX: usize = 20;
const MAX_N: usize = 70;

struct DiffCover {
    n: i32,
    d: i32,
    a: [i32; MAX],
    b: [i32; MAX],
    differences: [i32; MAX_N],
    threshold: i32,
    n_minus_d: i32,
    n1: i32,
    n2: usize,
}

impl DiffCover {
    fn print_d(&self) {
        for i in self.a[1..=self.d as usize].iter() {
            print!("{} ", i);
        }
        println!();
        exit(0);
    }

    fn gen_d(&mut self, t: i32, p: i32, tt: i32) {
        let mut diff_t = [0; MAX];
        let at = self.a[t as usize];

        for i in 0..t {
            let diff = at - self.a[i as usize];
            let diff_min = std::cmp::min(diff, self.n - diff);
            diff_t[i as usize] = diff_min; // cache the value
            self.differences[diff_min as usize] += 1;
        }
        if t >= self.threshold {
            let mut count = 0;
            for i in self.differences[1..=self.n2].iter() {
                if *i != 0 {
                    count += 1;
                }
            }
            if count < self.n1 + tt {
                for e in diff_t[0..t as usize].iter() {
                    self.differences[e as usize] -= 1;
                }
                return;
            }
        }
        let t1 = t + 1;
        if t1 >= self.d {
            self.print_d();
        } else {
            let mut tail = self.n_minus_d + t1;
            let max = self.a[t1 as usize - p as usize] + self.a[p as usize];
            let tt1 = t1 * (t1 + 1) / 2;
            if max <= tail {
                self.a[t1 as usize] = max;
                self.b[t1 as usize] = self.b[t1 as usize - p as usize];
                self.gen_d(t1, p, tt1);
                if self.b[t1 as usize] == 0 {
                    self.b[t1 as usize] = 1;
                    self.gen_d(t1, t1, tt1);
                }
                tail = max - 1;
            }
            for j in (at + 1..=tail).rev() {
                self.a[t1 as usize] = j;
                self.b[t1 as usize] = 1;
                self.gen_d(t1, t1, tt1);
            }
        }

        for e in diff_t[0..t as usize].iter() {
            self.differences[e as usize] -= 1;
        }
    }

    fn init(&mut self) {
        // for j in 0..=self.d {
        //     self.a[j] = 0;
        // }
        self.differences[0] = 1;
        for j in (((self.n - 1) / self.d + 1)..=(self.n_minus_d + 1)).rev() {
            self.a[1] = j;
            self.b[1] = 1;
            self.gen_d(1, 1, 1);
        }
        println!("No solution is found.");
    }
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
    let mut diffcover = DiffCover {
        n,
        d,
        a: [0; MAX],
        b: [0; MAX],
        differences: [0; MAX_N],
        threshold,
        n_minus_d: n - d,
        n1: n / 2 - d * (d - 1) / 2,
        n2: (n / 2) as usize,
    };
    diffcover.a[d as usize] = n;
    diffcover.a[0] = 0;
    diffcover.init();
}
