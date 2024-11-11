#![forbid(unsafe_code)]
//! # Natural Numbers

use num::{Zero, One};

/// A natural number.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NaturalNumber {
    val: u64,
}

impl NaturalNumber {
    /// Create a new `NaturalNumber` from a `u64`.
    pub fn new(val: u64) -> Self {
        Self { val }
    }

    /// Get the value of the `NaturalNumber`.
    pub fn val(&self) -> u64 {
        self.val
    }
}

impl std::ops::Add for NaturalNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            val: self.val + other.val,
        }
    }
}

impl std::ops::Mul for NaturalNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            val: self.val * other.val,
        }
    }
}

impl Zero for NaturalNumber {
    fn zero() -> Self {
        Self { val: 0 }
    }

    fn is_zero(&self) -> bool {
        self.val == 0
    }
}

impl One for NaturalNumber {
    fn one() -> Self {
        Self { val: 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_natural_numbers() {
        let a = NaturalNumber::new(5);
        let b = NaturalNumber::new(3);
        let c = a + b;
        assert_eq!(c, NaturalNumber::new(8));
    }

    #[test]
    fn test_mul_natural_numbers() {
        let a = NaturalNumber::new(5);
        let b = NaturalNumber::new(3);
        let c = a * b;
        assert_eq!(c, NaturalNumber::new(15));
    }

    #[test]
    fn test_zero_natural_numbers() {
        let a = NaturalNumber::zero();
        assert_eq!(a, NaturalNumber::new(0));
    }

    #[test]
    fn test_is_zero_natural_numbers() {
        let a = NaturalNumber::zero();
        assert!(a.is_zero());
    }

    #[test]
    fn test_one_natural_numbers() {
        let a = NaturalNumber::one();
        assert_eq!(a, NaturalNumber::new(1));
    }
}
