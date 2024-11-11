#![forbid(unsafe_code)]

pub mod polynomials;

fn main() {
    let x = polynomials::polynomial::Polynomial::<i32>::x();
    let x2 = polynomials::polynomial::Polynomial::<i32>::x_pow(2);
    let p1 = x2.clone() + x.clone() + polynomials::polynomial::Polynomial::from_constant(1);
    let p2 = x.clone() + polynomials::polynomial::Polynomial::from_constant(-1);
    let p3 = p2.eval(p1.clone());
    println!("{:?}", p3);
    let p4 = p1.eval(p2.clone());
    println!("{:?}", p4);

    let p5 = p1.eval(2);
    println!("{:?}", p5);
}

// p1 = x^2 + x + 1
// p2 = x - 1
// p3 = (x^2 + x + 1) - 1 = x^2 + x
// p4 = (x - 1)^2 + (x - 1) + 1 = x^2 - 2x + 1 + x - 1 + 1 = x^2 - x + 1
