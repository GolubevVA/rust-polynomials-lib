#![forbid(unsafe_code)]

use std::ops::Mul;

use num::{One, Zero};

#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial<T>
where
    T: One + Zero + Clone,
{
    coefficients: Vec<T>,
}

impl<T> Polynomial<T>
where
    T: Clone + Zero + One,
{
    pub fn from_constant(c: T) -> Self {
        Self {
            coefficients: vec![c],
        }
    }

    pub fn from_vector(coefficients: Vec<T>) -> Self {
        let mut result = Self { coefficients };
        result.normalize();
        result
    }

    pub fn x() -> Self {
        Self {
            coefficients: vec![T::zero(), T::one()],
        }
    }

    pub fn x_pow(n: usize) -> Self {
        let mut coefficients = vec![T::zero(); n + 1];
        coefficients[n] = T::one();
        Self { coefficients }
    }

    pub fn normalize(&mut self) {
        while let Some(c) = self.coefficients.last() {
            if self.coefficients.len() == 1 || !c.is_zero() {
                break;
            }
            self.coefficients.pop();
        }
    }
}

impl<T> Polynomial<T>
where
    T: One + Zero + Clone + Mul<Output = T> + std::ops::Add<Output = T>,
    Polynomial<T>: Mul<Output = Polynomial<T>>,
{
    pub fn eval(&self, x: Polynomial<T>) -> Polynomial<T> {
        let p = self.clone();
        let mut result = Polynomial::zero();
        for c in p.coefficients.iter().rev() {
            result = result * x.clone() + Polynomial::from_constant(c.clone());
        }
        result
    }
}

impl<T> Zero for Polynomial<T>
where
    T: Zero + Clone + One,
{
    fn zero() -> Self {
        Polynomial {
            coefficients: vec![T::zero()],
        }
    }

    fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|c| c.is_zero())
    }
}

impl<T> One for Polynomial<T>
where
    T: Zero + Clone + One,
    Polynomial<T>: Mul<Output = Polynomial<T>>,
{
    fn one() -> Self {
        Polynomial {
            coefficients: vec![T::one()],
        }
    }
}

impl<T> std::ops::Neg for Polynomial<T>
where
    T: std::ops::Neg<Output = T> + One + Zero + Clone,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            coefficients: self.coefficients.into_iter().map(|c| c.neg()).collect(),
        }
    }
}

impl<T> std::ops::Add<T> for Polynomial<T>
where
    T: One + Zero + Clone + std::ops::Add<Output = T>,
{
    type Output = Self;

    /// adding rhs constant to the constant term of the polynomial
    fn add(self, rhs: T) -> Self {
        let mut coefficients = self.coefficients;
        coefficients[0] = coefficients[0].clone() + rhs;
        Self { coefficients }
    }
}

impl<T> std::ops::Sub<T> for Polynomial<T>
where
    T: One + Zero + Clone + std::ops::Sub<Output = T>,
{
    type Output = Self;

    /// subtracting rhs constant from the constant term of the polynomial
    fn sub(self, rhs: T) -> Self {
        let mut coefficients = self.coefficients;
        coefficients[0] = coefficients[0].clone() - rhs;
        Self::from_vector(coefficients)
    }
}

impl<T> std::ops::Add for Polynomial<T>
where
    T: One + Zero + Clone + std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut coefficients = vec![T::zero(); self.coefficients.len().max(rhs.coefficients.len())];
        for i in 0..coefficients.len() {
            coefficients[i] = self.coefficients.get(i).cloned().unwrap_or(T::zero())
                + rhs.coefficients.get(i).cloned().unwrap_or(T::zero());
        }
        Self::from_vector(coefficients)
    }
}

impl<T> std::ops::Sub for Polynomial<T>
where
    T: One + Zero + Clone + std::ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut coefficients = vec![T::zero(); self.coefficients.len().max(rhs.coefficients.len())];
        for i in 0..coefficients.len() {
            coefficients[i] = self.coefficients.get(i).cloned().unwrap_or(T::zero())
                - rhs.coefficients.get(i).cloned().unwrap_or(T::zero());
        }
        Self::from_vector(coefficients)
    }
}

impl<T> Mul<T> for Polynomial<T>
where
    T: One + Zero + Clone + Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::from_vector(
            self.coefficients
                .into_iter()
                .map(|c| c * rhs.clone())
                .collect(),
        )
    }
}

impl<T> Mul for Polynomial<T>
where
    T: One + Zero + Clone + Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut coefficients =
            vec![T::zero(); self.coefficients.len() + rhs.coefficients.len() - 1];
        for i in 0..self.coefficients.len() {
            for j in 0..rhs.coefficients.len() {
                coefficients[i + j] = coefficients[i + j].clone()
                    + self.coefficients[i].clone() * rhs.coefficients[j].clone();
            }
        }
        Self::from_vector(coefficients)
    }
}

impl<T> std::ops::BitXor<usize> for Polynomial<T>
where
    T: One + Zero + Clone + std::ops::Add<Output = T> + Mul<Output = T>,
    Polynomial<T>: One,
{
    type Output = Self;

    fn bitxor(self, rhs: usize) -> Self::Output {
        let mut result = Polynomial::one();

        let mut a = self.clone();
        let mut exp = rhs;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * a.clone();
            }
            a = a.clone() * a.clone();
            exp /= 2;
        }

        result
    }
}
