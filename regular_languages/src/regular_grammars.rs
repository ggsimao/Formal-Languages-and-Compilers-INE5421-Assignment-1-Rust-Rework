use std::fmt::Display;

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
        productions = if productions.len() == 0 {
            vec!(Production::new('S', "aS"), Production::new('S', "a"))
        } else {
            Grammar::validate_productions(productions)
        };

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
    pub fn new(leftSide: char, rightSide: &str) -> Production {
        let prodRight = match rightSide.len() {
            0 => ('&', '&'),
            1 => (rightSide.chars().nth(0).unwrap(), '&'),
            2 => {
                let mut it = rightSide.chars();
                (it.next().take().unwrap(), it.next().take().unwrap())
            },
            _ => panic!("Production is too long!")
        };
        Production{leftSide, rightSide: prodRight}
    }
}