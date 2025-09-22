# Fib 🚀

[![Rust](https://img.shields.io/badge/rust-1.70+-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Fib** is a fast and flexible command-line tool written in Rust for generating Fibonacci sequences and their variants using **BigInt**. It supports custom functions, arbitrary initial values, and adjustable parameters.
See the [documentation](https://tejtex.github.io/fib) and join our [discord server](https://discord.gg/yhRanS6Z8w)

---

## Features

- Generate classical Fibonacci sequences with **arbitrary-length integers (BigInt)**.
- Specify custom initial values with `--init`.
- Define your own sequence function using **Reverse Polish Notation (RPN)** with `--expr`.
- Control the number of previous elements used in the function with `--n-params`.
- Output a **full sequence** (`--list`) or just the **last number**.
- Super fast thanks to `VecDeque` optimization.

---

## Installation

```bash
git clone https://github.com/Tejtex/fib.git
cd fib
cargo build --release
```
The binary will be available at:

`target/release/fib`

---

## Usage
```
# Generate 20th Fibonacci number
./fib 20

# Generate 20 Fibonacci numbers (default a=1, b=1)
./fib 20 --list

# Generate 20 Fibonacci numbers (default a=1, b=1) and plot them on a logarithmic scale
./fib 20 --list --plot

# Generate a custom sequence with initial values -1, 1
./fib 20 --init -1,1 --list

# Generate sequence with custom coefficients
./fib 10 --init 1,1 --coeffs 1,3 --list

# Use 3 previous elements (n-params)
./fib 10 --init 1,1,2 --n-params 3 --coeffs 1,1,1 --list

# Benchmark the binary
./fib 1000 --bench

# Show details instead of a giant number
./fib 10000 --details

# Generate 20th Fibonacci number modulo 10
./fib 20 --mod-x 10
```
## Notes
- --n-params controls how many previous values are used.
- --mod-x is applied to each iteration not only to the last one
- --coeffs controls the coefficients of each param. Example: ./fib 20 --coeffs 1,2 means f(n) = 1 * f(n-1) + 2 * f(n-2)


---
## License
This project is licensed under the MIT License.

Made with ❤️ in Rust.