#![forbid(unsafe_code)]
//! # Natural Numbers

/// A natural number.
#[derive(Debug, PartialEq, Clone)]
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

impl num::Zero for NaturalNumber {
    fn zero() -> Self {
        Self { val: 0 }
    }

    fn is_zero(&self) -> bool {
        self.val == 0
    }
}

impl num::One for NaturalNumber {
    fn one() -> Self {
        Self { val: 1 }
    }
}
