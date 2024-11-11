#![forbid(unsafe_code)]
//! # Polynomials

use std::ops::Mul;

use num::{One, Zero};

/// A polynomial.
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
    /// Create a new `Polynomial` from a constant.
    /// The constant becomes the constant term of the polynomial.
    pub fn from_constant(c: T) -> Self {
        Self {
            coefficients: vec![c],
        }
    }

    /// Create a new `Polynomial` from a vector of coefficients.
    /// The Polynomial is normalized after creation.
    /// # Examples
    /// ```
    /// use rust_polynomials_lib::polynomials::Polynomial;
    /// let p = Polynomial::from_vector(vec![1, 2, 3, 0]); // 1 + 2x + 3x^2
    /// assert_eq!(p, Polynomial::from_vector(vec![1, 2, 3]));
    /// ```
    pub fn from_vector(coefficients: Vec<T>) -> Self {
        let mut result = Self { coefficients };
        result.normalize();
        result
    }

    /// Create a new `Polynomial` representing the `x` variable.
    /// # Examples
    /// ```
    /// use rust_polynomials_lib::polynomials::Polynomial;
    /// let p = Polynomial::<i32>::x(); // x
    /// ```
    pub fn x() -> Self {
        Self {
            coefficients: vec![T::zero(), T::one()],
        }
    }

    /// Create a new `Polynomial` representing the `x^n` variable.
    /// # Examples
    /// ```
    /// use rust_polynomials_lib::polynomials::Polynomial;
    /// let p = Polynomial::<i32>::x_pow(3); // x^3
    /// ```
    pub fn x_pow(n: usize) -> Self {
        let mut coefficients = vec![T::zero(); n + 1];
        coefficients[n] = T::one();
        Self { coefficients }
    }

    /// Normalizes the polynomial by removing trailing zero coefficients.
    /// # Examples
    /// ```
    /// use rust_polynomials_lib::polynomials::Polynomial;
    /// let mut p = Polynomial::from_vector(vec![1, 2, 3, 0]); // 1 + 2x + 3x^2
    /// p.normalize(); // ensuring p to be 1 + 2x + 3x^2
    /// ```
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
    T: One + Zero + Clone,
{
    /// Evaluate the polynomial at a given value.
    /// The value is substituted for `x` in the polynomial so that the coefficients are multiplied from the right.
    /// Note, that this function assumes that multiplying the `One`` element of the `x`'s type with a coefficient is what the constant term of the polynomial is.
    /// # Examples
    /// ```
    /// use rust_polynomials_lib::polynomials::Polynomial;
    /// let p = Polynomial::from_vector(vec![1, 2, 3]); // 1 + 2x + 3x^2
    /// let res = p.eval(2); // 1 + 2*2 + 3*2^2 = 1 + 4 + 12 = 17
    /// assert_eq!(res, 17);
    /// ```
    pub fn eval<U>(&self, x: U) -> U
    where
        U: Clone + Zero + std::ops::Add<Output = U> + Mul<Output = U> + One + Mul<T, Output = U>,
    {
        let mut result = U::zero();
        for coeff in self.coefficients.iter().rev() {
            result = result * x.clone() + U::one() * coeff.clone();
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
    T: One + Zero + Clone + Mul<Output = T> + std::ops::Add<Output = T>,
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

/// Exponentiation of a polynomial.
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
