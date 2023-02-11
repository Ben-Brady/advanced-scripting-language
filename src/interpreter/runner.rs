use super::{binop, Value, Context, Error, Type};
use crate::parser::{Block, Expression, Literal, BinaryOperator};


pub fn run(ast: Block) -> Result<String, Error> {
    let mut context = Context::new();
    for statement in ast {
        eval_expression(&mut context, statement)?;
    }

    Ok(context.output)
}

fn eval_expression(ctx: &mut Context, expression: Expression) -> Result<Value, Error> {
    match expression {
        Expression::Block(block) => eval_block(ctx, *block),
        Expression::Print(expression) => eval_print(ctx, *expression),
        Expression::Literal(literal) => eval_literal(ctx, literal),
        Expression::If { condition, if_branch, else_branch } => eval_if(ctx, *condition, *if_branch, *else_branch),
        Expression::Variable(var_name) => eval_variable(ctx, var_name),
        Expression::BinOp { op, left_side, right_side } => eval_binary_operator(ctx, op, *left_side, *right_side),
        Expression::Assignment { name, value } => eval_assignment(ctx, name, *value),
        Expression::Declaration { name, value } => eval_declaration(ctx, name, *value),
    }
}

fn eval_block(ctx: &mut Context, block: Block) -> Result<Value, Error> {
    let mut last_expr = Value::None;
    for expression in block {
        last_expr = eval_expression(ctx, expression)?;
    }
    Ok(last_expr)
}

fn eval_print(ctx: &mut Context, expression: Expression) -> Result<Value, Error> {
    let value = eval_expression(ctx, expression)?;
    let output = match value {
        Value::Number(value) => format!("{}\n", value),
        Value::Boolean(value) => format!("{}\n", value),
        Value::String(value) => format!("{}\n", value),
        Value::None => "None\n".to_string(),
    };
    ctx.output.push_str(output.as_str());
    Ok(Value::None)
}

fn eval_literal(_ctx: &mut Context, literal: Literal) -> Result<Value, Error> {
    match literal {
        Literal::Number(number) => Ok(Value::Number(number)),
        Literal::String(string) => Ok(Value::String(string)),
        Literal::Boolean(boolean) => Ok(Value::Boolean(boolean)),
    }
}

fn eval_if(ctx: &mut Context, condition: Expression, if_branch: Expression, else_branch: Option<Expression>) -> Result<Value, Error> {
    let cond = eval_expression(ctx, condition)?;
    let condition = match cond {
        Value::Boolean(value) => Ok(value),
        value => Err(Error::InvalidType{
            expected: Type::Boolean,
            actual: value,
        })
    }?;

    if condition {
        eval_expression(ctx, if_branch)
    } else if let Some(else_branch) = else_branch {
        eval_expression(ctx, else_branch)
    } else {
        Ok(Value::None)
    }
}

fn eval_variable(context: &mut Context, name: String) -> Result<Value, Error> {
    if let Ok(value) = context.get_variable(name.clone()) {
        Ok(value)
    } else {
        Err(Error::VariableDoesNotExist(name))
    }
}

fn eval_assignment(context: &mut Context, name: String, expression: Expression) -> Result<Value, Error> {
    let value = eval_expression(context, expression)?;
    context.assign_variable(name, value).unwrap();
    Ok(Value::None)
}

fn eval_declaration(context: &mut Context, name: String, expression: Expression) -> Result<Value, Error> {
    let value = eval_expression(context, expression)?;
    context.declare_variable(name, value).unwrap();
    Ok(Value::None)
}

fn eval_binary_operator(
        context: &mut Context,
        op: BinaryOperator,
        left_side: Expression,
        right_side: Expression,
    ) -> Result<Value, Error> {
    let left_value = eval_expression(context, left_side)?;
    let right_value = eval_expression(context, right_side)?;

    if !are_variants_equal(&left_value, &right_value) {
        return Err(Error::UnsupportedOperation(
            op,
            left_value,
            right_value
        ));
    }
    
    match (left_value, right_value) {
        (Value::String(left), Value::String(right)) => {
            binop::string_bin_op(&op, left, right)
        },
        (Value::Number(left), Value::Number(right)) => {
            binop::number_bin_op(&op, left, right)
        },
        (Value::Boolean(left), Value::Boolean(right)) => {
            binop::boolean_bin_op(&op, left, right)
        },
        (left, right) => {
            Err(Error::UnsupportedOperation(op, left, right))
        },
    }

}

fn are_variants_equal<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}