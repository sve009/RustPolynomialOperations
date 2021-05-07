extern crate rand;

use polynomial_operations::polynomials::*;
use polynomial_operations::operations::*;

use rug::Rational;
use rand::prelude::*;

#[test]
fn monom_add() {
    let mut rng = thread_rng();

    // Randomly generate
    for _ in (0..500) {
        let c1: i64 = rng.gen();
        let c2: i64 = rng.gen();

        let n: u16 = rng.gen();

        let d1: Vec<u16> = vec![rng.gen(); n.into()];
        let d2: Vec<u16> = vec![rng.gen(); n.into()];

        let m1 = Monomial { coefficient: Rational::from(c1), degree: d1.clone() };
        let m2 = Monomial { coefficient: Rational::from(c2), degree: d2.clone() };

        let p1 = Polynomial { length: 0, terms: vec![m1.clone()] };
        let p2 = Polynomial { length: 0, terms: vec![m2.clone()] };

        let p3 = add_polys(&p1, &p2);

        if deg_eq(&d1, &d2) {
            assert!(p3.terms.len() == 1);
            assert!(p3.terms[0].coefficient == c1 + c2);
            assert!(deg_eq(&p3.terms[0].degree, &d1));
        } else {
            assert!(p3.terms.len() == 2);
            assert!(p3.terms[0] == m1 || p3.terms[0] == m2);
            assert!(p3.terms[1] == m1 || p3.terms[1] == m2);
        }
    }
}

#[test]
fn monom_add_zero() {
    let mut rng = thread_rng();
    for _ in (0..500) {
        let c1: i64 = rng.gen();

        let n: u16 = rng.gen();

        let d1: Vec<u16> = vec![rng.gen(); n.into()];
        let d2: Vec<u16> = vec![0; n.into()];

        let m1 = Monomial { coefficient: Rational::from(c1), degree: d1.clone() };
        let m2 = Monomial { coefficient: Rational::from(0), degree: d2.clone() };

        let p1 = Polynomial { length: 0, terms: vec![m1.clone()] };
        let p2 = Polynomial { length: 0, terms: vec![m2.clone()] };

        let p3 = add_polys(&p1, &p2);

        assert!(p1 == p3);
    }
}

#[test]
fn monom_add_inverse() {
    let mut rng = thread_rng();
    for _ in (0..500) {
        let c1: i64 = rng.gen();

        let n: u16 = rng.gen_range(1..=100);

        let d1: Vec<u16> = vec![rng.gen(); n.into()];
        let d2: Vec<u16> = d1.clone();

        let m1 = Monomial { coefficient: Rational::from(c1), degree: d1.clone() };
        let m2 = Monomial { coefficient: Rational::from(-1 * c1), degree: d2.clone() };

        let p1 = Polynomial { length: 0, terms: vec![m1.clone()] };
        let p2 = Polynomial { length: 0, terms: vec![m2.clone()] };

        let p3 = add_polys(&p1, &p2);

        println!("Finished adding a thing: {}", p3.to_string());

        assert!(p3.terms.len() == 0);
    }
}

#[test]
fn monom_mult() {
    let mut rng = thread_rng();
    for _ in (0..500) {
        let c1: i32 = rng.gen();
        let c2: i32 = rng.gen();

        let n: u16 = rng.gen();

        let d1: Vec<u16> = vec![rng.gen::<u8>().into(); n.into()];
        let d2: Vec<u16> = vec![rng.gen::<u8>().into(); n.into()];

        let d3: Vec<u16> = d1.iter().zip(&d2).map(|(x, y)| x + y).collect();

        let m1 = Monomial { coefficient: Rational::from(c1), degree: d1.clone() };
        let m2 = Monomial { coefficient: Rational::from(c2), degree: d2.clone() };

        let p1 = Polynomial { length: 0, terms: vec![m1.clone()] };
        let p2 = Polynomial { length: 0, terms: vec![m2.clone()] };

        let p3 = mult_polys(&p1, &p2);

        let c1: i64 = c1.into();
        let c2: i64 = c2.into();

        assert!(p3.terms.len() == 1);
        assert!(p3.terms[0].coefficient == (c1 * c2));
        assert!(deg_eq(&p3.terms[0].degree, &d3));
    }
}

#[test]
fn handpicked_mult() {
    let p1 = Polynomial::from_string("2x^3y^2 + 1x^1y^0 + 3x^0y^3").unwrap();
    let p2 = Polynomial::from_string("4x^2y^0 + 1x^0y^2").unwrap();

    let p3 = mult_polys(&p1, &p2);
    let p4 = Polynomial::from_string("8x^5y^2 + 2x^3y^4 + 4x^3y^0 + 12x^2y^3 + 1x^1y^2 + 3x^0y^5").unwrap();

    assert!(p3 == p4);
}

