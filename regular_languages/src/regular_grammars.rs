use std::fmt::Display;
use macros::prod;

pub struct Grammar {
    initialSymbol: char,
    productions: Vec<Production>,
    name: String,
}

pub struct Production {
    leftSide: char,
    rightSide: (char, char),
}

// impl Display for Grammar {

// }

// test
impl Grammar {
    pub fn new(&self, initialSymbol: char, mut productions: Vec<Production>, name: String) -> Grammar {
        // productions = if productions.len() == 0 {
        //     vec!(prod!('S' -> "aS" | "a"))
        // } else {
        //     Grammar::validate_productions(productions)
        // };
        prod!(S -> aS | a);

        Grammar{initialSymbol, productions, name}
    }

    pub fn validate_productions(productions: Vec<Production>) -> Vec<Production> {
        // TODO
        productions
    }

    pub fn produce(&self, size: i32) -> Vec<String> {
        // TODO
        vec!("kkk".to_string())
    }
}

impl Production {
    pub fn new(leftSide: char, rightSide: (char, char)) -> Production {
        Production{leftSide, rightSide}
    }
}