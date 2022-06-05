use super::{
    environment::Environment,
    parser::{Declaration, Expression, LiteralValue, Operator, Program, Statement},
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new(environment: Environment) -> Self {
        Self {
            environment,
        }
    }

    pub fn run(&mut self, program: &Program) {
        self.evaluate_declarations(program.get_declarations());
    }

    pub fn evaluate_declarations(&mut self, declarations: &Vec<Declaration>) {
        for declaration in declarations.iter() {
            self.evaluate_declaration(declaration);
        }
    }

    fn evaluate_declaration(&mut self, declaration: &Declaration) {
        match declaration {
            Declaration::VariableAssignment { identifier, value } => {
                self.environment
                    .assign(identifier, self.evaluate_expression(value));
            },
            Declaration::Statement(statement) => {
                self.evaluate_statement(statement);
            },
            Declaration::Block(block) => {
                let declarations = block.get_declarations();
                self.evaluate_declarations(declarations);
            },
        }
    }

    fn evaluate_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::If { condition, declaration } => self.if_statement(condition, declaration),
            Statement::Print(expression) => self.print(expression),
            Statement::Expression(expression) => self.evaluate_expression_statement(expression),
        }
    }

    fn if_statement(&mut self, condition: &Expression, declaration: &Declaration) {
        let condition_value = self.evaluate_expression(condition);

        match condition_value {
            LiteralValue::Boolean(value) => if value {
                self.evaluate_declaration(declaration);
            },
            LiteralValue::String(value) => if !value.eq("") {
                self.evaluate_declaration(declaration);
            },
            LiteralValue::Number(value) => if value != 0.0 {
                self.evaluate_declaration(declaration);
            },
            LiteralValue::Identifier(_) => panic!("Unexpected unresolved identifier"),
            LiteralValue::Nil => {},
        }
    }

    fn print(&self, expression: &Expression) {
        let result = self.evaluate_expression(expression);
        println!("{}", result.to_string())
    }

    fn evaluate_expression_statement(&self, expression: &Expression) {
        let result = self.evaluate_expression(expression);
        print!("{result:?}\n");
    }

    fn evaluate_expression(&self, expression: &Expression) -> LiteralValue {
        match expression {
            Expression::Binary {
                left,
                right,
                operator,
            } => self.evaluate_binary_expression(left, right, operator),
            Expression::Grouping(expression) => self.evaluate_expression(expression),
            Expression::Literal(LiteralValue::Identifier(identifier)) => {
                self.environment.resolve(identifier).clone()
            }
            Expression::Literal(literal_value) => literal_value.clone(),
            Expression::Unary { right, operator } => {
                self.evaluate_unary_expression(right, operator)
            }
        }
    }

    fn evaluate_binary_expression(
        &self,
        left: &Expression,
        right: &Expression,
        operator: &Operator,
    ) -> LiteralValue {
        let left_value = self.evaluate_expression(left);
        let right_value = self.evaluate_expression(right);

        match operator {
            Operator::BangEquals => LiteralValue::Boolean(left_value != right_value),
            Operator::EqualsEquals => LiteralValue::Boolean(left_value == right_value),
            Operator::Greater => LiteralValue::Boolean(left_value > right_value),
            Operator::GreaterEqual => LiteralValue::Boolean(left_value >= right_value),
            Operator::Less => LiteralValue::Boolean(left_value < right_value),
            Operator::LessEqual => LiteralValue::Boolean(left_value > right_value),
            Operator::Minus => left_value - right_value,
            Operator::Plus => left_value + right_value,
            Operator::Slash => left_value / right_value,
            Operator::Star => left_value * right_value,
            Operator::Bang => panic!("Invalid binary operator"),
        }
    }

    fn evaluate_unary_expression(&self, right: &Expression, operator: &Operator) -> LiteralValue {
        match operator {
            Operator::BangEquals => panic!("Invalid unary operator"),
            Operator::EqualsEquals => panic!("Invalid unary operator"),
            Operator::Greater => panic!("Invalid unary operator"),
            Operator::GreaterEqual => panic!("Invalid unary operator"),
            Operator::Less => panic!("Invalid unary operator"),
            Operator::LessEqual => panic!("Invalid unary operator"),
            Operator::Minus => match self.evaluate_expression(right) {
                LiteralValue::Boolean(_) => panic!("Boolean values cannot be negated"),
                LiteralValue::String(_) => panic!("String values cannot be negated"),
                LiteralValue::Number(value) => LiteralValue::Number(-value.clone()),
                LiteralValue::Nil => panic!("Nil values cannot be negated"),
                LiteralValue::Identifier(_) => panic!("Unexpected unresolved identifier"),
            },
            Operator::Plus => self.evaluate_expression(right),
            Operator::Slash => panic!("Invalid unary operator"),
            Operator::Star => panic!("Invalid unary operator"),
            Operator::Bang => match self.evaluate_expression(right) {
                LiteralValue::Boolean(value) => LiteralValue::Boolean(!value),
                LiteralValue::String(value) => LiteralValue::Boolean(value.len() == 0),
                LiteralValue::Number(value) => LiteralValue::Boolean(value == 0.0),
                LiteralValue::Nil => LiteralValue::Boolean(true),
                LiteralValue::Identifier(_) => panic!("Unexpected unresolved identifier"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::environment::Environment;
    use crate::lib::parser::Parser;
    use crate::lib::scanner::Scanner;
    use crate::{expr, tokens};

    use super::*;

    #[test]
    fn one_equals_equals_one() {
        let expression = expr!("1==1");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(true)");
    }

    #[test]
    fn one_equals_equals_two() {
        let expression = expr!("1==2");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn one_equals_equals_true() {
        let expression = expr!("1==true");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn one_bang_equals_one() {
        let expression = expr!("1!=1");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn one_bang_equals_two() {
        let expression = expr!("1!=2");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(true)");
    }

    #[test]
    fn one_greater_two() {
        let expression = expr!("1>2");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn string_star_number() {
        let expression = expr!("\"Hello \"*3");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "String(\"Hello Hello Hello \")");
    }

    #[test]
    fn string_star_negative_number() {
        let expression = expr!("\"Hello \"*-3");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "String(\"\")");
    }

    #[test]
    fn string_star_float() {
        let expression = expr!("\"Hello \"*3.9");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "String(\"Hello Hello Hello \")");
    }

    #[test]
    fn complex_expression() {
        let expression = expr!("!false == 5 > (1 - 2 + 5 / 2) * 100 - 10");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn regression_number_multiply_string() {
        let expression = expr!("3*\"Hello \"");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "String(\"Hello Hello Hello \")");
    }

    #[test]
    fn regression_divison_order() {
        let expression = expr!("1+2/4");
        let interpreter = Interpreter {
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Number(1.5)");
    }
}
