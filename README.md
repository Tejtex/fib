# Fib 🚀

[![Rust](https://img.shields.io/badge/rust-1.70+-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Fib** is a fast and flexible command-line tool written in Rust for generating Fibonacci sequences and their variants using **BigInt**. It supports custom functions, arbitrary initial values, and adjustable parameters.

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

# Generate sequence with a custom function (RPN)
./fib 10 --init 1,1 --expr "a b +" --list

# Use 3 previous elements (n-params)
./fib 10 --init 1,1,2 --n-params 3 --expr "a b c + +" --list

# Benchmark how many numbers can the generator generate in one second
./fib 0 --bench 1
```
## Notes
- `--expr` uses RPN syntax: operators come after operands.

  - Example: `"a b +"` → `a + b`

  - Example: `"a b * c +"` → `a * b + c`

- Variables: a = last value, b = second-to-last, c = third-to-last, etc.

- --n-params controls how many previous values are passed to the custom function.


---
## License
This project is licensed under the MIT License.

Made with ❤️ in Rust.