use polynomial_operations::polynomials::*;
use polynomial_operations::operations::*;

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
enum ParseError {
    InvalidOperation,
    ArgumentError,
    SyntaxError,
    RingError,
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

fn get_item(x: &str, ring: &Rc<Ring>, table: &HashMap<String, Item>) -> Result<Item, MonomError> {
    match table.get(x) {
        Some(item) => Ok(item.clone()),
        None => Ok(Item::P(Polynomial::from_string(x, ring)?)),
    }
}


fn prep_ps(x: &str, ring: &Rc<Ring>, table: &HashMap<String, Item>) -> Result<Vec<Item>, ParseError> {
    let s: Vec<&str> = x.split(';').collect();
    if s.len() == 1 {
        x.split(' ')
            .map(|a| a.trim())
            .map(|a| parse_expression_h(a, ring, table))
            .collect()
    } else {
        s.iter()
            .map(|a| a.trim())
            .map(|a| parse_expression_h(a, ring, table))
            .collect()
    }
}



fn parse_expression_h(x: &str, ring: &Rc<Ring>, table: &HashMap<String, Item>) -> Result<Item, ParseError> {
    if let Some((op, s)) = x.split_once(' ') {
        if op == "+" {
            let ps: Vec<Item> = prep_ps(s, ring, table)?;
            if ps.len() < 2 {
                return Err(ParseError::ArgumentError);
            }
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Ok(Item::P(add_polys(&p1, &p2))),
                _ => Err(ParseError::ArgumentError),
            }
        } else if op == "-" {
            let ps: Vec<Item> = prep_ps(s, ring, table)?;
            if ps.len() < 2 {
                return Err(ParseError::ArgumentError);
            }
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Ok(Item::P(sub_polys(&p1, &p2))),
                _ => Err(ParseError::ArgumentError),
            }
        } else if op == "*" {
            let ps: Vec<Item> = prep_ps(s, ring, table)?;
            if ps.len() < 2 {
                return Err(ParseError::ArgumentError);
            }
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Ok(Item::P(mult_polys(&p1, &p2))),
                _ => Err(ParseError::ArgumentError),
            }
        } else if op == "/" {
            let ps: Vec<Item> = prep_ps(s, ring, table)?;
            if ps.len() < 2 {
                return Err(ParseError::ArgumentError);
            }
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::P(p2)) => Ok(Item::Qr(divide_polys(&p1, &p2))),
                _ => Err(ParseError::ArgumentError),
            }
        } else if op == "s" {
            let ps:Result<Vec<Polynomial>, ParseError> = prep_ps(s, ring, table)?
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
            let ps: Vec<Item> = prep_ps(s, ring, table)?;
            if ps.len() < 2 {
                return Err(ParseError::ArgumentError);
            }
            match (&ps[0], &ps[1]) {
                (Item::P(p1), Item::Ps(polys)) => Ok(Item::Qsr(divide_poly_set(&p1, &mut polys.clone()))),
                _ => Err(ParseError::ArgumentError),
            }
        } else if op == "base" {
            let ps: Vec<Item> = prep_ps(s, ring, table)?;
            if ps.len() < 1 {
                return Err(ParseError::ArgumentError);
            }
            match &ps[0] {
                Item::Ps(ps) => Ok(Item::Ps(grobner_basis(&ps))),
                _ => Err(ParseError::ArgumentError),
            }
        } else {
            Err(ParseError::InvalidOperation)
        }
    } else {
        match get_item(x, ring, table) {
            Ok(item) => Ok(item),
            Err(_) => Err(ParseError::SyntaxError),
        }
    }
}

fn parse_ring(x: &str) -> Result<Ring, ParseError> {
    let s = match x.split_once('[') {
        Some((_, a)) => a,
        None => return Err(ParseError::SyntaxError),
    };
    let (symbs, ord) = match s.split_once(']') {
        Some((a, b)) => (a, b),
        None => return Err(ParseError::SyntaxError),
    };

    let vars = symbs.split(',').map(|x| x.trim().to_string()).collect();

    let ord = if ord.trim().chars().collect::<Vec<char>>()[0] == 'l' {
        MonomialOrdering::Lex
    } else {
        MonomialOrdering::DegLex
    };

    Ok(Ring { symbols: vars, ord })
}
    

fn parse_expression(x: &str, ring: &mut Option<Rc<Ring>>, table: &mut HashMap<String, Item>) -> Result<Item, ParseError> {
    if let Some(("=", s)) = x.split_once(' ') {
        if let Some((name, s)) = s.split_once(' ') {
            match ring {
                Some(r) => {
                    table.insert(name.to_string(), parse_expression_h(s, &r, table)?);
                    ()
                },
                None => return Err(ParseError::RingError),
            }
        }
        Ok(Item::OK)
    } else if let Some(("setring", s)) = x.split_once(' ') {
        let r = parse_ring(s)?;
        *ring = Some(Rc::new(r));
        Ok(Item::OK)
    } else {
        match ring {
            Some(r) => parse_expression_h(x, &r, table),
            None => Err(ParseError::RingError),
        }
    }
}

fn main() {
    let mut items = HashMap::new();
    let mut ring: Option<Rc<Ring>> = None;

    loop {
        let mut s = String::new();

        print!("> ");
        let _ = io::stdout().flush();

        io::stdin().read_line(&mut s).expect("Failed to read line");
        s = s.trim().to_string();

        if s == "q" || s == "quit" || s == "exit" {
            break;
        }

        let token = match parse_expression(&s, &mut ring, &mut items) {
            Ok(item) => item.to_string(),
            Err(err) => match err {
                ParseError::InvalidOperation => "ParseError: Invalid operation attempted".to_string(),
                ParseError::ArgumentError => 
                "Operation was applied with invalid arguments. Most operations take two polynomials."
                .to_string(),
                ParseError::RingError => "RingError: A ring must be provided".to_string(),
                ParseError::SyntaxError => "ParseError: Invalid syntax".to_string(),
                ParseError::Other => panic!("Something went horribly wrong"),
            }
        };

        println!("{}", token);
    }
}
