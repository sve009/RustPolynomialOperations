use polynomial_operations::polynomials::*;
use polynomial_operations::operations::*;

use std::io;
use std::io::prelude::*;
use rug::Rational;
use std::collections::HashMap;

enum Item {
    P(Polynomial),
    Qr((Polynomial, Polynomial)),
    Ps(PolySet),
    OK,
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match &self {
            Self::P(p1) => p1.to_string(),
            Self::Qr((q, r)) => format!("q: {}, r: {}", q.to_string(), r.to_string()),
            Self::Ps(ps) => ps.to_string(),
            Self::OK => String::new(),
        }
    }
}

// Currently kinda hacky, need to fully decide on input scheme
fn get_poly(x: &str, table: &HashMap<String, Item>) -> Polynomial {
    match table.get(x) {
        Some(Item::P(p)) => p.clone(),
        None => Polynomial::from_string(x),
        _ => Polynomial { length: 0, terms: vec![] },
    }
}

fn prep_ps(x: &str, table: &HashMap<String, Item>) -> Vec<Item> {
    let s: Vec<&str> = x.split(';').collect();
    if s.len() == 1 {
        x.split(' ')
            .map(|a| a.trim())
            .map(|a| parse_expression_h(a, table))
            .collect()
    } else {
        s.iter()
            .map(|a| a.trim())
            .map(|a| parse_expression_h(a, table))
            .collect()
    }
}



fn parse_expression_h(x: &str, table: &HashMap<String, Item>) -> Item {
    if let Some((op, s)) = x.split_once(' ') {
        if op == "+" {
            let ps: Vec<Item> = prep_ps(s, table);
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Item::P(add_polys(&p1, &p2)),
                _ => panic!("The first two arguments were not polynomials"),
            }
        } else if op == "-" {
            let ps: Vec<Item> = prep_ps(s, table);
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Item::P(sub_polys(&p1, &p2)),
                _ => panic!("The first two arguments were not polynomials"),
            }
        } else if op == "*" {
            let ps: Vec<Item> = prep_ps(s, table);
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Item::P(mult_polys(&p1, &p2)),
                _ => panic!("The first two arguments were not polynomials"),
            }
        } else if op == "/" {
            let ps: Vec<Item> = prep_ps(s, table);
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Item::Qr(divide_polys(&p1, &p2)),
                _ => panic!("The first two arguments were not polynomials"),
            }
        }
        else {
            panic!("Operation not recognized");
        }
    } else 
        Item::P(get_poly(x, table))
    }
}

fn parse_expression(x: &str, table: &mut HashMap<String, Item>) -> Item {
    if let Some(("=", s)) = x.split_once(' ') {
        if let Some((name, s)) = s.split_once(' ') {
            table.insert(name.to_string(), parse_expression_h(s, table));
        }
        Item::OK
    } else {
        parse_expression_h(x, table)
    }
}

fn main() {
    let mut items = HashMap::new();

    loop {
        let mut s = String::new();

        print!("> ");
        let _ = io::stdout().flush();

        io::stdin().read_line(&mut s).expect("Failed to read line");
        s = s.trim().to_string();

        println!("{}", parse_expression(&s, &mut items).to_string());

        if s == "q" || s == "quit" || s == "exit" {
            break;
        }

    }
}
