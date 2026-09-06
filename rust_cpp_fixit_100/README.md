# Rust Fix-It: 100 Exercises for C++ Developers

This archive contains **100 small Rust debugging/fix-it exercises**, grouped into 10 subjects with 10 examples each. Every exercise has a matching solution.

Each `.rs` file also contains a **correct C++ equivalent in comments** between `CPP_EQUIVALENT_BEGIN` and `CPP_EQUIVALENT_END`. The C++ is intended to show the behavior/concept the Rust code is trying to express, not necessarily the most idiomatic production C++ design.

## Subjects

1. Ownership and Moves
2. Borrowing and References
3. Strings and Slices
4. Structs, Enums, and Patterns
5. Option, Result, and Error Handling
6. Collections and Iterators
7. Traits and Generics
8. Lifetimes
9. Smart Pointers and Interior Mutability
10. Threads, Channels, and Synchronization

## Layout

Each subject folder contains:

- `exercises/` — the intentionally flawed Rust examples.
- `solutions/` — corrected Rust versions, with a short `Fix:` comment.
- `README.md` — an index for that subject.

## Suggested classroom workflow

Give students only the `exercises/` tree at first. Ask them to:

1. Predict whether the issue is compile-time, runtime, warning/API misuse, or logic/ownership related.
2. Explain the C++/Rust semantic difference involved.
3. Make the smallest Rust change that preserves the intended behavior.
4. Compare with the matching file under `solutions/`.

Most examples are deliberately tiny so they can be discussed in a few minutes.

## Checking the Rust solutions

Run from the archive root on a system with `rustc` installed:

```sh
./check_rust_solutions.sh
```

The examples use only the Rust standard library and are intended for a current stable Rust compiler.

## Notes

A few exercises demonstrate issues that may compile but are still intentionally wrong for the stated goal, such as ignored `Result` values, lazy iterators that are never consumed, or a lock/channel pattern that can block indefinitely. Those are marked by their solution comment.
