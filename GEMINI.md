# Project Overview

`ecgen-rs` is a Rust library focused on enumerative combinatoric generation. It provides a collection of algorithms and functions for generating various combinatorial objects, including:

*   Combinations
*   Permutations (using Steinhaus-Johnson-Trotter and Even-Headed-Reverse algorithms)
*   Binary Reflected Gray Codes
*   Set Bipartitions
*   Set Partitions

The project leverages the `genawaiter` crate for asynchronous generation, allowing for efficient iteration over combinatorial sequences. Benchmarking is handled using the `criterion` crate.

# Building and Running

This project is a Rust library and executable.

*   **Build the project:**
    ```bash
    cargo build
    ```
*   **Run the example executable:**
    ```bash
    cargo run
    ```
    (This executes the example found in `src/main.rs`, which demonstrates `genawaiter`.)
*   **Run tests:**
    ```bash
    cargo test --all-features --workspace
    ```
*   **Install the library as a command-line tool:**
    ```bash
    cargo install ecgen-rs
    ```

# Development Conventions

The project adheres to standard Rust development practices, enforced through its CI pipeline:

*   **Code Formatting:** Code style is maintained using `rustfmt`.
    *   To check formatting: `cargo fmt --all --check`
*   **Linting:** Code quality and potential issues are identified using `clippy`.
    *   To run clippy: `cargo clippy --all-targets`
*   **Testing:** Unit tests are integrated within the source code (e.g., `src/lib.rs`) and are executed as part of the CI process.
*   **Documentation:** Documentation is generated and checked to ensure completeness and correctness.
    *   To check documentation: `cargo doc --no-deps --document-private-items --all-features --workspace --examples`
