use std::collections::HashMap;
use crate::ast::{AST, Expression, Literal, BinaryOperator};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Number(usize),
    Boolean(bool),
    String(String),
    None
}

pub fn run(ast: AST) -> String {
    let mut context = Context::new();
    for statement in ast {
        context.eval_expression(statement);
    }

    context.output
}

struct Context {
    variables: HashMap<String, Value>,
    output: String,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables: HashMap::new(),
            output: String::new(),
        }
    }
}

impl Context {
    pub fn eval_expression(&mut self, expression: Expression) -> Value {
        match expression {
            Expression::Print(expression) => self.eval_print(*expression),
            Expression::Literal(literal) => self.eval_literal(literal),
            Expression::If { condition, if_branch, else_branch } => self.eval_if(*condition, *if_branch, *else_branch),
            Expression::Variable(var_name) => self.eval_variable(var_name),
            Expression::BinOp { op, left_side, right_side } => self.eval_binary_operator(op, *left_side, *right_side),
            Expression::Assignment { name, value } => self.eval_assignment(name, *value),
        }
    }
    
    fn eval_print(&mut self, expression: Expression) -> Value {
        let output = match self.eval_expression(expression) {
            Value::Number(value) => format!("{}\n", value),
            Value::Boolean(value) => format!("{}\n", value),
            Value::String(value) => format!("{}\n", value),
            Value::None => "None\n".to_string(),
        };
        self.output.push_str(output.as_str());
        Value::None
    }
    
    fn eval_literal(&mut self, literal: Literal) -> Value {
        match literal {
            Literal::Number(number) => Value::Number(number),
            Literal::String(string) => Value::String(string),
            Literal::Boolean(boolean) => Value::Boolean(boolean),
        }
    }
    
    fn eval_if(&mut self, condition: Expression, if_branch: Expression, else_branch: Option<Expression>) -> Value {
        let cond = self.eval_expression(condition);
        if !matches!(cond, Value::Boolean(_)) {
            panic!("Invalid Type")
        }
    
        if cond == Value::Boolean(true) {
            self.eval_expression(if_branch)
        } else if let Some(else_branch) = else_branch {
            self.eval_expression(else_branch)
        } else {
            Value::None
        }
    }

    fn eval_variable(&mut self, name: String) -> Value {
        if let Some(value) = self.variables.get(&name) {
            value.clone()
        } else {
            panic!("Varaible Does not exist")
        }
    }

    fn eval_assignment(&mut self, name: String, expression: Expression) -> Value {
        let value = self.eval_expression(expression);
        self.variables.insert(name, value);
        Value::None
    }

    fn eval_binary_operator(
            &mut self,
            op: BinaryOperator,
            left_side: Expression,
            right_side: Expression,
        ) -> Value {
        let left_value = self.eval_expression(left_side);
        let right_value = self.eval_expression(right_side);
        if !are_variants_equal(&left_value, &right_value) {
            panic!("Variants are not equal");
        }

        if matches!(left_value, Value::None) {
            panic!("Cannot Perform Binary Operation on None");
        }

        match (left_value, right_value) {
            (Value::String(left), Value::String(right)) => {
                string_bin_op(&op, left, right)
            },
            (Value::Number(left), Value::Number(right)) => {
                number_bin_op(&op, left, right)
            },
            (Value::Boolean(left), Value::Boolean(right)) => {
                boolean_bin_op(&op, left, right)
            },
            _ => {
                panic!("Unsupported Operation");
            },
        }
    }
}


fn string_bin_op(op: &BinaryOperator, left: String, right: String) -> Value {
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

fn number_bin_op(op: &BinaryOperator, left: usize, right: usize) -> Value {
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

fn boolean_bin_op(op: &BinaryOperator, left: bool, right: bool) -> Value {
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

fn are_variants_equal<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}