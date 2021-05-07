extern crate rug;

use std::cmp::Ordering;
use rug::Rational;

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
}

impl Monomial {
    pub fn get_degree(&self) -> &Vec<u16> {
        &self.degree
    }
    pub fn from_string(s: &str) -> Result<Monomial, MonomError> {
        let (h, mut t) = s.split_at(
            match s.find(|c: char| c.is_alphabetic()) {
                Some(i) => i,
                None => return Err(MonomError::NoAlphaSymbol),
            });

        let c: Rational = match h.parse() {
            Ok(r) => r,
            Err(_) => return Err(MonomError::InvalidCoefficient),
        };

        let mut v = Vec::new();

        while let Some(i) = t.find('^') {
            t = t.split_at(i + 1).1;
            t = match t.find(|c: char| c.is_alphabetic()) {
                Some(j) => {
                    v.push(t.split_at(j).0.parse::<u16>().unwrap());
                    t.split_at(j).1
                }
                None => {
                    v.push(t.parse::<u16>().unwrap());
                    ""
                }
            };
        }
        Ok(Monomial { coefficient: c, degree: v })
    }
}

impl Ord for Monomial {
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = &self.degree;
        let d2 = &other.degree;

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

impl PartialOrd for Monomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let d1 = &self.degree;
        let d2 = &other.degree;

        for (a, b) in d1.iter().zip(d2) {
            if a < b {
                return Some(Ordering::Less);
            } else if a > b {
                return Some(Ordering::Greater);
            } else {
                continue;
            }
        }
        Some(Ordering::Equal)
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
        }
    }
}

impl ToString for Monomial {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s += &self.coefficient.to_string();
        for i in 0..self.degree.len() {
            s += &format!("({}{}){}{} ", "x", (i+1).to_string(), "^", self.degree[i].to_string());
        }

        s
    }
}

#[derive(Debug)]
pub struct Polynomial {
    pub length: usize,
    pub terms: Vec<Monomial>,
}

impl Polynomial {
    pub fn get_terms(&self) -> &Vec<Monomial> {
        &self.terms
    }
    pub fn from_string(s: &str) -> Result<Self, MonomError> {
        let terms: Result<Vec<Monomial>, MonomError> = s.split('+')
            .map(|s| s.trim())
            .map(|s| Monomial::from_string(s)).collect();
        let t = terms?;
        Ok(Polynomial { length: t.len(), terms: t})
    }
    pub fn from_monom(m: Monomial) -> Self {
        Polynomial {
            length: 1,
            terms: vec![m],
        }
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


