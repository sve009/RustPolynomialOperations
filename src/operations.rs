extern crate rug;

use super::polynomials::*;

use std::collections::BinaryHeap;
use std::rc::Rc;
use rug::Rational;

// Implementation details for polynomial operatins

pub fn deg_eq(d1: &[u16], d2: &[u16]) -> bool {
    for i in 0..d1.len() {
        if d1[i] != d2[i] {
            return false;
        }
    }
    true
}


fn combine_terms(v: &mut Vec<Monomial>) {
    let mut v0 = Vec::new();

    let ring = Rc::clone(&v[0].ring);

    let mut n = 0;
    while n < v.len() - 1 {
        if deg_eq(&v[n].degree, &v[n + 1].degree) {
            if Rational::from(&v[n].coefficient + &v[n + 1].coefficient) != 0 {
                v0.push(Monomial { coefficient: v[n].coefficient.clone() + v[n + 1].coefficient.clone(), degree: v[n].get_degree().to_vec(), ring: Rc::clone(&ring) });
            }
            n += 2;
        } else if v[n].coefficient != 0 {
            v0.push(v[n].clone());

            n += 1;

        } else {
            n += 1;
        }

        if n == v.len() - 1  && v[n].coefficient != 0 {
            v0.push(v[n].clone());
        }
    }

        
    *v = v0;
}

pub fn add_polys(f: &Polynomial, g: &Polynomial) -> Polynomial {
    let t1 = f.get_terms();
    let t2 = g.get_terms();

    if t1.is_empty() {
        return (*g).clone();
    } else if t2.is_empty() {
        return (*f).clone();
    }

    let mut v = Vec::new();

    let mut n1 = 0;
    let mut n2 = 0;

    while n1 + n2 < t1.len() + t2.len() {
        if n1 < t1.len() && n2 < t2.len() && t1[n1] > t2[n2] {
            v.push(t1[n1].clone());
            n1 += 1;
        } else if n2 < t2.len() {
            v.push(t2[n2].clone());
            n2 += 1;
        } else if n1 < t1.len() {
            v.push(t1[n1].clone());
            n1 += 1;
        }
    }

    combine_terms(&mut v);

    Polynomial {
        length: v.len(),
        terms: v,
        ring: Rc::clone(&f.ring),
    }
}

pub fn sub_polys(f: &Polynomial, g: &Polynomial) -> Polynomial {
    add_polys(f, &scalar_mult(g, Rational::from(-1)))
}

pub fn scalar_mult(f: &Polynomial, n: Rational) -> Polynomial {
    let terms: Vec<Monomial> = f.terms.iter()
        .map(|m| Monomial { coefficient: m.coefficient.clone() * &n, degree: m.degree.clone(), ring: Rc::clone(&m.ring) })
        .collect();
    Polynomial {
        length: terms.len(),
        terms,
        ring: Rc::clone(&f.ring),
    }
}

pub fn mult_monoms(f : &Monomial, g: &Monomial) -> Monomial {
    Monomial {
        coefficient: f.coefficient.clone() * g.coefficient.clone(),
        degree: f.degree.iter().zip(&g.degree).map(|(x, y)| x + y).collect(),
        ring: Rc::clone(&f.ring),
    }
}

pub fn mult_polys(f: &Polynomial, g: &Polynomial) -> Polynomial {
    let t1 = &f.terms;
    let t2 = &g.terms;

    let mut c = Polynomial {
        length: 0,
        terms: Vec::new(),
        ring: Rc::clone(&f.ring),
    };

    let mut h = BinaryHeap::new();
    let mut fs = vec![0; t1.len()];

    for i in 0..t1.len() {
        h.push((mult_monoms(&t1[i], &t2[0]), i));
    }

    while let Some((d, s)) = h.pop() {
        let p = Polynomial { length: 1, terms: vec![d], ring: Rc::clone(&f.ring) };

        c = add_polys(&c, &p);

        if fs[s] < t2.len() - 1 {
            fs[s] += 1;
            h.push((mult_monoms(&t1[s], &t2[fs[s]]), s));
        }
    }
                   
    c
}

pub fn monom_divides(f: &Monomial, g: &Monomial) -> bool {
    let d1 = &f.degree;
    let d2 = &g.degree;

    d1.iter()
        .zip(d2)
        .map(|(x, y)| x <= y)
        .all(|x| x)
}

pub fn poly_divides(f: &Polynomial, g: &Polynomial) -> bool {
    if f.terms.is_empty() {
        return false;
    } else if g.terms.is_empty() {
        return true;
    }

    let m1 = &f.terms[0];

    g.terms.iter()
        .map(|m2| monom_divides(m1, m2))
        .any(|x| x)

}

pub fn divide_monoms(f: &Monomial, g: &Monomial) -> Monomial {
    Monomial { 
        coefficient: (f.coefficient.clone() / g. coefficient.clone()),
        degree: f.degree.iter().zip(&g.degree).map(|(x, y)| x - y).collect(),
        ring: Rc::clone(&f.ring),
    }
}


