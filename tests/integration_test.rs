#![forbid(unsafe_code)]

use rust_polynomials_lib::polynomials::Polynomial;
use rust_polynomials_lib::coefficients::NaturalNumber;
use rust_polynomials_lib::coefficients::RationalNumber;
use rust_polynomials_lib::coefficients::SquareMatrix;

#[test]
fn test_polynomial_of_natural_numbers_success() {
    let p = Polynomial::from_vector(vec![NaturalNumber::new(1), NaturalNumber::new(2), NaturalNumber::new(3), NaturalNumber::new(0)]);
    assert_eq!(p, Polynomial::from_vector(vec![NaturalNumber::new(1), NaturalNumber::new(2), NaturalNumber::new(3)]));

    let x = Polynomial::<NaturalNumber>::x();
    assert_eq!(x, Polynomial::from_vector(vec![NaturalNumber::new(0), NaturalNumber::new(1)]));

    let x_pow_3 = Polynomial::<NaturalNumber>::x_pow(3);
    assert_eq!(x_pow_3, Polynomial::from_vector(vec![NaturalNumber::new(0), NaturalNumber::new(0), NaturalNumber::new(0), NaturalNumber::new(1)]));
}