#![forbid(unsafe_code)]

#[derive(Debug, PartialEq, Clone)]
pub struct RationalNumber {
    numerator: i64,
    denominator: i64,
}

impl RationalNumber {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        Self { numerator, denominator }
    }

    pub fn numerator(&self) -> i64 {
        self.numerator
    }

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