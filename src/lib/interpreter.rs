use super::parser::{Expression, LiteralValue, Operator};

pub fn evaluate_binary_expression(
    left: Expression,
    right: Expression,
    operator: Operator,
) -> LiteralValue {
    let left_value = evaluate_expression(left);
    let right_value = evaluate_expression(right);

    match operator {
        Operator::BangEquals => {
            LiteralValue::Boolean(left_value != right_value)
        },
        Operator::EqualsEquals => {
            LiteralValue::Boolean(left_value == right_value)
        },
        Operator::Greater => {
            LiteralValue::Boolean(left_value > right_value)
        },
        Operator::GreaterEqual => {
            LiteralValue::Boolean(left_value >= right_value)
        },
        Operator::Less => {
            LiteralValue::Boolean(left_value < right_value)
        },
        Operator::LessEqual => {
            LiteralValue::Boolean(left_value > right_value)
        },
        Operator::Minus => {
            left_value - right_value
        },
        Operator::Plus => {
            left_value + right_value
        },
        Operator::Slash => {
            left_value / right_value
        },
        Operator::Star => {
            left_value * right_value
        },
        Operator::Bang => panic!("Invalid binary operator"),
    }
}

pub fn evaluate_unary_expression(
    right: Expression,
    operator: Operator,
) -> LiteralValue {
    match operator {
        Operator::BangEquals => panic!("Invalid unary operator"),
        Operator::EqualsEquals => panic!("Invalid unary operator"),
        Operator::Greater => panic!("Invalid unary operator"),
        Operator::GreaterEqual => panic!("Invalid unary operator"),
        Operator::Less => panic!("Invalid unary operator"),
        Operator::LessEqual => panic!("Invalid unary operator"),
        Operator::Minus => {
            match evaluate_expression(right) {
                LiteralValue::Boolean(_) => panic!("Boolean values cannot be negated"),
                LiteralValue::String(_) => panic!("String values cannot be negated"),
                LiteralValue::Number(value) => LiteralValue::Number(-value),
                LiteralValue::Nil => panic!("Nil values cannot be negated"),
            }
        },
        Operator::Plus => evaluate_expression(right),
        Operator::Slash => panic!("Invalid unary operator"),
        Operator::Star => panic!("Invalid unary operator"),
        Operator::Bang => {
            match evaluate_expression(right) {
                LiteralValue::Boolean(value) => LiteralValue::Boolean(!value),
                LiteralValue::String(value) => LiteralValue::Boolean(value.len() == 0),
                LiteralValue::Number(value) => LiteralValue::Boolean(value == 0.0),
                LiteralValue::Nil => LiteralValue::Boolean(true),
            }
        },
    }
}

pub fn evaluate_expression(expression: Expression) -> LiteralValue {
    match expression {
        Expression::Binary {
            left,
            right,
            operator,
        } => {
            evaluate_binary_expression(*left, *right, operator)
        }
        Expression::Grouping(expression) => {
            evaluate_expression(*expression)
        },
        Expression::Literal(literal_value) => {
            literal_value
        },
        Expression::Unary { right, operator } => {
            evaluate_unary_expression(*right, operator)
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{expr, tokens};
    use crate::lib::parser::Parser;
    use crate::lib::scanner::Scanner;

    use super::*;

    #[test]
    fn one_equals_equals_one() {
        let expression = expr!("1==1");

        let result = evaluate_expression(expression);

        assert_eq!(format!("{result:?}"), "Boolean(true)");
    }

    #[test]
    fn one_equals_equals_two() {
        let expression = expr!("1==2");

        let result = evaluate_expression(expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn one_equals_equals_true() {
        let expression = expr!("1==true");

        let result = evaluate_expression(expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn one_bang_equals_one() {
        let expression = expr!("1!=1");

        let result = evaluate_expression(expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn one_bang_equals_two() {
        let expression = expr!("1!=2");

        let result = evaluate_expression(expression);

        assert_eq!(format!("{result:?}"), "Boolean(true)");
    }

    #[test]
    fn one_greater_two() {
        let expression = expr!("1>2");

        let result = evaluate_expression(expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn string_star_number() {
        let expression = expr!("\"Hello \"*3");

        let result = evaluate_expression(expression);

        assert_eq!(format!("{result:?}"), "String(\"Hello Hello Hello \")");
    }

    #[test]
    fn string_star_negative_number() {
        let expression = expr!("\"Hello \"*-3");

        let result = evaluate_expression(expression);

        assert_eq!(format!("{result:?}"), "String(\"\")");
    }

    #[test]
    fn string_star_float() {
        let expression = expr!("\"Hello \"*3.9");

        let result = evaluate_expression(expression);

        assert_eq!(format!("{result:?}"), "String(\"Hello Hello Hello \")");
    }

    #[test]
    fn complex_expression() {
        let expression = expr!("!false == 5 > (1 - 2 + 5 / 2) * 100 - 10");

        let result = evaluate_expression(expression);

        assert_eq!(format!("{result:?}"), "Boolean(true)");
    }
}
