use polynomial_operations::polynomials::*;

fn main() {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    v1.push(Monomial{ coefficient: 1, degree: vec![3, 2] });
    v1.push(Monomial{ coefficient: 6, degree: vec![1, 3] });
    v1.push(Monomial{ coefficient: 2, degree: vec![0, 2] });

    v2.push(Monomial{ coefficient: 1, degree: vec![5, 0] });
    v2.push(Monomial{ coefficient: 4, degree: vec![1, 3] });
    v2.push(Monomial{ coefficient: 8, degree: vec![1, 1] });

    let f = Polynomial { length: 3, terms: v1 };
    let g = Polynomial { length: 3, terms: v2 };

    println!("An example of polynomial addition: {}", add_polys(&f, &g).to_string());

    let p1 = "1x^3 + 4x^2y^1 + 8x^1y^2";
    println!("The polynomial {} after everything is {}", p1, Polynomial::from_string(p1).to_string());
}
