// use std::mem;
use std::process::exit;

const MAX: usize = 20;
const MAX_N: usize = 70;

struct DiffCover {
    n: usize,
    d: usize,
    a: [usize; MAX],
    b: [usize; MAX],
    threshold: usize,
    n_minus_d: usize,
    n1: usize,
    n2: usize,
    size_n: usize,
}

impl DiffCover {
    fn print_d(&self) {
        for i in 1..=self.d {
            print!("{} ", self.a[i]);
        }
        println!();
        exit(0);
    }

    fn gen_d(&mut self, t: usize, p: usize, tt: usize, diffset: &mut [i8; MAX_N]) {
        let mut differences = [0; MAX_N];
        differences[..self.size_n].copy_from_slice(&diffset[..self.size_n]);
        // let mut differences = diffset.to_owned();
        for i in 0..t {
            let diff = self.a[t] - self.a[i];
            differences[std::cmp::min(diff, self.n - diff)] = 1;
        }
        if t >= self.threshold {
            let mut count = 0;
            for e in differences[1..=self.n2].iter() {
                if e != &0 {
                    count += 1;
                }
            }
            // let count: i8 = differences[1..=self.n2].iter().sum();
            if count < self.n1 + tt {
                return;
            }
        }
        let t1 = t + 1;
        if t1 >= self.d {
            self.print_d();
        } else {
            let mut tail = self.n_minus_d + t1;
            let max = self.a[t1 - p] + self.a[p];
            let tt1 = t1 * (t1 + 1) / 2;
            if max <= tail {
                self.a[t1] = max;
                self.b[t1] = self.b[t1 - p];
                self.gen_d(t1, p, tt1, &mut differences);
                if self.b[t1] == 0 {
                    self.b[t1] = 1;
                    self.gen_d(t1, t1, tt1, &mut differences);
                }
                tail = max - 1;
            }
            for j in (self.a[t] + 1..=tail).rev() {
                self.a[t1] = j;
                self.b[t1] = 1;
                self.gen_d(t1, t1, tt1, &mut differences);
            }
        }
    }

    fn init(&mut self) {
        // for j in 0..=self.d {
        //     self.a[j] = 0;
        // }
        self.a[self.d] = self.n;
        self.a[0] = 0;
        let mut differences = [0; MAX_N];
        differences[0] = 1;
        for j in (((self.n - 1) / self.d + 1)..=(self.n_minus_d + 1)).rev() {
            self.a[1] = j;
            self.b[1] = 1;
            self.gen_d(1, 1, 1, &mut differences);
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
    let n: usize = args[1].parse().unwrap();
    let d: usize = args[2].parse().unwrap();
    let threshold: usize = args[3].parse().unwrap();
    if n > d * (d - 1) + 1 {
        println!("Error: N must be less than D*(D-1)+1");
        return;
    }
    let mut diffcover = DiffCover {
        n,
        d,
        a: [0; MAX],
        b: [0; MAX],
        threshold,
        n_minus_d: n - d,
        n1: n / 2 - d * (d - 1) / 2,
        n2: n / 2,
        size_n: (n / 2 + 1),
    };
    diffcover.a[d] = n;
    diffcover.a[0] = 0;
    diffcover.init();
}
