extern crate proc_macro;
use proc_macro::TokenStream;
// use proc_macro2::TokenStream;
// use quote::quote;
// use syn::{braced, parse_macro_input, token, Field, Ident, Result, Token};
// use syn::parse::{Parse, ParseStream};
// use syn::punctuated::Punctuated;

#[proc_macro]
pub fn prod(entry: TokenStream) -> TokenStream {
    let entry_clone = entry.clone();

    // TODO: include multiple token parameters, e.g.: prod!(#self.leftSide(), #self.rightSide())

    // for i in entry_clone.clone() {
    //     println!("{}", i);
    // }

    let mut entry_iter = entry_clone.into_iter();
    let initial_symbol = entry_iter.next().unwrap();
    let initial_symbol_string = &(initial_symbol.to_string()[..]);
    let mut initial_symbol_argument;

    // println!("{}", initial_symbol_string);

    if initial_symbol_string != "#" {
        assert!(initial_symbol_string == initial_symbol_string.to_ascii_uppercase() && initial_symbol_string.len() == 1);
        initial_symbol_argument = String::from("'");
        initial_symbol_argument.push_str(initial_symbol_string);
        initial_symbol_argument.push_str("'");
    } else {
        initial_symbol_argument = entry_iter.next().unwrap().to_string();
    }

    // println!("{}", initial_symbol_argument);

    let arrow_left_side = entry_iter.next();
    assert_eq!("-", arrow_left_side.unwrap().to_string());
    let arrow_right_side = entry_iter.next();
    assert_eq!(">", arrow_right_side.unwrap().to_string());

    let mut prod_inst = String::new();
    let mut prod_code = TokenStream::new();
    

    let mut current_symbol = entry_iter.next();
    let current_symbol_string = current_symbol.unwrap().to_string();
    // println!("Primeira produção: {:?}", current_symbol_string);

    prod_inst.push_str("{let mut prod_vec: Vec<Production> = vec!();\n");
    prod_inst.push_str("prod_vec.push(Production::new(");
    prod_inst.push_str(&initial_symbol_argument[..]);
    prod_inst.push_str(",");
    
    if current_symbol_string != "#" {
        prod_inst.push_str("(");
        assert!(&current_symbol_string.len() < &3 && &current_symbol_string.len() > &0);
        prod_inst.push_str("'");
        prod_inst.push_str(&current_symbol_string[0..1]);
        prod_inst.push_str("','");
        if &current_symbol_string.len() == &2 {
            prod_inst.push_str(&current_symbol_string[1..2]);
        } else {
            prod_inst.push_str("&");
        }
        prod_inst.push_str("'");
        prod_inst.push_str(")");
    } else {
        prod_inst.push_str(&entry_iter.next().unwrap().to_string()[..]);
    }    

    prod_inst.push_str("));\n");

    // println!("{:?}", prod_inst.to_string());

    current_symbol = entry_iter.next();
    while let Some(separator) = current_symbol {
        assert_eq!(separator.to_string(), "|");

        current_symbol = entry_iter.next();

        let current_symbol_string = current_symbol.unwrap().to_string();

        prod_inst.push_str("prod_vec.push(Production::new(");
        prod_inst.push_str(&initial_symbol_argument[..]);
        prod_inst.push_str(",");
        
        if current_symbol_string != "#" {
            prod_inst.push_str("(");
            assert!(&current_symbol_string.len() < &3 && &current_symbol_string.len() > &0);
            prod_inst.push_str("'");
            prod_inst.push_str(&current_symbol_string[0..1]);
            prod_inst.push_str("','");
            if &current_symbol_string.len() == &2 {
                prod_inst.push_str(&current_symbol_string[1..2]);
            } else {
                prod_inst.push_str("&");
            }
            prod_inst.push_str("'");
            prod_inst.push_str(")");
        } else {
            prod_inst.push_str(&entry_iter.next().unwrap().to_string()[..]);
        }    

        prod_inst.push_str("));\n");

        current_symbol = entry_iter.next();
    }
    prod_inst.push_str("prod_vec\n}");
    prod_code.extend::<TokenStream>(prod_inst.parse().unwrap());
    println!("{}", prod_code);
    prod_code
}