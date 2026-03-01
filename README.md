[![Crates.io](https://img.shields.io/crates/v/ecgen-rs.svg)](https://crates.io/crates/ecgen-rs)
[![Docs.rs](https://docs.rs/ecgen-rs/badge.svg)](https://docs.rs/ecgen-rs)
[![CI](https://github.com/luk036/ecgen-rs/workflows/CI/badge.svg)](https://github.com/luk036/ecgen-rs/actions)
[![codecov](https://codecov.io/gh/luk036/ecgen-rs/branch/master/graph/badge.svg?token=KZnX3rl1gV)](https://codecov.io/gh/luk036/ecgen-rs)

# 🔢 ecgen-rs

A high-performance Rust library for **enumerative combinatoric generation**. This library provides efficient algorithms for generating various combinatorial structures using lazy evaluation with generators.

## ✨ Features

### Combinatorial Structures

- **Combinations** - Generate k-combinations from n elements using the homogeneous revolving-door algorithm
- **Permutations** - Generate all permutations using:
  - Steinhaus-Johnson-Trotter algorithm (adjacent transposition)
  - Ehrlich algorithm (star transposition)
- **Gray Codes** - Binary reflected Gray code generation
- **Set Partitions** - Generate all set partitions into k blocks using Restricted Growth Strings
- **Set Bipartitions** - Specialized generator for set partitions into 2 blocks

### Key Characteristics

- **Lazy Evaluation**: Uses generators to produce combinatorial structures on-demand without storing all results in memory
- **Gray Code Ordering**: All generators produce sequences with minimal changes between consecutive elements
- **High Performance**: Efficient algorithms optimized for speed and memory usage
- **Const Functions**: Mathematical computations use compile-time evaluation
- **Comprehensive Testing**: Full test coverage with property-based tests using QuickCheck

## 🛠️ Installation

### 📦 Cargo

Add this to your `Cargo.toml`:

```toml
[dependencies]
ecgen-rs = "0.1"
```

Or install the binary:

```bash
cargo install ecgen-rs
```

## 📖 Usage Examples

### Generate Combinations

```rust
use ecgen_rs::combin::emk_comb_gen;

// Generate all 2-combinations from 5 elements
let mut gen = emk_comb_gen(5, 2);
while let Some(comb) = gen.next() {
    println!("{:?}", comb);
}
// Output: [0, 1], [1, 2], [0, 2], [2, 3], [1, 3], [3, 4], [2, 4], [1, 4]
```

### Generate Permutations

```rust
use ecgen_rs::perm::sjt_gen;

// Generate all permutations of 3 elements
let mut gen = sjt_gen(3);
while let Some(perm) = gen.next() {
    println!("{:?}", perm);
}
// Output: [1, 2, 3], [1, 3, 2], [3, 1, 2], [2, 1, 3], [2, 3, 1], [3, 2, 1]
```

### Generate Gray Codes

```rust
use ecgen_rs::gray_code::brgc_gen;

// Generate 3-bit Gray codes
let mut gen = brgc_gen(3);
while let Some(pos) = gen.next() {
    // Returns the bit position to flip
    println!("Flip bit at position: {}", pos);
}
```

### Generate Set Partitions

```rust
use ecgen_rs::set_partition::set_partition_gen;

// Generate all partitions of 4 elements into 2 blocks
let mut gen = set_partition_gen(4, 2);
while let Some(part) = gen.next() {
    println!("{:?}", part);
}
```

### Mathematical Functions

```rust
use ecgen_rs::{combin::comb, perm::factorial, set_partition::stirling2nd};

assert_eq!(factorial(5), 120);
assert_eq!(comb(10, 3), 120);
assert_eq!(stirling2nd(5, 3), 25);
```

## 📚 API Documentation

Full API documentation is available at [docs.rs/ecgen-rs](https://docs.rs/ecgen-rs).

## 🏗️ Architecture

The library is organized into modules:

- `combin` - Combination generators and binomial coefficients
- `perm` - Permutation generators and factorial
- `gray_code` - Gray code generators
- `set_partition` - Set partition generators and Stirling numbers
- `set_bipart` - Specialized bipartition generators
- `logging` - Optional logging support

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run with coverage:

```bash
cargo tarpaulin --out Html
```

## 🔬 Benchmarks

Run benchmarks:

```bash
cargo bench
```

## 🤝 Contribution

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📜 License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

This library implements classic algorithms from the literature:
- Homogeneous revolving-door algorithm for combinations (Ehrlich, 1973)
- Steinhaus-Johnson-Trotter algorithm for permutations
- Ehrlich algorithm for permutations (star transposition)
- Binary reflected Gray code generation
- Restricted Growth String method for set partitions

## 📞 Support

For issues, questions, or contributions, please visit the [GitHub repository](https://github.com/luk036/ecgen-rs).
