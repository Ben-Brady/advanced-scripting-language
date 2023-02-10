mod errors;
use errors::{Error};
mod tokens;
use tokens::Token;
mod ast;
mod interpreter;
use std::env;


fn main() {
    let filepath = get_filepath();
    let tokens = tokens::parse_file(filepath).expect("Could not parse file into tokens");
    println!("Tokens: {:?}", tokens);
    let ast = ast::construct(tokens).expect("Could not construct AST from tokens");
    println!("AST: {:#?}", ast);
    let output = interpreter::run(ast);
    println!("Output:\n{}", output);
}

fn get_filepath() -> String {
    let default_file: String = "example.ssl".to_string();
    let path_arg = env::args().nth(1);
    path_arg.unwrap_or(default_file)
}