#[test]
fn monom_divide() {
    let m1 = Monomial { coefficient: Rational::from(4), degree: vec![3, 1] };
    let m2 = Monomial { coefficient: Rational::from(2), degree: vec![1, 1] };
    let m3 = Monomial { coefficient: Rational::from(6), degree: vec![2, 3] };
    let m4 = Monomial { coefficient: Rational::from(1), degree: vec![1, 0] };
    let m5 = Monomial { coefficient: Rational::from(1), degree: vec![0, 1] };

    let r1 = Monomial { coefficient: Rational::from(2), degree: vec![2, 0] };
    let r2 = Monomial { coefficient: Rational::from(3), degree: vec![1, 2] };
    let r3 = Monomial { coefficient: Rational::from(4), degree: vec![2, 1] };
    let r4 = Monomial { coefficient: Rational::from(4), degree: vec![3, 0] };

    assert!(divide_monoms(&m1, &m2) == r1);
    assert!(divide_monoms(&m3, &m2) == r2);
    assert!(divide_monoms(&m1, &m4) == r3);
    assert!(divide_monoms(&m1, &m5) == r4);
}

#[test]
fn divides_test() {
    let p1 = Polynomial::from_monom(Monomial { coefficient: Rational::from(4), degree: vec![3, 1] });
    let p2 = Polynomial::from_monom(Monomial { coefficient: Rational::from(2), degree: vec![1, 1] });
    let p3 = Polynomial::from_monom(Monomial { coefficient: Rational::from(6), degree: vec![2, 3] });
    let p4 = Polynomial::from_monom(Monomial { coefficient: Rational::from(1), degree: vec![1, 0] });
    let p5 = Polynomial::from_monom(Monomial { coefficient: Rational::from(1), degree: vec![0, 1] });

    let p6 = Polynomial::from_string("1x^1y^0 + 2x^0y^2").unwrap();
    let p7 = Polynomial::from_string("1x^0y^2").unwrap();

    assert!(poly_divides(&p2, &p1));
    assert!(poly_divides(&p4, &p1));
    assert!(poly_divides(&p4, &p2));
    assert!(poly_divides(&p4, &p3)); 
    assert!(poly_divides(&p4, &p4)); 
    assert!(!poly_divides(&p4, &p5)); 
    assert!(poly_divides(&p2, &p3));
    assert!(!poly_divides(&p3, &p1)); 
    assert!(poly_divides(&p7, &p6));
}

#[test]
fn handpicked_poly_divides() {
    let p1 = "2x^3 + 5x^1 + 3x^0";
    let p2 = "3x^1 + 2x^0";

    let m1 = Monomial {
        coefficient: Rational::from((2, 3)),
        degree: vec![2],
    };

    let m2 = Monomial {
        coefficient: Rational::from((-4, 9)),
        degree: vec![1],
    };

    let m3 = Monomial {
        coefficient: Rational::from((53, 27)),
        degree: vec![0],
    };

    let m4 = Monomial {
        coefficient: Rational::from((-25, 27)),
        degree: vec![0],
    };

    let q = Polynomial {
        length: 3,
        terms: vec![m1, m2, m3],
    };

    let r = Polynomial::from_monom(m4);

    assert!(divide_polys(&Polynomial::from_string(p1).unwrap(), &Polynomial::from_string(p2).unwrap()) == (q, r));

    let p1 = Polynomial::from_string("1x^3y^1 + 2x^2y^3 + 3x^1y^1").unwrap();
    let p2 = Polynomial::from_string("1x^1y^1").unwrap();

    let q = Polynomial::from_string("1x^2y^0 + 2x^1y^2 + 3x^0y^0").unwrap();
    let r = Polynomial { length: 0, terms: Vec::new() };

    let (q1, r1) = divide_polys(&p1, &p2);

    assert!(q == q1);
    assert!(r == r1);

    let p6 = Polynomial::from_string("1x^1y^0 + 2x^0y^2").unwrap();
    let p7 = Polynomial::from_string("1x^0y^2").unwrap();

    let (q, r) = divide_polys(&p6, &p7);

    let q1 = Polynomial::from_string("2x^0y^0").unwrap();
    let r1 = Polynomial::from_string("1x^1y^0").unwrap();

    assert!(q == q1);
    assert!(r == r1);
}

#[test]
pub fn handpicked_div_poly_set() {
    let p1 = Polynomial::from_string("1x^2y^0 + 1x^0y^2 + 1x^0y^0").unwrap();
    let p2 = Polynomial::from_string("1x^2y^0").unwrap();
    let p3 = Polynomial::from_string("1x^0y^2").unwrap();

    let q = Polynomial::from_string("1x^0y^0").unwrap();

    let (qs, r) = divide_poly_set(&p1, &PolySet(vec![p2, p3]));

    assert!(r == q);
    assert!(qs == PolySet(vec![q.clone(), q]));

    let p1 = Polynomial::from_string("5x^1y^3 + 3x^1y^2 + 2x^0y^1").unwrap();
    let p2 = Polynomial::from_string("1x^0y^2 + 2x^0y^0").unwrap();
    let p3 = Polynomial::from_string("1x^1y^0 + 4x^0y^0").unwrap();

    let (qs, r) = divide_poly_set(&p1, &PolySet(vec![p2, p3]));

    let p4 = Polynomial::from_string("5x^1y^1 + 3x^1y^0").unwrap();
    let p5 = Polynomial::from_string("-10x^0y^1 + -6x^0y^0").unwrap();
    let p6 = Polynomial::from_string("42x^0y^1 + 24x^0y^0").unwrap();

    assert!(qs == PolySet(vec![p4, p5]));
    assert!(r == p6);
}



