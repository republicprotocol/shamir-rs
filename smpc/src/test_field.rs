extern crate rand;

use std::ops::{Add, Sub, Mul};
use std::iter::Sum;
use shamir::Field;

const P: u32 = 4294967291;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TestField {
    pub x: u32,
}

impl Add for TestField {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let x = ((self.x as u64 + other.x as u64) % P as u64) as u32;
        TestField { x }
    }
}

impl Sub for TestField {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let x = (self.x as i64 - other.x as i64) % (P as i64);
        if x < 0 {
            TestField { x: (x + P as i64) as u32 }
        } else {
            TestField { x: x as u32 }
        }
    }
}

impl Mul for TestField {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let x = (self.x as u64 * other.x as u64 % P as u64) as u32;
        TestField { x }
    }
}

impl Sum for TestField {
    fn sum<I>(iter: I) -> Self
        where I: Iterator<Item=TestField> {
            iter.fold(Self::add_identity(), |acc, x| acc + x)
    }
}

impl Field for TestField {
    fn mul_inv(x: Self) -> Self {
        // Zero has no modular inverse.
        assert_ne!(x, Self::add_identity());
        
        let (mut r0, mut r1) = (x.x, P);
        let (mut s0, mut s1) = (1, 0);
        let mut r2 : u32;
        let mut s2: i64;
        let mut q: i64;

        while r1 != 0 {
            r2 = r0 % r1;
            q = ((r0 - r2)/r1) as i64;
            s2 = s0 - q*s1;

            r0 = r1;
            r1 = r2;
            s0 = s1;
            s1 = s2;
        }

        // r0 is the GCD, and for prime P this has to be 1.
        assert_eq!(r0, Self::mul_identity().x);

        s0 = s0 % P as i64;
        if s0 < 0 {
            s0 = s0 + P as i64;
        }

        TestField { x: s0 as u32 }
    }
    
    fn add_identity() -> Self { TestField { x: 0 } }
    fn mul_identity() -> Self { TestField { x: 1 } }

    fn rand_elem() -> Self {
        TestField { x: rand::random::<u32>() }
    }
    fn size() -> usize { P as usize }

    fn scalar_mult(x: Self, n: usize) -> Self {
        let y = TestField { x: (n % P as usize) as u32};
        x * y
    }
}