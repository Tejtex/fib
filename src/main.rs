//! # Fib CLI Executable
//!
//! This is the command-line interface for `fib`.
//! It parses command-line arguments (number of elements, initial values,
//! custom expression, etc.) and calls the library functions to generate
//! the sequence. Supports printing either the full list or just the last number

use std::time::Instant;
use clap::Parser;
use color_print::cprintln;
use num_bigint::BigInt;
use num_prime::nt_funcs::is_prime;
use num_traits::{Signed, Zero};
use textplots::{Chart, Plot, Shape};
use fib::{generate, generate_list, log10_bigint};

/// A powerful cli for generating sequences, mostly fibonacci
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Fib {
    /// Index of the number to generate
    n: u128,

    /// First N values of the sequence
    #[arg(short, long, value_delimiter=',', default_value="1,1", allow_hyphen_values = true)]
    init: Vec<BigInt>,

    /// Coefficients
    #[arg(short, long, value_delimiter=',', default_value="1,1", allow_hyphen_values = true)]
    coeffs: Vec<i64>,

    /// Number of params to use
    #[arg(long, default_value_t = 2)]
    n_params: usize,

    /// Generate a list of numbers
    #[arg(short, long)]
    list: bool,

    /// Plot the numbers in terminal, in log10 scale
    #[arg(short, long)]
    plot: bool,

    /// Compute the fib number mod X
    #[arg(short, long)]
    mod_x: Option<u64>,

    /// Benchmark the generator
    #[arg(short, long)]
    bench: bool,

    /// Don't display the whole number. Instead, show details about it
    #[arg(short, long)]
    details: bool,
}



fn main() {
    let fib = Fib::parse();
    if fib.init.len() != fib.n_params {
        cprintln!("<red>error:</red> length of the init vector has to be the same as n_params!");
        return;
    }
    if fib.coeffs.len() != fib.n_params {
        cprintln!("<red>error:</red> length of the coefficients vector has to be the same as n_params!");
        return;
    }
    if fib.plot && !fib.list {
        cprintln!("<red>error:</red> only use plot with list!");
        return;
    }

    let start = Instant::now();
    if fib.list {
        let seq = generate_list(fib.n, fib.init, fib.n_params, fib.coeffs.as_slice(), fib.mod_x);
        let time = start.elapsed();
        if fib.plot {
            // X: index, Y: log10(value)
            let data: Vec<(f32, f32)> = seq.iter()
                .enumerate()
                .map(|(i, n)| {
                    let x = i as f32;
                    let y = log10_bigint(n) as f32;
                    (x, y)
                })
                .collect();

            Chart::new(180, 30, 0.0, data.len() as f32)
                .lineplot(&Shape::Bars(&data))
                .display();
        } else if fib.details {
            println!("generated {:?} numbers", seq.len());
            println!("max number digits: {:?}", seq.iter().max().unwrap().to_string().chars().count());
            if fib.bench {
                println!("time: {:?}", time);
            }
        } else {
            println!("{:?}", seq);
            if fib.bench {
                println!("{:?}", time);
            }
        }
    } else {
        let res = generate(fib.n, fib.init, fib.n_params, fib.coeffs.as_slice(), fib.mod_x);
        let time = start.elapsed();
        if fib.details {
            println!("digits: {:?}", res.to_string().chars().count());
            println!("bits: {:?}", res.bits());
            println!("bytes: {:?}", res.to_signed_bytes_be().len());
            println!("sign: {:?}", if res.is_negative() {"-".to_string()} else {"+".to_string()});
            if res.is_positive() {
                println!("is prime: {:?}", is_prime(&res.to_biguint().unwrap(), None).probably())
            }
            println!("is even: {:?}", &res % 2 == BigInt::zero());
            if fib.bench {
                println!("time: {:?}", time);
            }

        } else {

            println!("{:?}", res);
            if fib.bench {
                println!("{:?}", time);
            }
        }
    }
}