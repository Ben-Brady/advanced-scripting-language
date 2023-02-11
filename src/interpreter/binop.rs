use super::{Value, Error};
use crate::parser::{BinaryOperator};


pub fn string_bin_op(op: &BinaryOperator, left: String, right: String) -> Result<Value, Error> {
    match op {
        BinaryOperator::Equals => Ok(Value::Boolean(left == right)),
        BinaryOperator::NotEquals => Ok(Value::Boolean(left != right)),
        BinaryOperator::Addition => {
            let mut output = left;
            output.push_str(right.as_str());
            Ok(Value::String(output))
        },
        _ => {
            Err(Error::UnsupportedOperation(
                op.to_owned(),
                Value::String(left),
                Value::String(right),
            ))
        },
    }
}

pub fn number_bin_op(op: &BinaryOperator, left: usize, right: usize) -> Result<Value, Error> {
    match op {
        BinaryOperator::Equals => Ok(Value::Boolean(left == right)),
        BinaryOperator::NotEquals => Ok(Value::Boolean(left != right)),
        BinaryOperator::GreaterThan => Ok(Value::Boolean(left > right)),
        BinaryOperator::GreaterThanOrEqual => Ok(Value::Boolean(left >= right)),
        BinaryOperator::LessThan => Ok(Value::Boolean(left < right)),
        BinaryOperator::LessThanOrEqual => Ok(Value::Boolean(left <= right)),

        BinaryOperator::Addition => Ok(Value::Number(left + right)),
        BinaryOperator::Subtraction => Ok(Value::Number(left - right)),
        BinaryOperator::Division => Ok(Value::Number(left / right)),
        BinaryOperator::Multiplication => Ok(Value::Number(left * right)),
        BinaryOperator::Exponent => Ok(Value::Number(left.pow(right.try_into().unwrap()))),
        BinaryOperator::Modulus => Ok(Value::Number(left % right)),
        _ => {
            Err(Error::UnsupportedOperation(
                op.to_owned(),
                Value::Number(left),
                Value::Number(right),
            ))
        },
    }
}

pub fn boolean_bin_op(op: &BinaryOperator, left: bool, right: bool) -> Result<Value, Error> {
    match op {
        BinaryOperator::Equals => Ok(Value::Boolean(left == right)),
        BinaryOperator::NotEquals => Ok(Value::Boolean(left != right)),
        BinaryOperator::And => Ok(Value::Boolean(left && right)),
        BinaryOperator::Or => Ok(Value::Boolean(left || right)),
        _ => {
            Err(Error::UnsupportedOperation(
                op.to_owned(),
                Value::Boolean(left),
                Value::Boolean(right)
            ))
        },
    }
}
