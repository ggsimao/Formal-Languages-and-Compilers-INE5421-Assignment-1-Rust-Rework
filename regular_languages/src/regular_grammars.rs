use std::fmt::Display;
use macros::prod;

pub struct Grammar {
    initial_symbol: char,
    productions: Vec<Production>,
    name: String,
}

#[derive(Copy, Clone)]
pub struct Production {
    left_side: char,
    right_side: (char, char),
}

// impl Display for Grammar {

// }

// test
impl Grammar {
    pub fn new(&self, initial_symbol: char, mut productions: Vec<Production>, name: String) -> Grammar {
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
        let mut prodVec = self.productions.clone();
        let mut toCheckProds: Vec<String> = vec!();

        while let Some(x) = prodVec.iter().position(|&i| i.leftSide() == self.initial_symbol) {
            let mut rightString = String::new();
            let rightTuple: (char, char) = prodVec.remove(x).rightSide();
            if size == 0 && rightTuple.0 != '&' {
                rightString.push(rightTuple.0);
                rightString.push(rightTuple.1);
                toCheckProds.push(rightString);
            }
        }

        vec!("kkk".to_string())
    }
}

impl Production {
    pub fn new(left_side: char, right_side: (char, char)) -> Production {
        Production{left_side, right_side}
    }

    pub fn leftSide(&self) -> char {
        self.left_side
    }

    pub fn rightSide(&self) -> (char, char) {
        self.right_side
    }
}