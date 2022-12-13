# Advent of Code 2022

[Advent of Code 2022](https://adventofcode.com/2022) solutions in Rust. Goals: learn idiomatic Rust and solve problems for general inputs.

## Compilation and running

To run all days:
```
cargo run
```

To run a specific day: `cargo run <day>`
```
cargo run day1
...
cargo run day25
```

To run benchmarks: `cargo run --release bench`

In debug mode, there are additional prints and visualizations. In release mode, just the answer is printed. 

## Benchmarks

For these naive solutions, the following times (n=1) were obtained on an AMD Ryzen 5950X:

| Day (both parts) | Time    |
|------------------|---------|
| 1                | 178 us  |
| 2                | 375 us  |
| 3                | 1.33 us |
| 4                | 424 us  |
| 5                | 145 us  |
| 6                | 633 us  |
| 7                | 409 us  |
| 8                | 2.39 ms |
| 9                | 13.3 ms |
| 10               | 37 us   |
| 11               | 23.6 ms |
| 12               | 2.0 ms  |

Total: 44 ms
