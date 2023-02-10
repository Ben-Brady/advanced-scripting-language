use super::Value;
use crate::ast::{BinaryOperator};


pub fn string_bin_op(op: &BinaryOperator, left: String, right: String) -> Value {
    match op {
        BinaryOperator::Equals => Value::Boolean(left == right),
        BinaryOperator::NotEquals => Value::Boolean(left != right),
        BinaryOperator::Addition => {
            let mut output = left;
            output.push_str(right.as_str());
            Value::String(output)
        },
        _ => {
            panic!("Unsupported Operation");
        },
    }
}

pub fn number_bin_op(op: &BinaryOperator, left: usize, right: usize) -> Value {
    match op {
        BinaryOperator::Equals => Value::Boolean(left == right),
        BinaryOperator::NotEquals => Value::Boolean(left != right),
        BinaryOperator::GreaterThan => Value::Boolean(left > right),
        BinaryOperator::GreaterThanOrEqual => Value::Boolean(left >= right),
        BinaryOperator::LessThan => Value::Boolean(left < right),
        BinaryOperator::LessThanOrEqual => Value::Boolean(left <= right),

        BinaryOperator::Addition => Value::Number(left + right),
        BinaryOperator::Subtraction => Value::Number(left - right),
        BinaryOperator::Division => Value::Number(left / right),
        BinaryOperator::Multiplication => Value::Number(left * right),
        BinaryOperator::Exponent => Value::Number(left.pow(right.try_into().unwrap())),
        BinaryOperator::Modulus => Value::Number(left % right),
        _ => {
            panic!("Unsupported Operation");
        },
    }
}

pub fn boolean_bin_op(op: &BinaryOperator, left: bool, right: bool) -> Value {
    match op {
        BinaryOperator::Equals => Value::Boolean(left == right),
        BinaryOperator::NotEquals => Value::Boolean(left != right),
        BinaryOperator::And => Value::Boolean(left && right),
        BinaryOperator::Or => Value::Boolean(left || right),
        _ => {
            panic!("Unsupported Operation");
        },
    }
}
