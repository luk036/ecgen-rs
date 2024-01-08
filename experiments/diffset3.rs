/*
 * Copyright (c) 2019 Joe Sawada
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
// use std::mem;

const MAX: usize = 20;
const MAX_N: usize = 70;

//-------------------------------------------------------------
// GLOBAL VARIABLES
//-------------------------------------------------------------
static mut N: usize = 0;
static mut D: usize = 0;
static mut a: [usize; MAX] = [0; MAX];
static mut b: [usize; MAX] = [0; MAX];
static mut THRESHOLD: usize = 0;
static mut D_MINUS_1: usize = 0;
static mut D_TIMES_D_MINUS_1: usize = 0;
static mut N_MINUS_D: usize = 0;
static mut N1: usize = 0;
static mut N2: usize = 0;
static mut SIZE_N: usize = 0;

//-------------------------------------------------------------
/**
 * Prints the generated set to stdout and exits.
 */
fn print_d() {
    // print a
    for i in 1..=unsafe { D } {
        print!("{} ", unsafe { a[i] });
    }
    println!();
    std::process::exit(0);
}

/*-----------------------------------------------------------*/
// FIXED DENSITY
/*-----------------------------------------------------------*/
/**
 * Recursively generates all possible D-sets by building them up one
 * element at a time.
 *
 * t - Current element index being added
 * p - Previous element index
 * tt - Triangle number of current index t
 * diffset[] - Bit array tracking differences between elements
 */
fn gen_d(t: usize, p: usize, tt: usize, diffset: &mut [i8; MAX_N]) {
    // let mut differences: [i8; MAX_N] = [0; MAX_N];
    // differences.copy_from_slice(&diffset[0..unsafe { SIZE_N }]);
    let mut differences: [i8; MAX_N] = diffset.to_owned();

    for i in 0..t {
        let diff = unsafe { a[t] - a[i] };
        let n_diff = unsafe { N - diff };
        let min_diff = if diff <= n_diff { diff } else { n_diff };
        differences[min_diff] = 1;
    }
    if t >= unsafe { THRESHOLD } {
        let mut count = 0;
        for i in 1..=unsafe { N2 } {
            if differences[i] != 0 {
                count += 1;
            }
        }
        if count < unsafe { N1 + tt } {
            return;
        }
    }
    let t1 = t + 1;
    if t1 >= unsafe { D } {
        print_d();
    } else {
        let mut tail = unsafe { N_MINUS_D + t1 };
        let max = unsafe { a[t1 - p] + a[p] };
        let tt1 = t1 * (t1 + 1) / 2;
        if max <= tail {
            unsafe {
                a[t1] = max;
                b[t1] = b[t1 - p];
            }
            gen_d(t1, p, tt1, &mut differences);
            if unsafe { b[t1] } == 0 {
                unsafe {
                    b[t1] = 1;
                }
                gen_d(t1, t1, tt1, &mut differences);
            }
            tail = max - 1;
        }
        for j in (unsafe { a[t] + 1 }..=tail).rev() {
            unsafe {
                a[t1] = j;
                b[t1] = 1;
            }
            gen_d(t1, t1, tt1, &mut differences);
        }
    }
}

/*------------------------------------------------------------*/
/*------------------------------------------------------------*/
/**
 * Initializes global variables and arrays used for generating necklaces.
 * Sets up the 'a' and 'b' arrays to track candidate necklaces,
 * the 'differences' array to track differences between necklace elements,
 * and key constants used in the generation algorithms.
 * Calls GenD() recursively to generate all possible necklaces.
 */
fn init() {
    unsafe {
        // for j in 0..=D {
        //     a[j] = 0;
        // }
        a[D] = N;
        a[0] = 0; // for convenience
        D_MINUS_1 = D - 1;
        D_TIMES_D_MINUS_1 = D * (D - 1);
        N_MINUS_D = N - D;
        N2 = N / 2;
        N1 = N2 - D_TIMES_D_MINUS_1 / 2;
        SIZE_N = N2 + 1;
        // let mut differences: Vec<i8> = vec![0; SIZE_N];
        let mut differences: [i8; MAX_N] = [0; MAX_N];
        differences[0] = 1;
        for j in ((N - 1) / D + 1..=N - D + 1).rev() {
            a[1] = j;
            b[1] = 1;
            gen_d(1, 1, 1, &mut differences);
        }
        println!("{}, {}, {}", N, D, THRESHOLD);
        println!("No solution is found.");
    }
}

//------------------------------------------------------
fn usage() {
    println!("Usage: necklace [n] [density] [threshold]");
}

//--------------------------------------------------------------------------------
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        usage();
        std::process::exit(1);
    }
    unsafe {
        N = args[1].parse().unwrap();
        D = args[2].parse().unwrap();
        THRESHOLD = args[3].parse().unwrap();
    }
    if unsafe { N } > unsafe { D * (D - 1) + 1 } {
        println!("Error: N must be less than D*(D-1)+1");
        std::process::exit(1);
    }
    init();
    std::process::exit(0);
}



