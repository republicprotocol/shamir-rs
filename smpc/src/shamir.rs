use std::fmt::Debug;
use std::ops::{Add, Sub, Mul};
use std::iter::Sum;
use std::mem;

pub trait Field : Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Sum + Copy + PartialEq {
    fn add_inv(x: Self) -> Self {
        Self::add_identity() - x
    }
    fn mul_inv(Self) -> Self;

    fn add_identity() -> Self;
    fn mul_identity() -> Self;
    fn rand_elem() -> Self;
    fn size() -> usize;

    fn scalar_mult(Self, usize) -> Self;
    fn exp(base: Self, mut n: usize) -> Self {
        // TODO: Logarithmic implementation.
        if n == 0 {
            Self::mul_identity()
        } else {
            let start = n.leading_zeros();
            let end = (8 * mem::size_of::<usize>()) as u32;
            n = n.rotate_left(start);
            (start..end).fold(Self::mul_identity(), |acc, _| {
                n = n.rotate_left(1);
                if n % 2 == 1 {
                    acc * acc * base
                } else {
                    acc * acc
                }
            })
        }
        
        
        // if n == 0 {
        //     Self::mul_identity()
        // } else {
        //     (0..n-1).fold(base, |acc, _| acc*base)
        // }
    }
}

pub enum Error {
    NotAnElement,
    IncorrectShareCount,
}

pub fn split<T>(secret: T, k: usize, n: usize) -> Result<Vec<(T, T)>, Error>
    where T: Field + Debug {
    if n > T::size() {
        return Err(Error::NotAnElement);
    }
    
    let coefficients = (0..k).map(|i|
        if i == 0 {
            secret
        } else {
            T::rand_elem()
        }).collect::<Vec<_>>();

    let shares = (0..n).map(|i| {
        let x = T::scalar_mult(T::mul_identity(), i + 1);
        (x,
        (0..k)
            .map(|n| T::exp(x, n))
            .zip(coefficients.iter())
            .fold(T::add_identity(), |acc, (x, c)| acc + (*c)*x))
    });

    Ok(shares.collect())
}

pub fn join<T>(shares: &[(T, T)], k: usize) -> Result<T, Error>
    where T: Field {
    if shares.len() != k {
        return Err(Error::IncorrectShareCount);
    }

    let indices = shares.iter().map(|(x, _)| x);

    let secret = shares.iter().map(|(x, v)| {
        indices.clone().fold(T::mul_identity(), |acc, xj|
            if xj == x {
                acc
            } else {
                acc * (*xj) * T::mul_inv((*x) - (*xj))
            })
            * (*v)
    }).sum();

    Ok(secret)
}