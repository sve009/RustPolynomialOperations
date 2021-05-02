pub struct Monomial {
    pub coefficient: i64,
    pub degree: Vec<u16>,
}

impl Monomial {
    pub fn get_degree(&self) -> &Vec<u16> {
        &self.degree
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
            v0.push(Monomial { coefficient: v[n].coefficient + v[n - 1].coefficient, degree: v[n].get_degree().to_vec()});
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

