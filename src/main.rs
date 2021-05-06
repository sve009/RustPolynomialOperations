use polynomial_operations::polynomials::*;
use polynomial_operations::operations::*;

use std::io;
use std::io::prelude::*;
use rug::Rational;
use std::collections::HashMap;

enum Item {
    P(Polynomial),
    Ps(PolySet),
}

// Currently kinda hacky, need to fully decide on input scheme
fn get_poly(x: &str, table: &HashMap<String, Item>) -> Polynomial {
    match table.get(x) {
        Some(Item::P(p)) => p.clone(),
        None => Polynomial::from_string(x),
        _ => Polynomial { length: 0, terms: vec![] },
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

        println!("You wrote {}", s);

        let commands: Vec<&str> = s.split(' ').collect();

        if s == "q" || s == "quit" || s == "exit" {
            break;
        }

        if commands.len() == 3 && commands[0] == "=" {
            println!("{}", commands[1]);
            items.insert(commands[1].to_string(), Item::P(Polynomial::from_string(commands[2])));
        } else if commands.len() == 1 {
            match items.get(commands[0]) {
                Some(Item::P(poly)) => println!("{}", poly.to_string()),
                Some(Item::Ps(set)) => println!("{}", set.to_string()),
                None => println!("Variable {} not recognized, assign with = first", commands[0]),
            }
        } else if commands.len() == 3 && commands[0] == "+" {
            let p1 = get_poly(commands[1], &items);
            let p2 = get_poly(commands[2], &items);
            println!("{}", add_polys(&p1, &p2).to_string());
        } else if commands.len() == 3 && commands[0] == "-" {
            let p1 = get_poly(commands[1], &items);
            let p2 = get_poly(commands[2], &items);
            println!("{}", sub_polys(&p1, &p2).to_string());
        } else if commands.len() == 3 && commands[0] == "*" {
            let p1 = get_poly(commands[1], &items);
            let p2 = get_poly(commands[2], &items);
            println!("{}", mult_polys(&p1, &p2).to_string());
        } else if commands.len() == 3 && commands[0] == "/" {
            let p1 = get_poly(commands[1], &items);
            let p2 = get_poly(commands[2], &items);
            let (q, r) = divide_polys(&p1, &p2);
            println!("q: {}, r: {}", q.to_string(), r.to_string());
        }
    }
}
