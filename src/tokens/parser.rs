use super::{Token, Scanner};
use crate::Error;
use std::fs::read_to_string;


pub fn parse_file(filepath: String) -> Result<Vec<Token>, Error> {
    let code = read_to_string(filepath).or(Err(Error::CouldntReadFile))?;
    parse(code)
}


pub fn parse(code: String) -> Result<Vec<Token>, Error> {
    let mut all_tokens = Vec::new();
    let mut scanner = Scanner::new(code);

    loop {
        scanner.strip_whitespace();

        if scanner.is_empty() {
            break;
        }
        
        let is_string = scanner.is_next_token("\"");
        let is_number = scanner.does_next_match(|ch| ch.is_numeric());
        let is_variable = scanner.does_next_match(|ch| matches!(ch, 'A'..='Z' | 'a'..='z' | '_'));

        if let Some(token) = find_constant_tokens(&mut scanner) {
            all_tokens.push(token);
            continue;
        } else if is_string {
            let token = parse_string(&mut scanner);
            all_tokens.push(token);
        } else if is_number {
            let token = parse_number(&mut scanner);
            all_tokens.push(token);
        } else if is_variable {
            let var_name = scanner.take_while(|ch| matches!(ch, 'A'..='Z' | 'a'..='z' | '_'));
            let token = Token::Variable(var_name.to_string());
            all_tokens.push(token);
        } else {
            Err(Error::InvalidToken)?
        }
    };

    Ok(all_tokens)
}

fn find_constant_tokens(scanner: &mut Scanner) -> Option<Token> {
    let token_list = vec![
        ("&&", Token::And),
        ("||", Token::Or),
        ("!", Token::Not),
        ("+", Token::Addition),
        ("-", Token::Subtraction),
        ("/", Token::Division),
        ("%", Token::Modulus),
        ("{", Token::BeginBlock),
        ("}", Token::EndBlock),
        (";", Token::EndStatement),
        ("let", Token::Let),
        ("print", Token::Print),
        ("if", Token::If),
        ("else", Token::Else),
        ("true", Token::LiteralBoolean(true)),
        ("false", Token::LiteralBoolean(false)),
        
        // Shorter tokens may conflict with longer ones
        // Longer ones should be placed first
        ("==", Token::Equals),
        ("!=", Token::NotEquals),
        ("=", Token::Assignment),
        
        ("**", Token::Exponent),
        ("*", Token::Multiplication),

        (">=", Token::GreaterThanOrEqual),
        (">", Token::GreaterThan),

        ("<=", Token::LessThanOrEqual),
        ("<", Token::LessThan),
    ];

    for (token, value) in token_list.iter() {
        if scanner.is_next_token(token) {
            scanner.take_token(token);
            let token = value.to_owned();
            return Some(token);
        }
    }

    None
}

fn parse_string(scanner: &mut Scanner) -> Token {
    let mut literal = String::new();
    
    scanner.take_token("\"");
    let string = scanner.take_while(|ch| ch != &'"');
    scanner.take_token("\"");
    
    literal.push_str(string.as_str());

    Token::LiteralString(literal)
}

fn parse_number(scanner: &mut Scanner) -> Token {
    let literal = scanner.take_while(|ch| ch.is_numeric());
    let num = literal.parse::<usize>().unwrap();
    Token::LiteralNumber(num)
}