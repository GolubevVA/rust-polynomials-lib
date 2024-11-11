#![forbid(unsafe_code)]
//! # Rational Numbers

use num::{Zero, One};

/// A rational number.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RationalNumber {
    numerator: i64,
    denominator: i64,
}

impl RationalNumber {
    /// Create a new `RationalNumber` from a numerator and a denominator.
    /// The result is normalized.
    /// # Examples
    /// ```
    /// use rust_polynomials_lib::coefficients::RationalNumber;
    /// let r = RationalNumber::new(2, 4);
    /// assert_eq!(r.numerator(), 1);
    /// assert_eq!(r.denominator(), 2);
    /// ```
    ///
    /// The denominator is always positive.
    pub fn new(numerator: i64, denominator: i64) -> Self {
        let mut result = Self {
            numerator,
            denominator,
        };
        result.normalize();
        result
    }

    /// Get the numerator of the `RationalNumber`.
    pub fn numerator(&self) -> i64 {
        self.numerator
    }

    /// Get the denominator of the `RationalNumber`.
    pub fn denominator(&self) -> i64 {
        self.denominator
    }

    fn normalize(&mut self) {
        let gcd = num::integer::gcd(self.numerator, self.denominator as i64);
        self.numerator /= gcd;
        self.denominator /= gcd;
        if self.denominator < 0 {
            self.numerator *= -1;
            self.denominator *= -1;
        }
    }
}

impl std::ops::Add for RationalNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Self {
            numerator: self.numerator * other.denominator + other.numerator * self.denominator,
            denominator: self.denominator * other.denominator,
        };
        result.normalize();
        result
    }
}

impl std::ops::Neg for RationalNumber {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl std::ops::Sub for RationalNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl std::ops::Mul for RationalNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Self {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        };
        result.normalize();
        result
    }
}

impl Zero for RationalNumber {
    fn zero() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }

    fn is_zero(&self) -> bool {
        self.numerator == 0
    }
}

impl One for RationalNumber {
    fn one() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_rational_numbers() {
        let a = RationalNumber::new(1, 2);
        let b = RationalNumber::new(1, 3);
        let c = a + b;
        assert_eq!(c, RationalNumber::new(5, 6));
    }

    #[test]
    fn test_sub_rational_numbers() {
        let a = RationalNumber::new(1, 2);
        let b = RationalNumber::new(1, 3);
        let c = a - b;
        assert_eq!(c, RationalNumber::new(1, 6));
    }

    #[test]
    fn test_mul_rational_numbers() {
        let a = RationalNumber::new(3, 2);
        let b = RationalNumber::new(1, 3);
        let c = a * b;
        assert_eq!(c, RationalNumber::new(1, 2));
    }

    #[test]
    fn test_zero_rational_numbers() {
        let a = RationalNumber::zero();
        assert_eq!(a, RationalNumber::new(0, 1));
    }

    #[test]
    fn test_one_rational_numbers() {
        let a = RationalNumber::one();
        assert_eq!(a, RationalNumber::new(1, 1));
    }

    #[test]
    fn test_normalize() {
        let a = RationalNumber::new(2, 4);
        assert_eq!(a, RationalNumber::new(1, 2));
    }
}
