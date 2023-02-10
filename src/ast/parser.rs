use crate::{Error, Token};
use super::utils::{pop_next_token, peek_next_token};
use super::{Block, utils};
use super::tokens::{Expression, Literal, BinaryOperator};


pub fn construct(tokens: Vec<Token>) -> Result<Block, Error> {
    let mut tokens = tokens;
    tokens.reverse();
    construct_expressions(&mut tokens)
}


fn construct_expression(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    let first_token = match utils::pop_next_token(tokens)? {
        Token::BeginBlock => construct_block(tokens),
        Token::Let => construct_declaration(tokens),
        Token::Print => construct_print(tokens),
        Token::If => construct_if(tokens),
        Token::Variable(name) => match utils::peek_next_token(tokens) {
            Ok(Token::Assignment) => construct_assignment(name, tokens),
            _ => Ok(Expression::Variable(name)),
        },
        Token::LiteralBoolean(value) => Ok(Expression::Literal(Literal::Boolean(value))),
        Token::LiteralNumber(value) => Ok(Expression::Literal(Literal::Number(value))),
        Token::LiteralString(value) => Ok(Expression::Literal(Literal::String(value))),
        token => {
            println!("Unexpected Token: {:?}", token);
            Err(Error::UnexpectedToken)?
        },
    };

    if let Ok(op_token) = peek_next_token(tokens) {
        let op = match op_token {
            Token::Addition => Some(BinaryOperator::Addition),
            Token::Equals => Some(BinaryOperator::Equals),
            Token::NotEquals => Some(BinaryOperator::NotEquals),
            Token::GreaterThan => Some(BinaryOperator::GreaterThan),
            Token::GreaterThanOrEqual => Some(BinaryOperator::GreaterThanOrEqual),
            Token::LessThan => Some(BinaryOperator::LessThan),
            Token::LessThanOrEqual => Some(BinaryOperator::LessThanOrEqual),
            Token::And => Some(BinaryOperator::And),
            Token::Or => Some(BinaryOperator::Or),
            Token::Subtraction => Some(BinaryOperator::Subtraction),
            Token::Division => Some(BinaryOperator::Division),
            Token::Multiplication => Some(BinaryOperator::Multiplication),
            Token::Exponent => Some(BinaryOperator::Exponent),
            Token::Modulus => Some(BinaryOperator::Modulus),
            _ => None,
        };
        
        if let Some(op) = op {
            pop_next_token(tokens)?;
            return Ok(Expression::BinOp {
                op,
                left_side: Box::new(first_token?),
                right_side: Box::new(construct_expression(tokens)?),
            });
        }
    }
    first_token
}

fn construct_declaration(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    let name = match utils::pop_next_token(tokens)? {
        Token::Variable(name) => name,
        _ => Err(Error::UnexpectedToken)?,
    };
    
    utils::require_next_token(tokens, Token::Assignment)?;
    let value = Box::new(construct_expression(tokens)?);
    
    Ok(Expression::Declaration { name, value })
}

fn construct_assignment(var_name: String, tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    utils::require_next_token(tokens, Token::Assignment)?;
    let value = Box::new(construct_expression(tokens)?);
    Ok(Expression::Assignment { name: var_name, value })
}

fn construct_print(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    let expr = construct_expression(tokens)?;
    Ok(Expression::Print(
        Box::new(expr)
    ))
}

fn construct_if(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    let condition = construct_expression(tokens)?;
    let if_branch = construct_expression(tokens)?;
    
    let else_branch = match utils::is_next_token_and_take(tokens, Token::Else) {
        Ok(true) => Some(construct_expression(tokens)?),
        _ => None
    };
    
    Ok(Expression::If {
        condition: Box::new(condition),
        if_branch: Box::new(if_branch),
        else_branch: Box::new(else_branch)
    })
}

fn construct_block(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    let mut expressions = Vec::new();

    loop {
        if utils::is_next_token_and_take(tokens, Token::EndBlock)? {
            break;
        }

        let expression = construct_expression(tokens)?;
        expressions.push(expression);
    }

    Ok(Expression::Block(Box::new(expressions)))
}

fn construct_expressions(tokens: &mut Vec<Token>) -> Result<Block, Error> {
    let mut ast = Vec::new();

    while !tokens.is_empty() {
        let expression = construct_expression(tokens)?;
        ast.push(expression);
    }

    Ok(ast)
}
