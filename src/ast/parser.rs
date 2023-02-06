
use crate::{Error, Token};
use super::AST;
use super::tokens::{Expression, Literal, BinaryOperator};


pub fn demo() -> AST {
    vec![
        Expression::Assignment {
            name: "var_a".to_string(),
            value: Box::new(Expression::Literal(Literal::Number(1))),
        },
        Expression::If {
            condition: Box::new(Expression::BinOp {
                op: BinaryOperator::Equals,
                left_side: Box::new(
                    Expression::Variable("var_a".to_string()
                )),
                right_side: Box::new(
                    Expression::Literal(Literal::Number(1)
                )),
            }),
            if_branch: Box::new(
                Expression::Print(
                    Box::new(Expression::Literal(
                        Literal::String("This is gonna work".to_string())
                    ))
                )
            ),
            else_branch: Box::new(
                Some(Expression::Print(
                    Box::new(Expression::Literal(
                        Literal::String("This ain't Working".to_string())
                    ))
                ))
            ),
        }
    ]
}

pub fn construct(tokens: Vec<Token>) -> Result<AST, Error> {
    let ast = tokens
        .split(|token| token == &Token::EndStatement)
        .filter(|tokens| !tokens.is_empty())
        .map(|tokens| {
            let mut tokens = tokens.to_vec();
            tokens.reverse();
            construct_expression(&mut tokens).unwrap()
        })
        .collect();
    Ok(ast)
}

fn get_next_token(tokens: &mut Vec<Token>) -> Result<Token, Error>{
    tokens.pop().ok_or(Error::UnexpectedEndOfTokens)
}

fn peek_next_token(tokens: &mut Vec<Token>) -> Result<&Token, Error>{
    tokens.get(0).ok_or(Error::UnexpectedEndOfTokens)
}

fn assert_take_token(tokens: &mut Vec<Token>, token: Token) -> Result<(), Error>{
    let next_token = peek_next_token(tokens)?;
    if are_variants_equal(next_token, &token) {
        Err(Error::UnexpectedToken)
    } else {
        Ok(())
    }
}

fn construct_expression(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    let first_token = match get_next_token(tokens)? {
        Token::Let => construct_assignment(tokens),
        Token::Print => construct_print(tokens),
        Token::If => construct_if(tokens),
        Token::LiteralBoolean(value) => Ok(Expression::Literal(Literal::Boolean(value))),
        Token::LiteralNumber(value) => Ok(Expression::Literal(Literal::Number(value))),
        Token::LiteralString(value) => Ok(Expression::Literal(Literal::String(value))),
        Token::Variable(name) => Ok(Expression::Variable(name)),
        token => {
            println!("Unexpected Token: {:?}", token);
            Err(Error::UnexpectedToken)?
        },
    };

    if tokens.is_empty() {
        first_token
    } else {
        let op_token = tokens.pop().unwrap();
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
            Ok(Expression::BinOp {
                op,
                left_side: Box::new(first_token?),
                right_side: Box::new(construct_expression(tokens)?),
            })
        } else {
            first_token
        }
    }
}

fn construct_assignment(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    match get_next_token(tokens)? {
        Token::Variable(name) => {
            if !(matches!(get_next_token(tokens)?, Token::Assignment)) {
                Err(Error::UnexpectedToken)?
            }

            let value = Box::new(construct_expression(tokens)?);
            Ok(Expression::Assignment { name, value })
        },
        _ => Err(Error::UnexpectedToken)?,
    }
}

fn construct_print(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    let expr = construct_expression(tokens)?;
    Ok(Expression::Print(
        Box::new(expr)
    ))
}

fn construct_if(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    assert_take_token(tokens, Token::If)?;
    let condition = construct_expression(tokens)?;
    assert_take_token(tokens, Token::BeginBlock)?;
    let if_branch = construct_expression(tokens)?;
    assert_take_token(tokens, Token::EndBlock)?;

    Ok(Expression::If {
        condition: Box::new(condition),
        if_branch: Box::new(if_branch),
        else_branch: Box::new(None)
    })
}

fn are_variants_equal(a: &Token, b: &Token) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
