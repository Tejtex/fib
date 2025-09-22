//! # Fib Library
//!
//! This crate provides core functionality for generating Fibonacci sequences
//! and custom numerical sequences using `BigInt`. It supports arbitrary
//! initial values, custom RPN expressions, and configurable number of
//! previous elements used for calculations.


use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use num_bigint::BigInt;
use num_traits::{One, Zero};

/// Generates nth number using a given function.
///
/// # Arguments
/// * `n` - Index of the number to generate
/// * `init` - Initial values of the sequence
/// * `n_params` - Number of previous elements
/// * `coeffs` - Coefficient of each param
/// * `mod_x` - Uses the given modulo
///
/// # Returns
/// A number.
pub fn generate( n: u128, init: Vec<BigInt>, n_params: usize, coeffs: &[i64], mod_x: Option<u64>) -> BigInt
{

    let n = n - 1;
    let m = build_matrix(coeffs);
    let m_pow = mat_pow(m, n, mod_x.map(|e| BigInt::from(e)));
    let mut result = BigInt::zero();
    for i in 0..n_params {
        result += &m_pow[0][i] * &init[n_params - i - 1];
        if let Some(m) = mod_x {
            result %= m;
        }
    }
    result

}

/// Generates a sequence of BigInt numbers using a given function.
///
/// # Arguments
/// * `n` - Number of elements to generate
/// * `init` - Initial values of the sequence
/// * `coeffs` - Coefficient of each param
/// * `mod_x` - Uses the given modulo
///
/// # Returns
/// A vector containing the generated sequence.
pub fn generate_list(n: u128, init: Vec<BigInt>, _n_params: usize, coeffs: &[i64], mod_x: Option<u64>) -> Vec<BigInt>
{

    let k = coeffs.len();
    let mut seq: Vec<BigInt> = init.clone().into_iter().map(|e| BigInt::from(e)).collect();

    for i in k..=n as usize {
        let mut next = BigInt::zero();
        for j in 0..k {
            next += coeffs[j] * &seq[i - j - 1];
        }
        if let Some(m) = mod_x {
            next %= m;
        }
        seq.push(next);
    }
    seq
}


/// Log10 of bigint.
///
/// # Arguments
/// * `n` - a big int number
///
/// # Returns
/// log10 of the given number
pub fn log10_bigint(n: &BigInt) -> f64 {
    let digits = n.to_string();
    let len = digits.len();

    let leading: f64 = digits
        .chars()
        .take(15)
        .collect::<String>()
        .parse()
        .unwrap_or(1.0);

    let log_leading = leading.abs().log10();

    log_leading + if len > 15 {len - 15} else {0} as f64
}

/// Multiplies two square matrices `a` and `b`.
///
/// # Arguments
/// * `a` - First matrix (k x k) to multiply.
/// * `b` - Second matrix (k x k) to multiply.
/// * `mod_x` - Optional modulus; if provided, all operations are done modulo this value.
///
/// # Returns
/// A new matrix which is the result of `a * b`.
///
/// # Notes
/// - The matrices must be square and of the same size.
/// - If `mod_x` is `Some(m)`, all multiplications and additions are performed modulo `m`.
fn mat_mul(a: &Vec<Vec<BigInt>>, b: &Vec<Vec<BigInt>>, mod_x: &Option<BigInt>) -> Vec<Vec<BigInt>> {
    let k = a.len();
    let mut result = vec![vec![BigInt::zero(); k]; k];
    for i in 0..k {
        for j in 0..k {
            let mut sum = BigInt::zero();
            for l in 0..k {
                let mut prod = &a[i][l] * &b[l][j];
                if let Some(m) = mod_x {
                    prod %= m;
                }
                sum += prod;
                if let Some(m) = mod_x {
                    sum %= m;
                }
            }
            result[i][j] = sum;
        }
    }
    result
}

/// Computes the exponentiation of a square matrix `base` to the power `exp`.
///
/// # Arguments
/// * `base` - The base matrix (k x k) to exponentiate.
/// * `exp` - The exponent to raise the matrix to.
/// * `mod_x` - Optional modulus; if provided, all operations are done modulo this value.
///
/// # Returns
/// The matrix `base` raised to the power `exp`.
///
/// # Notes
/// - The matrix must be square.
/// - Uses fast exponentiation by squaring for efficiency.
/// - If `mod_x` is `Some(m)`, all intermediate operations are performed modulo `m`.
fn mat_pow(mut base: Vec<Vec<BigInt>>, mut exp: u128, mod_x: Option<BigInt>) -> Vec<Vec<BigInt>> {
    let k = base.len();
    let mut result = vec![vec![BigInt::zero(); k]; k];
    for i in 0..k {
        result[i][i] = BigInt::one();
    }

    while exp > 0 {
        if exp % 2 == 1 {
            result = mat_mul(&result, &base, &mod_x);
        }
        base = mat_mul(&base, &base, &mod_x);
        exp /= 2;
    }
    result
}

/// Creates a fibonacci matrix based on the coefficients
///
/// # Arguments
/// * `coeffs` - the coefficients
///
/// # Returns
/// Fibonacci exp matrix
///
/// # Example
///
/// build_matrix([1, 1]) = [[1, 1]
///                         [0, 1]]
fn build_matrix(coeffs: &[i64]) -> Vec<Vec<BigInt>> {
    let k = coeffs.len();
    let mut mat = vec![vec![BigInt::zero(); k]; k];

    for i in 0..k {
        mat[0][i] = BigInt::from(coeffs[i]);
    }

    for i in 1..k {
        mat[i][i - 1] = BigInt::one();
    }

    mat
}