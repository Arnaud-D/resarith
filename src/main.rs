use std::env;

use crate::{
    eval::eval,
    lex::tokenize,
    parse::parse,
};

mod parse;
mod eval;
mod defs;
mod lex;

fn print_help() {
    println!("Usage: resarith input");
    println!("Example: resarith \"(1 : (4 | 3.5)\"");
}

fn print_eval(string: String) {
    let mut tokens = tokenize(string);
    let expr = parse(&mut tokens);
    let result = eval(expr);
    println!("{}", result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => print_eval(args[1].clone()),
        _ => print_help(),
    }
}
