use super::{Block, Expression, Literal, BinaryOperator};

pub fn demo() -> Block {
    vec![
        Expression::Declaration {
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
                Expression::Block(
                    Box::new(
                        vec![
                            Expression::Print(
                                Box::new(Expression::Literal(
                                    Literal::String("Hello".to_string())
                                )),
                            ),
                            Expression::Print(
                                Box::new(Expression::Literal(
                                    Literal::String("World".to_string())
                                )),
                            ),
                            
                        ]
                    )
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