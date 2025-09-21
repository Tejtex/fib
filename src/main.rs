//! # Fib CLI Executable
//!
//! This is the command-line interface for `fib`.
//! It parses command-line arguments (number of elements, initial values,
//! custom expression, etc.) and calls the library functions to generate
//! the sequence. Supports printing either the full list or just the last number

use std::collections::VecDeque;
use clap::Parser;
use color_print::cprintln;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use textplots::{Chart, Plot, Shape};
use fib::{generate, generate_bench, generate_list, log10_bigint, parse_custom_bigint};

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
    func: Option<String>,


    /// Plot the numbers in terminal, in log10 scale
    #[arg(short, long)]
    plot: bool,
    
    /// Benchmark how many numbers can be generated in n seconds. The original N should be zero
    #[arg(short, long)]
    bench: Option<f32>
}

fn f(v: &VecDeque<BigInt>, _n: usize) -> BigInt {
    v.iter().sum()
}


fn main() {
    let fib = Fib::parse();
    let mut func: Box<dyn Fn(&VecDeque<BigInt>, usize) -> BigInt> = Box::new(f);
    if fib.init.len() != fib.n_params {
        cprintln!("<red>error:</red> length of the init vector has to be the same as n_params!");
        return;
    }
    if fib.plot && !fib.list {
        cprintln!("<red>error:</red> only use plot with list!");
        return;
    }
    if let Some(f) = fib.func {
        func = Box::new(parse_custom_bigint(f));
    }
    if let Some(secs) = fib.bench {
        if fib.n != 0 {
            cprintln!("<red>error:</red> to use bench set n to zero!");
            return;
        }
        let (res, dur) = generate_bench(secs, fib.init, fib.n_params,  func);
        println!("Generated {} numbers in {:.3} seconds", res, dur.as_secs_f64());
        return;
    }
    if fib.list {
        let seq = generate_list(fib.n, fib.init, fib.n_params, func);
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
        } else {
            println!("{:?}", seq);
        }
    } else {

        println!("{:?}", generate(fib.n, fib.init, fib.n_params, func));
    }
}