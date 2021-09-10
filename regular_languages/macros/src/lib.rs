extern crate proc_macro;
use proc_macro::TokenStream;
// use proc_macro2::TokenStream;
// use quote::quote;
// use syn::{braced, parse_macro_input, token, Field, Ident, Result, Token};
// use syn::parse::{Parse, ParseStream};
// use syn::punctuated::Punctuated;

#[proc_macro]
pub fn prod(entry: TokenStream) -> TokenStream {
    let mut prod_inst = String::new();
    let mut prod_code = TokenStream::new();
    let mut entry_iter = entry.into_iter();

    let initial_symbol_string = &(entry_iter.next().unwrap().to_string()[..]);
    let mut initial_symbol_argument = String::from("");

    if initial_symbol_string != "#" {
        assert!(initial_symbol_string == initial_symbol_string.to_ascii_uppercase() && initial_symbol_string.len() == 1);

        initial_symbol_argument.push_str("\"");
        initial_symbol_argument.push_str(initial_symbol_string);
        initial_symbol_argument.push_str("\"");
    } else {
        initial_symbol_argument.push_str(&entry_iter.next().unwrap().to_string()[..]);
        initial_symbol_argument.push_str(".clone()");
    }

    let arrow_left_side = entry_iter.next();
    assert_eq!("-", arrow_left_side.unwrap().to_string());
    let arrow_right_side = entry_iter.next();
    assert_eq!(">", arrow_right_side.unwrap().to_string());

    let mut current_symbol = entry_iter.next();
    let current_symbol_string = current_symbol.unwrap().to_string();

    prod_inst.push_str("{
    let initial_symbol = ");

    prod_inst.push_str(&initial_symbol_argument[..]);

    prod_inst.push_str(";
    let mut prod_vec: Vec<Production> = vec!();
    {
        let prod = Production::new(initial_symbol, ");
    
    if current_symbol_string != "#" {
        assert!(&current_symbol_string.len() > &0);

        prod_inst.push_str("('");

        prod_inst.push_str(&current_symbol_string[0..1]);

        prod_inst.push_str("', \"");

        prod_inst.push_str(&current_symbol_string[1..]);

        prod_inst.push_str("\")");
    } else {
        prod_inst.push_str(&entry_iter.next().unwrap().to_string()[..]);
    }    

    prod_inst.push_str(");
        prod_vec.push(prod);
    }\n");

    current_symbol = entry_iter.next();
    while let Some(separator) = current_symbol {
        assert_eq!(separator.to_string(), "|");

        current_symbol = entry_iter.next();

        let current_symbol_string = current_symbol.unwrap().to_string();

        prod_inst.push_str("    {
        let prod = Production::new(initial_symbol, ");
        
        if current_symbol_string != "#" {
            assert!(&current_symbol_string.len() > &0);

            prod_inst.push_str("('");

            prod_inst.push_str(&current_symbol_string[0..1]);

            prod_inst.push_str("', \"");

            prod_inst.push_str(&current_symbol_string[1..]);
            
            prod_inst.push_str("\")");
        } else {
            prod_inst.push_str(&entry_iter.next().unwrap().to_string()[..]);
        }    

        prod_inst.push_str(");
        prod_vec.push(prod);
    }\n");

        current_symbol = entry_iter.next();
    }

    prod_inst.push_str("    prod_vec
}");

    prod_code.extend::<TokenStream>(prod_inst.parse().unwrap());

    println!("{}", prod_inst);

    prod_code
}