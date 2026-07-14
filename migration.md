# Migration Report: genawaiter → next_gen

**Date:** 2026-07-11
**Project:** ecgen-rs (v0.1.4)
**Attempted Migration:** `genawaiter` (v0.99.1) → `next-gen` (v0.1.1)
**Status:** ABORTED

## Summary

An attempt was made to replace the `genawaiter` crate with `next_gen` as the
generator/coroutine provider for ecgen-rs. After extensive effort involving
multiple automated migration scripts and manual fixes, the migration was
aborted due to fundamental incompatibilities with the codebase's recursive
generator patterns.

## Codebase Characteristics

The ecgen-rs library contains **33 generator functions** across 7 source files:

| File | Generator Functions | Pattern |
|------|-------------------|---------|
| `gray_code.rs` | 1 | Recursive (self-calling) |
| `combin.rs` | 5 | Mutually recursive (cross-calling) |
| `combin_old.rs` | 2 | Mutually recursive |
| `perm.rs` | 3 | Recursive + iterative |
| `set_bipart.rs` | 4 | Mutually recursive |
| `set_partition.rs` | 9 | Mutually recursive (8-function cycle) |
| `set_partition_old.rs` | 9 | Mutually recursive (8-function cycle) |

All generators use lazy evaluation via `genawaiter`'s `GenBoxed<T>` type,
returned from `Gen::new_boxed(|co| async move { ... })`.

## The Two Crates

### genawaiter (current)

```rust
use genawaiter::sync::{Gen, GenBoxed};

pub fn brgc_gen(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| async move {
        for i in brgc_gen(n - 1) {
            co.yield_(i).await;
        }
        co.yield_(n - 1).await;
    })
}
```

- Uses `async`/`await` under the hood
- `GenBoxed<T>` implements `IntoIterator` directly
- Recursive calls work naturally: `for x in gen(args)` creates a nested generator
- Each recursive `Gen::new_boxed()` heap-allocates a new generator state machine
- The `|co|` closure receives a `Co` handle for yielding

### next_gen (target)

```rust
use ::next_gen::prelude::*;

#[generator(yield(usize))]
pub fn brgc_gen(n: usize) {
    for i in brgc_gen.call_boxed((n - 1,)).into_iter() {
        yield_!(i);
    }
    yield_!(n - 1);
}
```

- Uses the `#[generator(yield(T))]` proc-macro attribute
- `yield_!(x)` replaces `co.yield_(x).await`
- Recursive calls REQUIRE `.call_boxed((args,)).into_iter()` to avoid
  infinitely-sized types (recursive async fn limitation)
- `call_boxed` heap-allocates via `Box::pin`
- The function signature changes: no explicit return type, the attribute
  handles type transformation

## Migration Challenges

### 1. Brace Structure Incompatibility

**genawaiter pattern:**
```rust
pub fn f() -> GenBoxed<T> {
    Gen::new_boxed(|co| async move {
        body               // 2 levels of nesting inside fn body
    })
}
```

**next_gen pattern:**
```rust
#[generator(yield(T))]
pub fn f() {
    body                   // 0 levels of nesting (body is directly in fn)
}
```

The two extra brace levels (`Gen::new_boxed(|co|` and `async move {`) must
be removed during migration. Automated removal is error-prone because:

- Some generators use `Gen::new_boxed(|co| {` without `async move`
- The closing `})` is on a separate line with inconsistent indentation
- Removing braces incorrectly breaks the surrounding control flow
- Regex-based removal cannot reliably track brace depth across 33 functions

### 2. Recursive Call Transformation

Every recursive generator call must be changed from:
```rust
for x in gen(args) {     // genawaiter: direct call, IntoIterator
    co.yield_(x).await;
}
```
to:
```rust
for x in gen.call_boxed((args,)).into_iter() {  // next_gen: explicit boxing
    yield_!(x);
}
```

This adds **heap allocation per recursion level** via `Box::pin`, which is
the same cost as genawaiter's `Gen::new_boxed()`. However, the syntax
change is non-trivial for mutually recursive functions where 8+ functions
call each other in complex patterns.

### 3. Cross-File Function Name Collisions

The function names `gen0_even`, `gen1_even`, `neg1_even`, etc. appear in
**three different files** (`set_bipart.rs`, `set_partition.rs`,
`set_partition_old.rs`) with **different yield types**:

- `set_bipart.rs::gen0_even` → yields `usize`
- `set_partition.rs::gen0_even` → yields `(usize, usize)`
- `set_partition_old.rs::gen0_even` → yields `(usize, usize)`

This makes find-and-replace migration impossible — each call site must be
examined in context to determine the correct `#[generator(yield(...))]`
type annotation.

### 4. Generated State Machine Size

The `#[generator]` proc-macro transforms the function into a state machine
that implements `Generator`. For deeply recursive generators (like the
8-function cycle in `set_partition`), the generated state machine can become
large, potentially impacting compile times and binary size.

### 5. Error Count Progression

| Attempt | Errors | Notes |
|---------|--------|-------|
| Baseline (genawaiter) | 0 | Clean build |
| Simple import swap | ~140 | `GenBoxed`, `Gen`, `co.yield_` not found |
| + attribute insertion | 261 | Wrong yield types, missing call_boxed |
| + per-file yield types | 111 | Correct yield types, but async blocks remain |
| + brace cleanup | 4 | Remaining `async move {` blocks |
| + final cleanup | 1 | Brace mismatch from over-eager dedup |

The error count decreased steadily but the final brace-dedup step
introduced structural breakage that could only be fixed by manual
per-function inspection of all 33 functions.

## Recommendation

**Do not migrate.** The `genawaiter` crate is the correct choice for this
codebase for the following reasons:

1. **Natural recursive pattern:** `for x in gen(args)` works directly
   without needing `.call_boxed().into_iter()` at every call site
2. **Familiar syntax:** `async move { co.yield_(x).await }` maps to
   standard Rust async/await — any Rust developer can understand it
3. **Mature crate:** genawaiter v0.99.1 is stable and widely used
4. **No migration benefit:** Both crates heap-allocate per generator
   instance — there is no performance advantage to switching
5. **Code complexity:** The 33-function mutually-recursive structure
   makes automated migration infeasible

If a migration to `next_gen` were attempted in the future, it should be
done by **rewriting each generator function from scratch** rather than
attempting automated transformation. The manual rewrite would take
approximately 4–6 hours for a developer familiar with both crates.

## Cleanup

All migration-related changes were reverted via `git checkout -- src/`.
The `Cargo.toml` was restored to use `genawaiter`. The `next-gen` crate
dependency was removed.
