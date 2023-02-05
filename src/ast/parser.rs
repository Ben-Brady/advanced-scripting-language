use std::{slice::Iter};
use crate::{Error, Token};
use super::tokens::{*};


pub fn construct(tokens: Vec<Token>) -> Result<AST, Error> {
    let mut iter_tokens = tokens.iter();
    
    let statements = Vec::new();
    
    Ok(AST {
        statements
    })
}

fn parse_print(tokens: &mut Iter<Token>) -> Result<Expression, Error> {
    Err(Error::UnexpectedEndOfTokens)
}

fn get_next(token: Option<&Token>) -> Result<&Token, Error> {
    token.ok_or(Error::UnexpectedEndOfTokens)
}