pub fn divide_polys(f: &Polynomial, g: &Polynomial) -> (Polynomial, Polynomial) {
    let mut q = Polynomial { length: 0, terms: Vec::new(), ring: Rc::clone(&f.ring) };
    let mut r = Polynomial { length: 0, terms: Vec::new(), ring: Rc::clone(&f.ring) };

    let mut rp;

    loop {
        rp = sub_polys(&sub_polys(&f, &mult_polys(&q, &g)), &r);
        if rp.terms.is_empty()  {
            break;
        }

        if poly_divides(&g, &rp) {
            for m in rp.terms {
                if monom_divides(&g.terms[0], &m) {
                    q = add_polys(&q, 
                          &Polynomial::from_monom(
                              divide_monoms(&m,
                                            &g.terms[0])));
                }
            }
        } else {
            r = add_polys(&r, &rp);
        }
    }
    (q, r)
}

fn check_divides(r: &Polynomial, gs: &Vec<Polynomial>) -> bool {
    gs.iter()
        .map(|p| poly_divides(p, r))
        .any(|x| x)
}


pub fn divide_poly_set(f: &Polynomial, g: &mut PolySet) -> (PolySet, Polynomial) {
    // Don't know if this is actually more efficient?
    // g.0.sort_unstable();

    let mut qs: PolySet = PolySet(Vec::new());
    let mut r = Polynomial { length: 0, terms: vec![], ring: Rc::clone(&f.ring) };

    let mut p = f.clone();

    for _ in 0..g.0.len() {
        qs.0.push(Polynomial { length: 0, terms: vec![], ring: Rc::clone(&f.ring) });
    }

    while !p.terms.is_empty() { 
        let mut div = false;
        let mut i = 0;
        while i < qs.0.len() && !div {
            if poly_divides(&g.0[i].lt(), &p.lt()) {
                let cancel = divide_polys(&p.lt(), &g.0[i].lt()).0;
                qs.0[i] = add_polys(&qs.0[i], &cancel);
                p = sub_polys(&p, &mult_polys(&cancel, &g.0[i]));
                div = true;
            } else {
                i += 1;
            }
        }
        if !div {
            r = add_polys(&r, &p.lt());
            p = sub_polys(&p, &p.lt());
        }
    }

    (qs, r)
}

pub fn gcd(f: &Monomial, g: &Monomial) -> Monomial {
    if f.degree.len() != g.degree.len() {
        panic!("Degrees are unequal");
    }

    let degree: Vec<u16> = f.degree.iter()
        .zip(&g.degree)
        .map(|(x, y)| {
            if *x > *y {
                *y
            } else {
                *x
            }
        })
        .collect();

    Monomial { coefficient: Rational::from(1), degree, ring: Rc::clone(&f.ring) }
}

pub fn lcm(f: &Monomial, g: &Monomial) -> Monomial {
    divide_monoms(&mult_monoms(f, g), &gcd(f, g))
}

pub fn s_poly(f: &Polynomial, g: &Polynomial) -> Polynomial {
    if f.terms.is_empty() {
        return scalar_mult(g, Rational::from(-1));
    } else if g.terms.is_empty() {
        return f.clone();
    }

    let m1 = &f.terms[0];
    let m2 = &g.terms[0];

    let deg = m1.degree.iter()
        .zip(&m2.degree)
        .map(|(x, y)| {
            if *x < *y {
                *y
            } else {
                *x
            }
        }).collect();

    let m = Monomial { coefficient: Rational::from(1), degree: deg, ring: Rc::clone(&f.ring) };

    let p1 = Polynomial::from_monom(divide_monoms(&m, m1));
    let p2 = Polynomial::from_monom(divide_monoms(&m, m2));

    sub_polys(&mult_polys(&p1, f), &mult_polys(&p2, g))
}

pub fn grobner_basis(ps: &PolySet) -> PolySet {
    let mut s = ps.0.clone();

    if s.is_empty() {
        return PolySet(s);
    }

    let mut g = PolySet(Vec::new());

    while let Some(f) = s.pop() {
        let (_qs, r) = divide_poly_set(&f, &mut g);
        if !r.terms.is_empty() {
            for p in g.0.iter() {
                if lcm(&r.lm(), &p.lm()) != mult_monoms(&r.lm(), &p.lm()) {
                    s.push(s_poly(&r, &p));
                }
            }
            g.0.push(r);
        }
    }
    reduce(g)
}

pub fn reduce(mut g: PolySet) -> PolySet {
    let mut gp = Vec::new();

    for i in 0..g.0.len() {
        let p = g.0.remove(i);
        let (_, r) = divide_poly_set(&p, &mut g);
        g.0.insert(i, p);
        if !r.terms.is_empty() {
            let r = scalar_mult(&r, Rational::from(1) / &r.terms[0].coefficient);
            gp.push(r);
        }
    }
    PolySet(gp)
}

