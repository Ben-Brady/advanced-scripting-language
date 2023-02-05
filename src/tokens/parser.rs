use super::Token;
use crate::{Error, Scanner};
use std::fs::read_to_string;


pub fn parse_file(filepath: String) -> Result<Vec<Token>, Error> {
    let code = read_to_string(filepath).or(Err(Error::CouldntReadFile))?;
    parse(code)
}


pub fn parse(code: String) -> Result<Vec<Token>, Error> {
    let mut all_tokens = Vec::new();
    let mut scanner = Scanner::new(code);

    let token_list = vec![
        ("&&", Token::And),
        ("||", Token::Or),
        ("!", Token::Not),
        ("+", Token::Addition),
        ("-", Token::Subtraction),
        ("let", Token::Let),
        ("print", Token::Print),
        ("if", Token::If),
        ("else", Token::Else),
        ("\n", Token::EndStatement),

        // Shorter tokens may conflict with longer ones
        // Longer ones should be placed first
        ("==", Token::Equal),
        ("!=", Token::NotEqual),
        ("=", Token::Assignment),

        (">=", Token::GreaterThanOrEqual),
        (">", Token::GreaterThan),

        ("<=", Token::LessThanOrQual),
        ("<", Token::LessThan),
    ];

    loop {
        scanner.strip_whitespace();

        if scanner.is_empty() {
            break;
        }
        
        let mut const_token = None;

        for (token, value) in token_list.iter() {
            if scanner.is_next_token(token) {
                scanner.take_token(token);
                const_token = Some(value.to_owned());
                break;
            }
        }

        if let Some(token) = const_token {
            all_tokens.push(token);
            continue;
        }

        if scanner.is_next_token("\"") {
            let token = parse_string(&mut scanner);
            all_tokens.push(token);
        } else if scanner.does_next_match(|ch| ch.is_numeric()) {
            let token = parse_number(&mut scanner);
            all_tokens.push(token);
        } else if scanner.does_next_match(|ch| matches!(ch, 'A'..='Z' | 'a'..='z' | '_')) {
            let var_name = scanner.take_while(|ch| !ch.is_whitespace());
            let token = Token::Variable(var_name.to_string());
            all_tokens.push(token);
        } else {
            Err(Error::InvalidToken)?
        }
    };

    Ok(all_tokens)
}

fn parse_string(scanner: &mut Scanner) -> Token {
    let mut literal = String::new();
    
    scanner.take_token("\"");
    let string = scanner.take_while(|ch| ch != &'"');
    scanner.take_token("\"");
    
    literal.push('"');
    literal.push_str(string.as_str());
    literal.push('"');

    Token::LiteralString(literal)
}

fn parse_number(scanner: &mut Scanner) -> Token {
    let literal = scanner.take_while(|ch| ch.is_numeric());
    let num = literal.parse::<usize>().unwrap();
    Token::LiteralNumber(num)
}