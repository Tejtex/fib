//! # Fib CLI Executable
//!
//! This is the command-line interface for `fib`.
//! It parses command-line arguments (number of elements, initial values,
//! custom expression, etc.) and calls the library functions to generate
//! the sequence. Supports printing either the full list or just the last number

use std::collections::VecDeque;
use clap::Parser;
use num_bigint::BigInt;
use fib::{generate, generate_list, parse_custom_bigint};

/// A powerful cli for generating sequences, mostly fibonacci
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Fib {
    /// Index of the number to generate
    n: i64,

    /// First N values of the sequence
    #[arg(short, long, value_delimiter=',', default_value="1,1", allow_hyphen_values = true)]
    init: Vec<BigInt>,

    /// Number of params to use
    #[arg(long, default_value_t = 2)]
    n_params: usize,

    /// Generate a list of numbers
    #[arg(short, long)]
    list: bool,

    /// Function that is used to generate the sequence. In RPN
    #[arg(short, long)]
    func: Option<String>
}

fn f(v: &VecDeque<BigInt>, _n: usize) -> BigInt {
    v.iter().sum()
}


fn main() {
    let fib = Fib::parse();
    let mut func: Box<dyn Fn(&VecDeque<BigInt>, usize) -> BigInt> = Box::new(f);
    if let Some(f) = fib.func {
        func = Box::new(parse_custom_bigint(f));
    }
    if fib.list {
        println!("{:?}", generate_list(fib.n, fib.init, fib.n_params, func))
    } else {

        println!("{:?}", generate(fib.n, fib.init, fib.n_params, func));
    }
}