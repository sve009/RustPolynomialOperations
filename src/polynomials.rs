extern crate rug;

use std::cmp::Ordering;
use std::rc::Rc;
use rug::Rational;

// Polynomial representations and supporting functions. 

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ring {
    pub symbols: Vec<String>,
    pub ord: MonomialOrdering,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MonomialOrdering {
    Lex,
    DegLex,
}

#[derive(PartialEq)]
pub struct PolySet(pub Vec<Polynomial>);

impl Clone for PolySet {
    fn clone(&self) -> Self {
        PolySet(self.0.clone())
    }
}

impl ToString for PolySet {
    fn to_string(&self) -> String {
        if self.0.is_empty() {
            return String::from("{}");
        }

        let mut s = String::new();

        s += "{ ";

        for i in 0..self.0.len()-1 {
            s += &self.0[i].to_string();
            s += ", ";
        }

        s += &self.0[self.0.len() - 1].to_string();
        s += " }";

        s
    }
}

#[derive(Debug)]
pub enum MonomError {
    InvalidCoefficient,
    NoAlphaSymbol,
}

#[derive(Eq, Debug)]
pub struct Monomial {
    pub coefficient: rug::Rational,
    pub degree: Vec<u16>,
    pub ring: Rc<Ring>,
}

fn find(v: &Vec<String>, s: &str) -> Option<usize> {
    for i in 0..v.len() {
        if &v[i] == s {
            return Some(i);
        }
    }
    None
}

impl Monomial {
    pub fn get_degree(&self) -> &Vec<u16> {
        &self.degree
    }
    pub fn from_string(s: &str, ring: Rc<Ring>) -> Result<Monomial, MonomError> {
        let (h, mut t) = s.split_at(
            match s.find(|c: char| c.is_alphabetic()) {
                Some(i) => i,
                None => {
                    return Err(MonomError::NoAlphaSymbol)
                },
            });

        let c: Rational = if h.is_empty() {
            Rational::from(1)
        } else {
            match h.parse() {
                Ok(r) => r,
                Err(_) => {
                    return Err(MonomError::InvalidCoefficient)
                },
            }
        };

        let mut v = vec![0; ring.symbols.len()];

        while let Some(i) = t.find('^') {
            let res = t.split_at(i + 1);
            let temp = res.0;
            t = res.1;
            let symb = temp.split_at(temp.len() - 1).0;
            let k = match find(&ring.symbols, symb) {
                Some(a) => a,
                None => return Err(MonomError::InvalidCoefficient),
            };
            t = match t.find(|c: char| c.is_alphabetic()) {
                Some(j) => {
                    let n = t.split_at(j).0.parse::<u16>().unwrap();
                    v[k] = n;
                    t.split_at(j).1
                }
                None => {
                    v[k] = t.parse::<u16>().unwrap();
                    ""
                }
            };
        }
        Ok(Monomial { coefficient: c, degree: v, ring })
    }
}

impl Ord for Monomial {
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = &self.degree;
        let d2 = &other.degree;

        match self.ring.ord {
            MonomialOrdering::Lex => {
                for (a, b) in d1.iter().zip(d2) {
                    if a < b {
                        return Ordering::Less;
                    } else if a > b {
                        return Ordering::Greater;
                    } else {
                        continue;
                    }
                }
                Ordering::Equal
            },
            MonomialOrdering::DegLex => {
                let a1: u16 = d1.iter().sum();
                let a2: u16 = d2.iter().sum();
                if a1 < a2 {
                    Ordering::Less
                } else if a1 > a2 {
                    Ordering::Greater
                } else {
                    for (a, b) in d1.iter().zip(d2) {
                        if a < b {
                            return Ordering::Less;
                        } else if a > b {
                            return Ordering::Greater;
                        } else {
                            continue;
                        }
                    }
                    Ordering::Equal
                }
            }
        }
    }
}

