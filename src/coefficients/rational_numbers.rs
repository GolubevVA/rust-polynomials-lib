#![forbid(unsafe_code)]
//! # Rational Numbers

/// A rational number.
#[derive(Debug, PartialEq, Clone)]
pub struct RationalNumber {
    numerator: i64,
    denominator: i64,
}

impl RationalNumber {
    /// Create a new `RationalNumber` from a numerator and a denominator.
    /// The result is normalized.
    /// # Examples
    /// ```
    /// use math::coefficients::rational_numbers::RationalNumber;
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

impl num::Zero for RationalNumber {
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

impl num::One for RationalNumber {
    fn one() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
        }
    }
}