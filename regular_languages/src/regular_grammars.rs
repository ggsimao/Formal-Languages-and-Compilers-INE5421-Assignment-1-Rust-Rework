use std::fmt::Display;
use macros::prod;

pub struct Grammar<'a> {
    initial_symbol: &'a str,
    productions: Vec<Production<'a>>,
    name: String,
}

#[derive(Copy, Clone)]
pub struct Production<'a> {
    left_side: &'a str,
    right_side: (char, &'a str),
}

// impl Display for Grammar {

// }

// test
impl Grammar<'_> {
    pub fn new<'a>(&self, initial_symbol: &'a str, mut productions: Vec<Production<'a>>, name: String) -> Grammar<'a> {
        productions = if productions.len() == 0 {
            prod!(S -> aS | a)
        } else {
            Grammar::validate_productions(productions)
        };

        Grammar{initial_symbol, productions, name}
    }

    fn validate_productions(productions: Vec<Production>) -> Vec<Production> {
        // TODO
        productions
    }

    pub fn produce(&self, size: u32) -> Vec<String> {
        // TODO
        let mut prod_vec = self.productions.clone();
        let mut to_check_prods: Vec<String> = vec!();

        while let Some(x) = prod_vec.iter().position(|&i| i.left_side() == self.initial_symbol) {
            let mut right_string = String::new();
            let removed_production = prod_vec.remove(x).clone();
            let right_tuple: (char, &str) = removed_production.right_side();
            if size == 0 && right_tuple.0 != '&' {
                right_string.push(right_tuple.0);
                right_string.push_str(right_tuple.1);
                to_check_prods.push(right_string);
            }
        }

        vec!("kkk".to_string())
    }
}

impl Production<'_> {
    pub fn new<'a>(left_side: &'a str, right_side: (char, &'a str)) -> Production<'a> {
        Production{left_side, right_side}
    }

    pub fn left_side(&self) -> &str {
        self.left_side
    }

    pub fn right_side(&self) -> (char, &str) {
        self.right_side
    }
}