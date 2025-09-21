//! # Fib Library
//!
//! This crate provides core functionality for generating Fibonacci sequences
//! and custom numerical sequences using `BigInt`. It supports arbitrary
//! initial values, custom RPN expressions, and configurable number of
//! previous elements used for calculations.


use std::collections::VecDeque;
use num_bigint::BigInt;

/// Generates nth number using a given function.
///
/// # Arguments
/// * `n` - Index of the number to generate
/// * `init` - Initial values of the sequence
/// * `n_params` - Number of previous elements used in `func`
/// * `func` - Function that computes the next value based on previous elements
///
/// # Returns
/// A number.
pub fn generate<F>(n: i64, init: Vec<BigInt>, n_params: usize, func: F) -> BigInt
where
    F: Fn(&VecDeque<BigInt>, usize) -> BigInt,
{
    let mut dp: VecDeque<BigInt> = VecDeque::from(init.clone());
    for _ in 0..n {
        let new = func(&dp, n_params);
        dp.pop_front();
        dp.push_back(new);
    }
    dp.pop_front().unwrap().clone()
}

/// Generates a sequence of BigInt numbers using a given function.
///
/// # Arguments
/// * `n` - Number of elements to generate
/// * `init` - Initial values of the sequence
/// * `n_params` - Number of previous elements used in `func`
/// * `func` - Function that computes the next value based on previous elements
///
/// # Returns
/// A vector containing the generated sequence.
pub fn generate_list<F>(n: i64, init: Vec<BigInt>, n_params: usize, func: F) -> Vec<BigInt>
where
    F: Fn(&VecDeque<BigInt>, usize) -> BigInt,
{
    let mut dp: VecDeque<BigInt> = VecDeque::from(init.clone());
    let mut res = init;
    for _ in 0..n {
        let new = func(&dp, n_params);
        dp.pop_front();
        dp.push_back(new.clone());
        res.push(new);
    }
    res
}


/// Parses a custom expression in Reverse Polish Notation (RPN)
/// and returns a closure that computes BigInt values.
///
/// # Arguments
/// * `expr` - The expression string (e.g., "a b +")
///
/// # Returns
/// A closure implementing `Fn(&VecDeque<BigInt>, usize) -> BigInt`
pub fn parse_custom_bigint(expr: String) -> impl Fn(&VecDeque<BigInt>, usize) -> BigInt {
    let tokens: Vec<String> = expr
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();


    move |dp: &VecDeque<BigInt>, n_params: usize| {
        let mut stack: Vec<BigInt> = Vec::new();

        for token in &tokens {
            match token.as_str() {
                "+" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a + b);
                },
                "*" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a * b);
                },
                "-" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a - b);
                },
                "**" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.pow(u32::try_from(b).unwrap()));
                }
                other => {
                    if other.len() == 1 && other.chars().last().unwrap().is_alphabetic() && other.chars().last().unwrap() as u8 - b'a' < n_params as u8 {
                        stack.push(dp[(other.chars().last().unwrap() as u8 - b'a') as usize].clone());
                    } else if other.parse::<i64>().is_ok() {
                        stack.push(BigInt::from(other.parse::<i64>().unwrap()));
                    } else {
                        panic!("invalid token {}", other);
                    }
                }
            }
        }

        stack.pop().unwrap()
    }
}