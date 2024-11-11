#![forbid(unsafe_code)]
//! # Matrices

use num::{One, Zero};
use std::ops::{Add, Mul, Neg, Sub};

/// A square matrix.
#[derive(Clone, Debug, PartialEq)]
pub struct SquareMatrix<T, const N: usize> {
    data: [[T; N]; N],
}

impl<T, const N: usize> SquareMatrix<T, N>
where
    T: Copy,
{
    /// Creates a new `SquareMatrix` from a 2D array.
    pub fn new(data: [[T; N]; N]) -> Self {
        Self { data }
    }

    /// Returns a reference to the internal data of the matrix.
    pub fn data(&self) -> &[[T; N]; N] {
        &self.data
    }
}

impl<T, const N: usize> Add for SquareMatrix<T, N>
where
    T: Add<Output = T> + Copy + Zero,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [[T::zero(); N]; N];

        for i in 0..N {
            for j in 0..N {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        Self { data }
    }
}

impl<T, const N: usize> Sub for SquareMatrix<T, N>
where
    T: Sub<Output = T> + Copy + Zero,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [[T::zero(); N]; N];

        for i in 0..N {
            for j in 0..N {
                data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }

        Self { data }
    }
}

impl<T, const N: usize> Neg for SquareMatrix<T, N>
where
    T: Neg<Output = T> + Copy + Zero,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut data = [[T::zero(); N]; N];

        for i in 0..N {
            for j in 0..N {
                data[i][j] = -self.data[i][j];
            }
        }

        Self { data }
    }
}

impl<T, const N: usize> Mul for SquareMatrix<T, N>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [[T::zero(); N]; N];

        for i in 0..N {
            for j in 0..N {
                let mut sum = T::zero();
                for k in 0..N {
                    sum = sum + self.data[i][k] * rhs.data[k][j];
                }
                data[i][j] = sum;
            }
        }

        Self { data }
    }
}

impl<T, const N: usize> Zero for SquareMatrix<T, N>
where
    T: Zero + Copy,
{
    fn zero() -> Self {
        Self {
            data: [[T::zero(); N]; N],
        }
    }

    fn is_zero(&self) -> bool {
        self.data
            .iter()
            .all(|row| row.iter().all(|&elem| elem.is_zero()))
    }
}

impl<T, const N: usize> One for SquareMatrix<T, N>
where
    T: Zero + One + Copy,
{
    fn one() -> Self {
        let mut data = [[T::zero(); N]; N];
        for i in 0..N {
            data[i][i] = T::one();
        }
        Self { data }
    }
}

impl<T, const N: usize> Mul<T> for SquareMatrix<T, N>
where
    T: Copy + Mul<Output = T> + Zero,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = [[T::zero(); N]; N];

        for i in 0..N {
            for j in 0..N {
                data[i][j] = self.data[i][j] * rhs;
            }
        }

        Self { data }
    }
}
