# Maniac 🌀

**Maniac** is a Rust library focused on implementing, exploring, and benchmarking a wide variety of sorting algorithms. Its goal is to offer a collection of canonical and experimental sorting methods with an easy way to compare their performance against Rust’s standard library sorter.

Whether you're learning how sorting works under the hood, experimenting with algorithm variants, or analyzing performance trade-offs — Maniac gives you the playground and tools to do it.

---

## Features

- ✅ A growing collection of sorting algorithm implementations
- ✅ A unified `Sorter` trait to abstract over sorting strategies
- 🚧 A planned `Benchmarker` utility for comparing sorting methods
- 🧪 Built-in test suite for correctness
- 📦 Designed as a minimal, zero-dependency library

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
maniac = "0.1.0"
```

---

## Usage

```rust
use maniac::{Sorter, comb_sorter::CombSorter};

let mut items = vec![5, 3, 1, 4, 2];
CombSorter::default().sort(&mut items);

assert_eq!(items, vec![1, 2, 3, 4, 5]);
```

_(Full usage examples coming soon.)_

---

## Contributing

Contributions are welcome and encouraged!

If you'd like to add a new sorting algorithm, improve performance, fix a bug, or help with benchmarking tools, feel free to:

1. Fork the repository
2. Create a new branch
3. Commit your changes
4. Open a pull request

Please try to write tests for your contributions and keep things idiomatic and simple. If in doubt, open an issue or discussion first — we’re happy to help!

---

## License

This project is licensed under :

- UNLICENSE ([UNLICENSE](UNLICENSE) or [https://unlicense.org/](https://unlicense.org/))

All contribution will fall under Unlicense

---

## TODO

- [ ] Implement all classic sorting algorithms (Bubble, Insertion, Merge, Quick, Heap, Radix, etc.)
- [ ] Implement less common and exotic sorts (e.g. Comb, Shell, TimSort, Stooge)
- [ ] Create a `Benchmarker` struct or trait that can:

  - Take any dataset and run all available sorting strategies
  - Compare results to the standard library’s `.sort()` in terms of time and allocations

- [ ] Add support for plotting or reporting benchmark results
- [ ] Provide rich examples and docs for each algorithm
- [ ] Test behavior on large datasets and edge cases
- [ ] Property testing with quickcheck or equivalent crate

---

Happy sorting! 🧠💡
