extern crate proc_macro;
use proc_macro::TokenStream;
// use proc_macro2::TokenStream;
use quote::quote;
// use syn::{braced, parse_macro_input, token, Field, Ident, Result, Token};
// use syn::parse::{Parse, ParseStream};
// use syn::punctuated::Punctuated;

#[proc_macro]
pub fn prod(entry: TokenStream) -> TokenStream {
    let entryClone = entry.clone();

    for i in entryClone.clone() {
        println!("{}", i);
    }

    let mut entryIter = entryClone.into_iter();
    let initialSymbol = entryIter.next().unwrap();
    let initialSymbolString = &(initialSymbol.to_string()[..]);
    let mut initialSymbolArgument;

    // println!("{}", initialSymbolString);

    if initialSymbolString != "#" {
        assert!(initialSymbolString == initialSymbolString.to_ascii_uppercase() && initialSymbolString.len() == 1);
        initialSymbolArgument = String::from("'");
        initialSymbolArgument.push_str(initialSymbolString);
        initialSymbolArgument.push_str("'");
    } else {
        initialSymbolArgument = entryIter.next().unwrap().to_string();
    }

    // println!("{}", initialSymbolArgument);

    let arrowLeftSide = entryIter.next();
    assert_eq!("-", arrowLeftSide.unwrap().to_string());
    let arrowRightSide = entryIter.next();
    assert_eq!(">", arrowRightSide.unwrap().to_string());

    let mut prodInst = String::new();
    let mut prodCode = TokenStream::new();

    let mut currentSymbol = entryIter.next();
    let currentSymbolString = currentSymbol.unwrap().to_string();
    println!("Primeira produção: {:?}", currentSymbolString);

    prodInst.push_str("let mut prodVec: Vec<Production> = vec!();\n");
    prodInst.push_str("prodVec.push(Production::new(");
    prodInst.push_str(&initialSymbolArgument[..]);
    prodInst.push_str(",");
    
    prodInst.push_str("(");
    if currentSymbolString != "#" {
        assert!(&currentSymbolString.len() < &3 && &currentSymbolString.len() > &0);
        prodInst.push_str("'");
        prodInst.push_str(&currentSymbolString[0..1]);
        prodInst.push_str("','");
        if &currentSymbolString.len() == &2 {
            prodInst.push_str(&currentSymbolString[1..2]);
        } else {
            prodInst.push_str("&");
        }
        prodInst.push_str("'");
    } else {
        prodInst.push_str(&entryIter.next().unwrap().to_string()[..]);
    }    
    prodInst.push_str(")");

    prodInst.push_str("));\n");

    println!("{:?}", prodInst.to_string());

    currentSymbol = entryIter.next();
    while let Some(separator) = currentSymbol {
        assert_eq!(separator.to_string(), "|");

        currentSymbol = entryIter.next();

        let currentSymbolString = currentSymbol.unwrap().to_string();

        prodInst.push_str("prodVec.push(Production::new(");
        prodInst.push_str(&initialSymbolArgument[..]);
        prodInst.push_str(",");
        
        prodInst.push_str("(");
        if currentSymbolString != "#" {
            assert!(&currentSymbolString.len() < &3 && &currentSymbolString.len() > &0);
            prodInst.push_str("'");
            prodInst.push_str(&currentSymbolString[0..1]);
            prodInst.push_str("','");
            if &currentSymbolString.len() == &2 {
                prodInst.push_str(&currentSymbolString[1..2]);
            } else {
                prodInst.push_str("&");
            }
            prodInst.push_str("'");
        } else {
            prodInst.push_str(&entryIter.next().unwrap().to_string()[..]);
        }    
        prodInst.push_str(")");

        prodInst.push_str("));\n");

        currentSymbol = entryIter.next();
    }
    prodCode.extend::<TokenStream>(prodInst.parse().unwrap());
    prodCode
}