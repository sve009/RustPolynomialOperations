use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq)]
pub struct Monomial {
    pub coefficient: i64,
    pub degree: Vec<u16>,
}

impl Monomial {
    pub fn get_degree(&self) -> &Vec<u16> {
        &self.degree
    }
}

impl Ord for Monomial {
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = &self.degree;
        let d2 = &other.degree;

        for (a, b) in d1.iter().zip(d2) {
            if a < &b {
                return Ordering::Less;
            } else if a > &b {
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
            if a < &b {
                return Some(Ordering::Less);
            } else if a > &b {
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
            .fold(true, |x, y| x && y)
    }
}

impl Clone for Monomial {
    fn clone(&self) -> Self {
        Monomial {
            coefficient: self.coefficient,
            degree: self.degree.clone(),
        }
    }
}

impl ToString for Monomial {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s += &self.coefficient.to_string();
        for i in (0..(self.degree.len())) {
            s += &format!("{}{}{}{}", "x", (i+1).to_string(), "^", self.degree[i].to_string());
        }

        s
    }
}

pub struct Polynomial {
    pub length: usize,
    pub terms: Vec<Monomial>,
}

impl Polynomial {
    pub fn get_terms(&self) -> &Vec<Monomial> {
        &self.terms
    }
    pub fn from_string(s: &str) -> Polynomial {
        let terms: Vec<Monomial> = s.split("+")
            .map(|s| s.trim())
            .map(|s| {
                let (h, mut t) = s.split_at(s.find(|c: char| c.is_alphabetic()).unwrap());
                println!("h: {}, t: {}", h, t);
                let c = h.parse::<i64>().unwrap();

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
                Monomial { coefficient: c, degree: v }
            }).collect();
        Polynomial { length: terms.len(), terms: terms }
    }
}

impl ToString for Polynomial {
    fn to_string(&self) -> String {
        let mut s = String::new();

        println!("Length: {}", &self.terms.len());

        for term in &self.terms {
            s += &term.to_string();
            s += " + ";
        }

        s
    }
}

fn higher_deg(d1: &Vec<u16>, d2: &Vec<u16>) -> bool {
    for i in (0..(d1.len())) {
        if d1[i] > d2[i] {
            return true;
        } else if d1[i] < d2[i] {
            return false;
        } else {
            continue;
        }
    }
    true
}

fn deg_eq(d1: &Vec<u16>, d2: &Vec<u16>) -> bool {
    for i in (0..(d1.len())) {
        if d1[i] != d2[i] {
            return false;
        }
    }
    true
}


fn combine_terms(v: &mut Vec<Monomial>) {
    let mut v0 = Vec::new();

    let mut n = 1;
    while n < v.len() {
        if deg_eq(v[n].get_degree(), v[n - 1].get_degree()) {
            if v[n].coefficient + v[n - 1].coefficient != 0 {
                v0.push(Monomial { coefficient: v[n].coefficient + v[n - 1].coefficient, degree: v[n].get_degree().to_vec()});
            }
        } else {
            v0.push(v[n].clone());
        }
        n += 1;
    }
        
    *v = v0;
}

pub fn add_polys(f: &Polynomial, g: &Polynomial) -> Polynomial {
    let t1 = f.get_terms();
    let t2 = g.get_terms();

    let mut v = Vec::new();

    let mut n1 = 0;
    let mut n2 = 0;

    while n1 + n2 < t1.len() + t2.len() {
        if n1 < t1.len() && n2 < t2.len() && higher_deg(t1[n1].get_degree(), t2[n2].get_degree()) {
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

    print!("Length before combine: {}", v.len());

    combine_terms(&mut v);

    print!("Length after combine: {}", v.len());

    Polynomial {
        length: v.len(),
        terms: v,
    }
}

pub fn mult_monoms(f : &Monomial, g: &Monomial) -> Monomial {
    Monomial {
        coefficient: f.coefficient * g.coefficient,
        degree: f.degree.iter().zip(&g.degree).map(|(x, y)| x + y).collect(),
    }
}

pub fn mult_polys(f: &Polynomial, g: &Polynomial) -> Polynomial {
    let t1 = &f.terms;
    let t2 = &g.terms;

    let mut c = Polynomial { length: 0, terms: Vec::new() };

    let mut h = BinaryHeap::new();
    let mut fs = vec![0; t1.len()];

    for i in (0..t1.len()) {
        h.push((mult_monoms(&t1[i], &t2[1]), i));
    }

    while let Some((d, s)) = h.pop() {
        let p = Polynomial { length: 0, terms: vec![d] };
        c = add_polys(&c, &p);

        if fs[s] < t2.len() {
            fs[s] += 1;
            h.push((mult_monoms(&t1[s], &t2[fs[s]]), s));
        }
    }
                   
    c
}

