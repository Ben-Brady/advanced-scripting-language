mod lexer;
use lexer::Token;
mod parser;
mod interpreter;
use std::env;


fn main() {
    let filepath = get_filepath();
    let tokens = lexer::parse_file(filepath).expect("Could not parse file into tokens");
    println!("Tokens:\n  {:?}", tokens);
    let ast = parser::construct(tokens).expect("Could not construct AST from tokens");
    println!("Abstract Syntax Tree:\n  {:#?}", ast);
    let output = interpreter::run(ast).expect("Interpreter Error");
    println!("\nOutput:\n{}", output);
}

fn get_filepath() -> String {
    let default_file: String = "main.ssl".to_string();
    let path_arg = env::args().nth(1);
    path_arg.unwrap_or(default_file)
}