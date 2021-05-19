extern crate rand;

use polynomial_operations::polynomials::*;
use polynomial_operations::operations::*;

use rug::Rational;
use rand::prelude::*;

use std::rc::Rc;

#[test]
fn monom_add() {
    let mut rng = thread_rng();

    let ring = Rc::new(Ring { symbols: vec![], ord: MonomialOrdering::DegLex });

    // Randomly generate
    for _ in (0..500) {
        let c1: i64 = rng.gen();
        let c2: i64 = rng.gen();

        let n: u16 = rng.gen_range(0..10);

        let d1: Vec<u16> = vec![rng.gen_range(0..1000); n.into()];
        let d2: Vec<u16> = vec![rng.gen_range(0..1000); n.into()];

        let m1 = Monomial { coefficient: Rational::from(c1), degree: d1.clone(), ring: Rc::clone(&ring) };
        let m2 = Monomial { coefficient: Rational::from(c2), degree: d2.clone(), ring: Rc::clone(&ring) };

        let p1 = Polynomial { length: 0, terms: vec![m1.clone()], ring: Rc::clone(&ring) };
        let p2 = Polynomial { length: 0, terms: vec![m2.clone()], ring: Rc::clone(&ring) };

        let p3 = add_polys(&p1, &p2);

        if deg_eq(&d1, &d2) {
            assert!(p3.terms.len() == 1);
            assert!(p3.terms[0].coefficient == Rational::from(c1) + c2);
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
    let ring = Rc::new(Ring { symbols: vec![], ord: MonomialOrdering::DegLex });
    for _ in (0..500) {
        let c1: i64 = rng.gen();

        let n: u16 = rng.gen_range(0..10);

        let d1: Vec<u16> = vec![rng.gen_range(0..1000); n.into()];
        let d2: Vec<u16> = vec![0; n.into()];

        let m1 = Monomial { coefficient: Rational::from(c1), degree: d1.clone(), ring: Rc::clone(&ring) };
        let m2 = Monomial { coefficient: Rational::from(0), degree: d2.clone(), ring: Rc::clone(&ring) };

        let p1 = Polynomial { length: 0, terms: vec![m1.clone()], ring: Rc::clone(&ring) };
        let p2 = Polynomial { length: 0, terms: vec![m2.clone()], ring: Rc::clone(&ring)};

        let p3 = add_polys(&p1, &p2);

        assert!(p1 == p3);
    }
}

#[test]
fn monom_add_inverse() {
    let mut rng = thread_rng();
    let ring = Rc::new(Ring { symbols: vec![], ord: MonomialOrdering::DegLex });
    for _ in (0..500) {
        let c1: i64 = rng.gen();

        let n: u16 = rng.gen_range(1..=10);

        let d1: Vec<u16> = vec![rng.gen_range(0..1000); n.into()];
        let d2: Vec<u16> = d1.clone();

        let m1 = Monomial { coefficient: Rational::from(c1), degree: d1.clone(), ring: Rc::clone(&ring) };
        let m2 = Monomial { coefficient: Rational::from(-1 * c1), degree: d2.clone(), ring: Rc::clone(&ring) };

        let p1 = Polynomial { length: 0, terms: vec![m1.clone()], ring: Rc::clone(&ring) };
        let p2 = Polynomial { length: 0, terms: vec![m2.clone()], ring: Rc::clone(&ring) };

        let p3 = add_polys(&p1, &p2);

        println!("Finished adding a thing: {}", p3.to_string());

        assert!(p3.terms.len() == 0);
    }
}

#[test]
fn monom_mult() {
    let mut rng = thread_rng();
    let ring = Rc::new(Ring { symbols: vec![], ord: MonomialOrdering::DegLex });
    for _ in (0..500) {
        let c1: i32 = rng.gen();
        let c2: i32 = rng.gen();

        let n: u16 = rng.gen();

        let d1: Vec<u16> = vec![rng.gen::<u8>().into(); n.into()];
        let d2: Vec<u16> = vec![rng.gen::<u8>().into(); n.into()];

        let d3: Vec<u16> = d1.iter().zip(&d2).map(|(x, y)| x + y).collect();

        let m1 = Monomial { coefficient: Rational::from(c1), degree: d1.clone(), ring: Rc::clone(&ring) };
        let m2 = Monomial { coefficient: Rational::from(c2), degree: d2.clone(), ring: Rc::clone(&ring) };

        let p1 = Polynomial { length: 0, terms: vec![m1.clone()], ring: Rc::clone(&ring) };
        let p2 = Polynomial { length: 0, terms: vec![m2.clone()], ring: Rc::clone(&ring) };

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
    let ring = Rc::new(Ring { symbols: vec!["x".to_string(), "y".to_string()], ord: MonomialOrdering::DegLex });
    let p1 = Polynomial::from_string("2x^3y^2 + 1x^1y^0 + 3x^0y^3", &ring).unwrap();
    let p2 = Polynomial::from_string("4x^2y^0 + 1x^0y^2", &ring).unwrap();

    let p3 = mult_polys(&p1, &p2);
    let p4 = Polynomial::from_string("8x^5y^2 + 2x^3y^4 + 12x^2y^3 + 3x^0y^5 + 4x^3y^0 + x^1y^2", &ring).unwrap();

    println!("p3: {}, p4: {}", p3.to_string(), p4.to_string());
    assert!(p3 == p4);
}

#[test]
fn monom_divide() {
    let ring = Rc::new(Ring { symbols: vec![], ord: MonomialOrdering::DegLex });

    let m1 = Monomial { coefficient: Rational::from(4), degree: vec![3, 1], ring: Rc::clone(&ring) };
    let m2 = Monomial { coefficient: Rational::from(2), degree: vec![1, 1], ring: Rc::clone(&ring) };
    let m3 = Monomial { coefficient: Rational::from(6), degree: vec![2, 3], ring: Rc::clone(&ring) };
    let m4 = Monomial { coefficient: Rational::from(1), degree: vec![1, 0], ring: Rc::clone(&ring) };
    let m5 = Monomial { coefficient: Rational::from(1), degree: vec![0, 1], ring: Rc::clone(&ring) };

    let r1 = Monomial { coefficient: Rational::from(2), degree: vec![2, 0], ring: Rc::clone(&ring) };
    let r2 = Monomial { coefficient: Rational::from(3), degree: vec![1, 2], ring: Rc::clone(&ring) };
    let r3 = Monomial { coefficient: Rational::from(4), degree: vec![2, 1], ring: Rc::clone(&ring) };
    let r4 = Monomial { coefficient: Rational::from(4), degree: vec![3, 0], ring: Rc::clone(&ring) };

    assert!(divide_monoms(&m1, &m2) == r1);
    assert!(divide_monoms(&m3, &m2) == r2);
    assert!(divide_monoms(&m1, &m4) == r3);
    assert!(divide_monoms(&m1, &m5) == r4);
}

#[test]
fn divides_test() {
    let ring = Rc::new(Ring { symbols: vec!["x".to_string(), "y".to_string()], ord: MonomialOrdering::DegLex });

    let p1 = Polynomial::from_monom(Monomial { coefficient: Rational::from(4), degree: vec![3, 1], ring: Rc::clone(&ring) });
    let p2 = Polynomial::from_monom(Monomial { coefficient: Rational::from(2), degree: vec![1, 1], ring: Rc::clone(&ring) });
    let p3 = Polynomial::from_monom(Monomial { coefficient: Rational::from(6), degree: vec![2, 3], ring: Rc::clone(&ring) });
    let p4 = Polynomial::from_monom(Monomial { coefficient: Rational::from(1), degree: vec![1, 0], ring: Rc::clone(&ring) });
    let p5 = Polynomial::from_monom(Monomial { coefficient: Rational::from(1), degree: vec![0, 1], ring: Rc::clone(&ring) });

    let p6 = Polynomial::from_string("1x^1y^0 + 2x^0y^2", &ring).unwrap();
    let p7 = Polynomial::from_string("1x^0y^2", &ring).unwrap();

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
    let ring = Rc::new(Ring { symbols: vec!["x".to_string(), "y".to_string()], ord: MonomialOrdering::DegLex });

    let p1 = "2x^3 + 5x^1 + 3x^0";
    let p2 = "3x^1 + 2x^0";

    let m1 = Monomial {
        coefficient: Rational::from((2, 3)),
        degree: vec![2],
        ring: Rc::clone(&ring),
    };

    let m2 = Monomial {
        coefficient: Rational::from((-4, 9)),
        degree: vec![1],
        ring: Rc::clone(&ring),
    };

    let m3 = Monomial {
        coefficient: Rational::from((53, 27)),
        degree: vec![0],
        ring: Rc::clone(&ring),
    };

    let m4 = Monomial {
        coefficient: Rational::from((-25, 27)),
        degree: vec![0],
        ring: Rc::clone(&ring),
    };

    let q = Polynomial {
        length: 3,
        terms: vec![m1, m2, m3],
        ring: Rc::clone(&ring),
    };

    let r = Polynomial::from_monom(m4);

    assert!(divide_polys(&Polynomial::from_string(p1, &ring).unwrap(), &Polynomial::from_string(p2, &ring).unwrap()) == (q, r));

    let p1 = Polynomial::from_string("2x^2y^3 + 1x^3y^1 + 3x^1y^1", &ring).unwrap();
    let p2 = Polynomial::from_string("1x^1y^1", &ring).unwrap();

    let q = Polynomial::from_string("2x^1y^2 + 1x^2y^0 + 3x^0y^0", &ring).unwrap();
    let r = Polynomial { length: 0, terms: Vec::new(), ring: Rc::clone(&ring) };

    let (q1, r1) = divide_polys(&p1, &p2);

    assert!(q == q1);
    assert!(r == r1);

    let p6 = Polynomial::from_string("2x^0y^2 + 1x^1y^0", &ring).unwrap();
    let p7 = Polynomial::from_string("1x^0y^2", &ring).unwrap();

    let (q, r) = divide_polys(&p6, &p7);

    let q1 = Polynomial::from_string("2x^0y^0", &ring).unwrap();
    let r1 = Polynomial::from_string("1x^1y^0", &ring).unwrap();

    assert!(q == q1);
    assert!(r == r1);
}

#[test]
pub fn handpicked_div_poly_set() {
    let ring = Rc::new(Ring { symbols: vec!["x".to_string(), "y".to_string()], ord: MonomialOrdering::DegLex });

    let p1 = Polynomial::from_string("1x^2y^0 + 1x^0y^2 + 1x^0y^0", &ring).unwrap();
    let p2 = Polynomial::from_string("1x^2y^0", &ring).unwrap();
    let p3 = Polynomial::from_string("1x^0y^2", &ring).unwrap();

    let q = Polynomial::from_string("1x^0y^0", &ring).unwrap();

    let (qs, r) = divide_poly_set(&p1, &mut PolySet(vec![p2, p3]));

    assert!(r == q);
    assert!(qs == PolySet(vec![q.clone(), q]));

    let p1 = Polynomial::from_string("5x^1y^3 + 3x^1y^2 + 2x^0y^1", &ring).unwrap();
    let p2 = Polynomial::from_string("1x^0y^2 + 2x^0y^0", &ring).unwrap();
    let p3 = Polynomial::from_string("1x^1y^0 + 4x^0y^0", &ring).unwrap();

    let (qs, r) = divide_poly_set(&p1, &mut PolySet(vec![p2, p3]));

    let p4 = Polynomial::from_string("5x^1y^1 + 3x^1y^0", &ring).unwrap();
    let p5 = Polynomial::from_string("-10x^0y^1 + -6x^0y^0", &ring).unwrap();
    let p6 = Polynomial::from_string("42x^0y^1 + 24x^0y^0", &ring).unwrap();

    println!("qs: {}, r: {}", qs.to_string(), r.to_string());
    println!("p4: {}, p5: {}, p6: {}", p4.to_string(), p5.to_string(), p6.to_string());
    assert!(qs == PolySet(vec![p4, p5]));
    assert!(r == p6);
}

fn basis_test() {
    let mut rng = thread_rng();
    let ring = Rc::new(Ring { symbols: vec![], ord: MonomialOrdering::DegLex });

    for _ in (0..5) {
        let degs = 3;
        let mut polys = Vec::new();
        for _ in (0..4) {
            let mut p = Polynomial { length: 0, terms: Vec::new(), ring: Rc::clone(&ring) };
            for _ in 0..rng.gen_range(0..24) {
                let c1 = rng.gen_range(0..10000);
                let c2 = rng.gen_range(1..10000);

                let c = Rational::from((c1, c2));
                
                let mut degree = Vec::new();
                for _ in (0..degs) {
                    let d: u16 = rng.gen_range(0..500);
                    degree.push(d);
                }

                let m = Monomial { coefficient: c, degree, ring: Rc::clone(&ring) };
                p = add_polys(&p, &Polynomial::from_monom(m));
            }
            polys.push(p);
        }
        let ps = PolySet(polys);
        let mut b = grobner_basis(&ps);

        for p in ps.0 {
            let (_, r) = divide_poly_set(&p, &mut b);
            assert!(r.terms.is_empty());
        }
    }
}

fn handpicked_basis_test() {
    let ring = Rc::new(Ring { symbols: vec![], ord: MonomialOrdering::DegLex });

    let p1 = Polynomial::from_string("t^0u^0x^1y^0z^0 + -1t^1u^0x^0y^0z^0 + -1t^0u^1x^0y^0z^0", &ring).unwrap();
    let p2 = Polynomial::from_string("-1t^2u^0x^0y^0z^0 + -2t^1u^1x^0y^0z^0 + t^0u^0x^0y^1z^0", &ring).unwrap();
    let p3 = Polynomial::from_string("-1t^3u^0x^0y^0z^0 + -3t^2u^1x^0y^0z^0 + t^0u^0x^0y^0z^1", &ring).unwrap();

    let ps = PolySet(vec![p1, p2, p3]);

    let g1 = Polynomial::from_string("t^1u^0x^0y^0z^0 + t^0u^1x^0y^0z^0 + -1t^0u^0x^1y^0z^0", &ring).unwrap();
    let g2 = Polynomial::from_string("t^0u^2x^0y^0z^0 + -1t^0u^0x^2y^0z^0 + t^0u^0x^0y^1z^0", &ring).unwrap();
    let g3 = Polynomial::from_string("t^0u^1x^2 + -1t^0u^1x^0y^1z^0 + -1t^0u^0x^3y^0z^0 + 3/2t^0u^0x^1y^1z^0 + -1/2t^0u^0x^0y^0z^1", &ring).unwrap();
    let g4 = Polynomial::from_string("t^0u^1x^1y^1z^0 + -1t^0u^1x^0y^0z^1 + -1t^0u^0x^2y^1z^0 + -1t^0u^0x^1y^0z^1 + 2t^0u^0x^0y^2z^0", &ring).unwrap();
    let g5 = Polynomial::from_string("t^0u^1x^1y^0z^1 + -1t^0u^1x^0y^2z^0 + t^0u^0x^2y^0z^1 + -1/2t^0u^0x^1y^3z^0 + -1/2t^0u^0x^0y^1z^1", &ring).unwrap();
    let g6 = Polynomial::from_string("t^0u^1x^0y^3z^1 + -1t^0u^1x^0y^0z^2 + -2t^0u^0x^2y^1z^1 + 1/2t^0u^0x^1y^3z^0 + -1t^0u^0x^1y^0z^2 + 5/2t^0u^0x^0y^2z^1", &ring).unwrap();
    let g7 = Polynomial::from_string("t^0u^0x^3y^0z^1 + -3/4t^0u^0x^2y^2z^0 + -3/2t^0u^0x^1y^1z^1 + t^0u^0x^0y^3z^0 + 1/4t^0u^0x^0y^0z^2", &ring).unwrap();

    let gc = PolySet(vec![g1, g2, g3, g4, g5, g6, g7]);

    assert!(grobner_basis(&ps) == gc);
}



