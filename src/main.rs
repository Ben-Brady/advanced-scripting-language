mod scanner;
use scanner::Scanner;
mod errors;
use errors::{Error};
mod tokens;
use tokens::Token;
mod ast;
use std::env;




fn main() {
    let default_file: String = "example.ssl".to_string();

    let path_arg = env::args().nth(1);
    let filepath = path_arg.unwrap_or(default_file);
    let tokens = tokens::parse_file(filepath).expect("Could not parse file into tokens");
    // let ast = ast::construct(tokens).expect("Could not construct AST from tokens");
    println!("{:?}", tokens);
}
