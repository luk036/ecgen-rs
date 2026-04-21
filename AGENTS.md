# AGENTS.md - Agent Coding Guidelines for ecgen-rs

This document provides guidance for agentic coding agents operating in this repository.

## Project Overview

**ecgen-rs** is a Rust library for enumerative combinatorics generation. It provides efficient algorithms for generating combinations, permutations, Gray codes, and set partitions using lazy evaluation with generators.

- **Edition**: 2021
- **License**: MIT OR Apache-2.0
- **Repository**: https://github.com/luk036/ecgen-rs

## Module Structure

| Module | Description |
|--------|-------------|
| `combin` | Combination generators & binomial coefficients |
| `perm` | Permutation generators (SJT, Ehrlich) & factorial |
| `gray_code` | Binary reflected Gray code generators |
| `set_partition` | Set partition generators & Stirling numbers |
| `set_bipart` | Bipartition generators & stirling2nd2 |
| `diffset` | Difference set generation & validation |
| `logging` | Optional logging support (feature-gated) |

---

## Build, Test, and Lint Commands

### All-in-One CI Checks
```bash
cargo test --all-features --workspace           # Run all tests
cargo clippy --all-targets --all-features      # Lint with Clippy
cargo fmt --all -- --check                  # Check formatting
cargo doc --no-deps --document-private-items --all-features --workspace --examples
```

### Single Test
```bash
cargo test test_brgc           # Run specific test
cargo test test_sjt -- --nocapture  # With output
```

### Code Quality
```bash
cargo fmt --all               # Format code
cargo clippy --all-targets --all-features --workspace
cargo doc --no-deps --document-private-items --all-features
```

### QuickCheck Tests
```bash
cargo run --example quickcheck_tests
```

---

## Code Style Guidelines

### Formatting
- **Use rustfmt**: Run `cargo fmt --all` before committing
- **Indentation**: 4 spaces
- **Line length**: Prefer ≤ 100 chars

### Imports
- External: `use genawaiter::sync::{Gen, GenBoxed};`
- Group: stdlib → external → local

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Functions | snake_case | `emk_comb_gen` |
| Types | PascalCase | `GenBoxed` |
| Constants | SCREAMING_SNAKE | `max_n = 70` |
| Tests | test_<feature> | `test_emk_even_odd` |

### Documentation

All public APIs must have doc comments with examples:

```rust
/// Calculate binomial coefficient C(n,k)
///
/// # Examples
/// ```
/// use ecgen::comb;
/// assert_eq!(comb(5, 2), 10);
/// ```
pub const fn comb(n: usize, k: usize) -> usize { ... }
```

### Error Handling
No runtime errors for valid inputs. Use early returns:
```rust
if k >= n || k == 0 { return 1; }
```

### Testing Conventions

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_feature_name() {
        assert_eq!(expected, actual);
    }
}
```

### Generator Pattern

All generators use genawaiter:

```rust
use genawaiter::sync::{Gen, GenBoxed};

pub fn generator_name(n: usize) -> GenBoxed<OutputType> {
    Gen::new_boxed(|co| async move {
        co.yield_(value).await;
    })
}
```

### Performance Guidelines

- **Use `#[inline]`** for small functions
- **Use `const fn`** for compile-time evaluation
- **Use `genawaiter`** for lazy evaluation

---

## Common Workflows

### Adding a New Generator
1. Add to appropriate module
2. Return `GenBoxed<T>` using genawaiter
3. Add doc comments with examples
4. Add unit tests in `#[cfg(test)]` module
5. Export from `lib.rs` if public API

### Adding Mathematical Functions
1. Use `const fn` for compile-time evaluation
2. Use `#[inline]` for small functions
3. Handle edge cases (n=0, k=0, etc.)

---

## Dependencies

| Dependency | Purpose |
|------------|---------|
| genawaiter | Async generators |
| log, env_logger | Logging (optional) |
| quickcheck | Property-based testing (dev) |
| criterion | Benchmarks (dev) |

---

## CI Pipeline

GitHub Actions (`.github/workflows/ci.yml`):
1. Test Suite
2. Rustfmt
3. Clippy
4. Docs (fails on warnings)

---

## Useful Patterns

### Const Function
```rust
pub const fn factorial(n: usize) -> usize {
    if n < 2 { 1 } else { n * factorial(n - 1) }
}
```

### Generator with Multiple Yields
```rust
pub fn example_gen(n: usize) -> GenBoxed<(usize, usize)> {
    Gen::new_boxed(|co| async move {
        for i in 0..n {
            co.yield_((i, i + 1)).await;
        }
    })
}
```

### Even/Odd Pattern
```rust
if k.is_multiple_of(2) {
    for item in gen_even(n, k) { co.yield_(item).await; }
} else {
    for item in gen_odd(n, k) { co.yield_(item).await; }
}
```