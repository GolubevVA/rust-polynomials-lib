#![forbid(unsafe_code)]

use num::{One, Zero};
use rust_polynomials_lib::coefficients::NaturalNumber;
use rust_polynomials_lib::coefficients::RationalNumber;
use rust_polynomials_lib::coefficients::SquareMatrix;
use rust_polynomials_lib::polynomials::Polynomial;

#[test]
fn test_polynomial_of_natural_numbers() {
    let p = Polynomial::from_vector(vec![
        NaturalNumber::new(1),
        NaturalNumber::new(2),
        NaturalNumber::new(3),
        NaturalNumber::new(0),
    ]);
    assert_eq!(
        p,
        Polynomial::from_vector(vec![
            NaturalNumber::new(1),
            NaturalNumber::new(2),
            NaturalNumber::new(3)
        ])
    );

    let x = Polynomial::<NaturalNumber>::x();
    assert_eq!(
        x,
        Polynomial::from_vector(vec![NaturalNumber::new(0), NaturalNumber::new(1)])
    );

    let x_pow_3 = Polynomial::<NaturalNumber>::x_pow(3);
    assert_eq!(
        x_pow_3,
        Polynomial::from_vector(vec![
            NaturalNumber::new(0),
            NaturalNumber::new(0),
            NaturalNumber::new(0),
            NaturalNumber::new(1)
        ])
    );

    let const_polynom = Polynomial::<NaturalNumber>::from_constant(NaturalNumber::new(5));
    assert_eq!(
        const_polynom,
        Polynomial::from_vector(vec![NaturalNumber::new(5)])
    );

    let p = x_pow_3.clone() + x.clone() * NaturalNumber::new(2) + const_polynom.clone();
    assert_eq!(
        p,
        Polynomial::from_vector(vec![
            NaturalNumber::new(5),
            NaturalNumber::new(2),
            NaturalNumber::new(0),
            NaturalNumber::new(1)
        ])
    );

    let res = p.eval(NaturalNumber::new(2));
    assert_eq!(res, NaturalNumber::new(17));
}

#[test]
fn test_matrices_with_custom_coefficients() {
    let m1 = SquareMatrix::new([
        [RationalNumber::new(1, 2), RationalNumber::new(1, 3)],
        [RationalNumber::new(1, 4), RationalNumber::new(1, 5)],
    ]);
    let m2 = SquareMatrix::new([
        [RationalNumber::new(1, 6), RationalNumber::new(1, 7)],
        [RationalNumber::new(1, 8), RationalNumber::new(1, 9)],
    ]);
    let m3 = m1.clone() + m2.clone();
    assert_eq!(
        m3,
        SquareMatrix::new([
            [RationalNumber::new(2, 3), RationalNumber::new(10, 21)],
            [RationalNumber::new(3, 8), RationalNumber::new(14, 45)]
        ])
    );
    let m4 = m1.clone() * m2.clone();
    assert_eq!(
        m4,
        SquareMatrix::new([
            [RationalNumber::new(1, 8), RationalNumber::new(41, 378)],
            [RationalNumber::new(1, 15), RationalNumber::new(73, 1260)]
        ])
    );
    let m4 = m1.clone() - m2.clone();
    assert_eq!(
        m4,
        SquareMatrix::new([
            [RationalNumber::new(1, 3), RationalNumber::new(4, 21)],
            [RationalNumber::new(1, 8), RationalNumber::new(4, 45)]
        ])
    );

    let m5 = SquareMatrix::<RationalNumber, 2>::zero();
    assert_eq!(
        m5,
        SquareMatrix::new([
            [RationalNumber::zero(), RationalNumber::zero()],
            [RationalNumber::zero(), RationalNumber::zero()]
        ])
    );

    let m6 = SquareMatrix::<RationalNumber, 2>::one();
    assert_eq!(
        m6,
        SquareMatrix::new([
            [RationalNumber::one(), RationalNumber::zero()],
            [RationalNumber::zero(), RationalNumber::one()]
        ])
    );
}

#[test]
fn test_polynomial_of_matrices() {
    let p = Polynomial::from_vector(vec![
        SquareMatrix::new([
            [NaturalNumber::new(1), NaturalNumber::new(2)],
            [NaturalNumber::new(3), NaturalNumber::new(4)],
        ]),
        SquareMatrix::new([
            [NaturalNumber::new(5), NaturalNumber::new(6)],
            [NaturalNumber::new(7), NaturalNumber::new(8)],
        ]),
        SquareMatrix::new([
            [NaturalNumber::new(9), NaturalNumber::new(10)],
            [NaturalNumber::new(11), NaturalNumber::new(12)],
        ]),
        SquareMatrix::new([
            [NaturalNumber::new(0), NaturalNumber::new(0)],
            [NaturalNumber::new(0), NaturalNumber::new(0)],
        ]),
    ]);
    assert_eq!(
        p,
        Polynomial::from_vector(vec![
            SquareMatrix::new([
                [NaturalNumber::new(1), NaturalNumber::new(2)],
                [NaturalNumber::new(3), NaturalNumber::new(4)]
            ]),
            SquareMatrix::new([
                [NaturalNumber::new(5), NaturalNumber::new(6)],
                [NaturalNumber::new(7), NaturalNumber::new(8)]
            ]),
            SquareMatrix::new([
                [NaturalNumber::new(9), NaturalNumber::new(10)],
                [NaturalNumber::new(11), NaturalNumber::new(12)]
            ])
        ])
    );

    let x = Polynomial::<SquareMatrix<NaturalNumber, 2>>::x();
    assert_eq!(
        x,
        Polynomial::from_vector(vec![
            SquareMatrix::new([
                [NaturalNumber::new(0), NaturalNumber::new(0)],
                [NaturalNumber::new(0), NaturalNumber::new(0)]
            ]),
            SquareMatrix::new([
                [NaturalNumber::new(1), NaturalNumber::new(0)],
                [NaturalNumber::new(0), NaturalNumber::new(1)]
            ])
        ])
    );

    let x_pow_3 = Polynomial::<SquareMatrix<NaturalNumber, 2>>::x_pow(3);
    assert_eq!(
        x_pow_3,
        Polynomial::from_vector(vec![
            SquareMatrix::new([
                [NaturalNumber::new(0), NaturalNumber::new(0)],
                [NaturalNumber::new(0), NaturalNumber::new(0)]
            ]),
            SquareMatrix::new([
                [NaturalNumber::new(0), NaturalNumber::new(0)],
                [NaturalNumber::new(0), NaturalNumber::new(0)]
            ]),
            SquareMatrix::new([
                [NaturalNumber::new(0), NaturalNumber::new(0)],
                [NaturalNumber::new(0), NaturalNumber::new(0)]
            ]),
            SquareMatrix::new([
                [NaturalNumber::new(1), NaturalNumber::new(0)],
                [NaturalNumber::new(0), NaturalNumber::new(1)]
            ])
        ])
    );

    let const_polynom =
        Polynomial::<SquareMatrix<NaturalNumber, 2>>::from_constant(SquareMatrix::new([
            [NaturalNumber::new(5), NaturalNumber::new(6)],
            [NaturalNumber::new(7), NaturalNumber::new(8)],
        ]));
    assert_eq!(
        const_polynom,
        Polynomial::from_vector(vec![SquareMatrix::new([
            [NaturalNumber::new(5), NaturalNumber::new(6)],
            [NaturalNumber::new(7), NaturalNumber::new(8)]
        ])])
    );
}
