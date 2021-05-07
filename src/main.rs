use polynomial_operations::polynomials::*;
use polynomial_operations::operations::*;

use std::io;
use std::io::prelude::*;
use rug::Rational;
use std::collections::HashMap;

#[derive(Debug)]
enum ParseError {
    InvalidOperation,
    ArgumentError,
    Other,
}

#[derive(Clone)]
enum Item {
    P(Polynomial),
    Qr((Polynomial, Polynomial)),
    Ps(PolySet),
    Qsr((PolySet, Polynomial)),
    OK,
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match &self {
            Self::P(p1) => p1.to_string(),
            Self::Qr((q, r)) => format!("q: {}, r: {}", q.to_string(), r.to_string()),
            Self::Ps(ps) => ps.to_string(),
            Self::Qsr((ps, r)) => format!("qs: {}, r: {}", ps.to_string(), r.to_string()),
            Self::OK => String::new(),
        }
    }
}

fn get_item(x: &str, table: &HashMap<String, Item>) -> Result<Item, MonomError> {
    match table.get(x) {
        Some(item) => Ok(item.clone()),
        None => Ok(Item::P(Polynomial::from_string(x)?)),
    }
}


fn prep_ps(x: &str, table: &HashMap<String, Item>) -> Result<Vec<Item>, ParseError> {
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



fn parse_expression_h(x: &str, table: &HashMap<String, Item>) -> Result<Item, ParseError> {
    if let Some((op, s)) = x.split_once(' ') {
        if op == "+" {
            let ps: Vec<Item> = prep_ps(s, table)?;
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Ok(Item::P(add_polys(&p1, &p2))),
                _ => Err(ParseError::ArgumentError),
            }
        } else if op == "-" {
            let ps: Vec<Item> = prep_ps(s, table)?;
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Ok(Item::P(sub_polys(&p1, &p2))),
                _ => Err(ParseError::ArgumentError),
            }
        } else if op == "*" {
            let ps: Vec<Item> = prep_ps(s, table)?;
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Ok(Item::P(mult_polys(&p1, &p2))),
                _ => Err(ParseError::ArgumentError),
            }
        } else if op == "/" {
            let ps: Vec<Item> = prep_ps(s, table)?;
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Ok(Item::Qr(divide_polys(&p1, &p2))),
                _ => Err(ParseError::ArgumentError),
            }
        } else if op == "s" {
            let ps:Result<Vec<Polynomial>, ParseError> = prep_ps(s, table)?
                .iter()
                .map(|x| {
                    match x {
                        Item::P(p1) => Ok(p1.clone()),
                        _ => Err(ParseError::ArgumentError),
                    }
                })
                .collect();
            Ok(Item::Ps(PolySet(ps?)))
        } else if op == "/s" {
            let ps: Vec<Item> = prep_ps(s, table)?;
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::Ps(ps)) => Ok(Item::Qsr(divide_poly_set(&p1, &ps))),
                _ => Err(ParseError::ArgumentError),
            }
        } else if op == "base" {
            let ps: Vec<Item> = prep_ps(s, table)?;
            match &ps[0] {
                Item::Ps(ps) => Ok(Item::Ps(grobner_basis(&ps))),
                _ => Err(ParseError::ArgumentError),
            }
        } else {
            Err(ParseError::InvalidOperation)
        }
    } else {
        match get_item(x, table) {
            Ok(item) => Ok(item),
            Err(_) => Err(ParseError::InvalidOperation),
        }
    }
}

fn parse_expression(x: &str, table: &mut HashMap<String, Item>) -> Result<Item, ParseError> {
    if let Some(("=", s)) = x.split_once(' ') {
        if let Some((name, s)) = s.split_once(' ') {
            table.insert(name.to_string(), parse_expression_h(s, table)?);
        }
        Ok(Item::OK)
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

        if s == "q" || s == "quit" || s == "exit" {
            break;
        }

        let token = match parse_expression(&s, &mut items) {
            Ok(item) => item.to_string(),
            Err(err) => match err {
                ParseError::InvalidOperation => "ParseError: Invalid operation attempted".to_string(),
                ParseError::ArgumentError => 
                "Operation was applied with invalid arguments. Most operations take two polynomials."
                .to_string(),
                ParseError::Other => panic!("Something went horribly wrong"),
            }
        };

        println!("{}", token);
    }
}
