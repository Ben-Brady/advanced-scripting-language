use crate::{Token, Error};


pub fn are_variants_equal<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

pub fn pop_next_token(tokens: &mut Vec<Token>) -> Result<Token, Error>{
    tokens.pop().ok_or(Error::UnexpectedEndOfTokens)
}

pub fn peek_next_token(tokens: &mut [Token]) -> Result<&Token, Error>{
    tokens.last().ok_or(Error::UnexpectedEndOfTokens)
}

pub fn require_next_token(tokens: &mut Vec<Token>, token: Token) -> Result<(), Error>{
    let next_token = pop_next_token(tokens)?;
    if are_variants_equal(&next_token, &token) {
        Ok(())
    } else {
        Err(Error::UnexpectedToken)
    }
}

pub fn is_next_token_and_take(tokens: &mut Vec<Token>, token: Token) -> Result<bool, Error>{
    let next_token = peek_next_token(tokens)?;
    let is_next_token = are_variants_equal(next_token, &token);
    if is_next_token {
        pop_next_token(tokens)?;
    }

    Ok(is_next_token)
}