impl PartialOrd for Monomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let d1 = &self.degree;
        let d2 = &other.degree;

        match self.ring.ord {
            MonomialOrdering::Lex => {
                for (a, b) in d1.iter().zip(d2) {
                    if a < b {
                        return Some(Ordering::Less);
                    } else if a > b {
                        return Some(Ordering::Greater);
                    } else {
                        continue;
                    }
                }
                return Some(Ordering::Equal)
            },
            MonomialOrdering::DegLex => {
                let a1: u16 = d1.iter().sum();
                let a2: u16 = d2.iter().sum();
                if a1 < a2 {
                    return Some(Ordering::Less)
                } else if a1 > a2 {
                    return Some(Ordering::Greater)
                } else {
                    for (a, b) in d1.iter().zip(d2) {
                        if a < b {
                            return Some(Ordering::Less);
                        } else if a > b {
                            return Some(Ordering::Greater);
                        } else {
                            continue;
                        }
                    }
                    return Some(Ordering::Equal);
                }
            }
        }
    }
}

impl PartialEq for Monomial {
    fn eq(&self, other: &Self) -> bool {
        self.coefficient == other.coefficient &&
            self.degree.iter().zip(&other.degree)
            .map(|(x, y)| x == y)
            .all(|x| x)
    }
}

impl Clone for Monomial {
    fn clone(&self) -> Self {
        Monomial {
            coefficient: self.coefficient.clone(),
            degree: self.degree.clone(),
            ring: Rc::clone(&self.ring),
        }
    }
}

impl ToString for Monomial {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s += &self.coefficient.to_string();
        for i in 0..self.degree.len() {
            if self.degree[i] != 0 {
                s += &format!("{}{}{}", self.ring.symbols[i], "^", self.degree[i].to_string());
            }
        }

        s
    }
}

#[derive(Debug, Eq)]
pub struct Polynomial {
    pub length: usize,
    pub terms: Vec<Monomial>,
    pub ring: Rc<Ring>,
}

impl Polynomial {
    pub fn get_terms(&self) -> &Vec<Monomial> {
        &self.terms
    }
    pub fn from_string(s: &str, ring: &Rc<Ring>) -> Result<Self, MonomError> {
        let terms: Result<Vec<Monomial>, MonomError> = s.split('+')
            .map(|s| s.trim())
            .map(|s| Monomial::from_string(s, Rc::clone(ring))).collect();
        let t = terms?;
        Ok(Polynomial { length: t.len(), terms: t, ring: Rc::clone(ring) })
    }
    pub fn from_monom(m: Monomial) -> Self {
        let ring = Rc::clone(&m.ring);
        Polynomial {
            length: 1,
            terms: vec![m],
            ring,
        }
    }
    pub fn lt(&self) -> Self {
        Polynomial::from_monom(self.terms[0].clone())
    }
    pub fn lm(&self) -> Monomial {
        Monomial {
            coefficient: Rational::from(1),
            degree: self.terms[0].degree.clone(),
            ring: Rc::clone(&self.ring),
        }
    }
}

impl Ord for Polynomial {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.terms.len() {
            if i >= other.terms.len() {
                return Ordering::Greater;
            } else {
                match self.terms[i].cmp(&other.terms[i]) {
                    Ordering::Equal => continue,
                    x => return x,
                }
            }
        }
        Ordering::Less
    }
}

impl PartialOrd for Polynomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in 0..self.terms.len() {
            if i >= other.terms.len() {
                return Some(Ordering::Greater);
            } else {
                match self.terms[i].cmp(&other.terms[i]) {
                    Ordering::Equal => continue,
                    x => return Some(x),
                }
            }
        }
        Some(Ordering::Less)
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        if self.terms.len() == other.terms.len() {
            self.terms.iter()
                .zip(&other.terms)
                .map(|(x, y)| x == y)
                .all(|x| x) 
        } else {
            false
        }
    }
}

impl Clone for Polynomial {
    fn clone(&self) -> Self {
        Polynomial {
            length: self.length,
            terms: self.terms.clone(),
            ring: Rc::clone(&self.ring),
        }
    }
}

impl ToString for Polynomial {
    fn to_string(&self) -> String {
        if self.terms.is_empty() {
            return String::from("0");
        }

        let mut s = String::new();

        for i in 0..self.terms.len() - 1 {
            s += &self.terms[i].to_string();
            s += " + ";
        }

        s += &self.terms[self.terms.len() - 1].to_string();

        s
    }
}


