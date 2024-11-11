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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_matrices() {
        let a = SquareMatrix::new([[1, 2], [3, 4]]);
        let b = SquareMatrix::new([[5, 6], [7, 8]]);
        let c = a + b;
        assert_eq!(c, SquareMatrix::new([[6, 8], [10, 12]]));
    }

    #[test]
    fn test_sub_matrices() {
        let a = SquareMatrix::new([[1, 2], [3, 4]]);
        let b = SquareMatrix::new([[5, 6], [7, 8]]);
        let c = a - b;
        assert_eq!(c, SquareMatrix::new([[-4, -4], [-4, -4]]));
    }

    #[test]
    fn test_neg_matrices() {
        let a = SquareMatrix::new([[1, 2], [3, 4]]);
        let b = -a;
        assert_eq!(b, SquareMatrix::new([[-1, -2], [-3, -4]]));
    }

    #[test]
    fn test_mul_matrices() {
        let a = SquareMatrix::new([[1, 2], [3, 4]]);
        let b = SquareMatrix::new([[5, 6], [7, 8]]);
        let c = a * b;
        assert_eq!(c, SquareMatrix::new([[19, 22], [43, 50]]));
    }

    #[test]
    fn test_zero_matrices() {
        let a = SquareMatrix::<i32, 2>::zero();
        assert_eq!(a, SquareMatrix::new([[0, 0], [0, 0]]));
    }

    #[test]
    fn test_one_matrices() {
        let a = SquareMatrix::<i32, 2>::one();
        assert_eq!(a, SquareMatrix::new([[1, 0], [0, 1]]));
    }

    #[test]
    fn test_mul_scalar_matrices() {
        let a = SquareMatrix::new([[1, 2], [3, 4]]);
        let b = a * 2;
        assert_eq!(b, SquareMatrix::new([[2, 4], [6, 8]]));
    }

    #[test]
    fn test_is_zero_matrices() {
        let a = SquareMatrix::new([[0, 0], [0, 0]]);
        assert!(a.is_zero());
    }

    #[test]
    fn test_data_matrices() {
        let a = SquareMatrix::new([[1, 2], [3, 4]]);
        assert_eq!(a.data(), &[[1, 2], [3, 4]]);
    }

    #[test]
    fn test_add_matrices_3x3() {
        let a = SquareMatrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let b = SquareMatrix::new([[9, 8, 7], [6, 5, 4], [3, 2, 1]]);
        let c = a + b;
        assert_eq!(
            c,
            SquareMatrix::new([[10, 10, 10], [10, 10, 10], [10, 10, 10]])
        );
    }
}
