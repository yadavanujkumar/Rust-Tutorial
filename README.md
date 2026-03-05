# Rust Tutorial

A hands-on, beginner-to-intermediate Rust tutorial organised as a Cargo workspace. Each chapter is a self-contained crate with heavily commented source code and a built-in test suite you can run instantly.

---

## Prerequisites

Install Rust (includes `cargo`) via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify the installation:

```bash
rustc --version   # e.g. rustc 1.93.1
cargo --version   # e.g. cargo 1.93.1
```

---

## Repository Layout

```
Rust-Tutorial/
├── Cargo.toml                          # Workspace manifest
└── tutorials/
    ├── 01_hello_world/
    ├── 02_variables_and_types/
    ├── 03_control_flow/
    ├── 04_functions/
    ├── 05_ownership/
    ├── 06_structs/
    ├── 07_enums_and_pattern_matching/
    ├── 08_collections/
    ├── 09_error_handling/
    ├── 10_traits_and_generics/
    └── 11_iterators_and_closures/
```

Each directory contains a `Cargo.toml` and `src/main.rs` with:
- **Explanatory comments** describing every concept as it is introduced
- **Runnable `main` function** that demonstrates the concepts
- **`#[cfg(test)]` module** with unit tests

---

## Tutorials

| # | Topic | Key Concepts |
|---|-------|-------------|
| 01 | **Hello World** | `main`, `println!`, macros, string formatting |
| 02 | **Variables & Types** | `let`, `mut`, shadowing, `const`, integers, floats, booleans, chars, tuples, arrays |
| 03 | **Control Flow** | `if`/`else if`/`else`, `if` as an expression, `loop`, `while`, `for`, ranges, loop labels, `continue` |
| 04 | **Functions** | Parameters, return values, statements vs expressions, function pointers, closures, `move` closures |
| 05 | **Ownership** | Ownership rules, move semantics, `Clone`, `Copy` types, references, borrowing, mutable references, slices |
| 06 | **Structs** | Struct definition, instantiation, update syntax, tuple structs, `impl` blocks, methods, associated functions |
| 07 | **Enums & Pattern Matching** | Enum variants with data, `Option<T>`, `match`, `if let`, `while let`, guard clauses |
| 08 | **Collections** | `Vec<T>`, `HashMap<K,V>`, `HashSet<T>`, entry API, sorting, set operations |
| 09 | **Error Handling** | `panic!`, `Result<T,E>`, `?` operator, `unwrap`/`expect`, custom error types, `Display` |
| 10 | **Traits & Generics** | Trait definition, default impls, `impl Trait`, trait bounds, `where` clauses, generic structs, trait objects (`dyn`) |
| 11 | **Iterators & Closures** | `Iterator` trait, `iter()`/`into_iter()`, adapters (`map`, `filter`, `flat_map`, `zip`, …), `collect`, custom iterators, `Fn`/`FnMut`/`FnOnce` |

---

## Running the Tutorials

### Run a single tutorial

```bash
cargo run -p hello_world
cargo run -p variables_and_types
cargo run -p control_flow
cargo run -p functions
cargo run -p ownership
cargo run -p structs
cargo run -p enums_and_pattern_matching
cargo run -p collections
cargo run -p error_handling
cargo run -p traits_and_generics
cargo run -p iterators_and_closures
```

### Run all tutorials sequentially

```bash
for pkg in hello_world variables_and_types control_flow functions ownership \
           structs enums_and_pattern_matching collections error_handling \
           traits_and_generics iterators_and_closures; do
    echo "=== $pkg ==="
    cargo run -p "$pkg" --quiet
done
```

---

## Running the Tests

### Test a single tutorial

```bash
cargo test -p hello_world
cargo test -p ownership
# etc.
```

### Test the entire workspace

```bash
cargo test
```

Expected output: **83 tests pass, 0 failures**.

---

## Learning Path

Work through the tutorials **in order** — later chapters assume knowledge from earlier ones:

```
01 → 02 → 03 → 04 → 05 ──► 06 → 07 → 08
                │                        │
                └────────────────────────┘
                                         ↓
                              09 → 10 → 11
```

Ownership (05) is the hardest chapter for newcomers; take your time there before moving on.

---

## Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/) — the official free book
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) — small exercises
- [Rust Playground](https://play.rust-lang.org/) — run Rust in the browser
- [crates.io](https://crates.io/) — the Rust package registry

---

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.