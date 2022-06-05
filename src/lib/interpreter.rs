use super::{parser::{Expression, LiteralValue, Operator, Statement, Program, Declaration}, environment::Environment};

pub struct Interpreter {
    environment: Environment,
    program: Program,
    current: usize,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Self {
            environment: Environment::new(),
            program,
            current: 0,
        }
    }

    pub fn run(&mut self) {
        while self.current < self.program.len() {
            self.evaluate_declaration();
            self.current += 1;
        }
    }

    pub fn evaluate_declarations(&mut self, declarations: &mut Vec<Declaration>) {
        self.program.add_declarations(declarations);
        self.run();
    }

    fn evaluate_declaration(&mut self) {
        match self.program.get(self.current) {
            Some(Declaration::VariableAssignment { identifier, value }) => {
                self.environment.assign(identifier, self.evaluate_expression(value));
            },
            Some(Declaration::Statement(statement)) => {
                self.evaluate_statement(statement);
            },
            None => {},
        }
    }

    fn evaluate_statement(&self, statement: &Statement) {
        match statement {
            Statement::Print(expression) => self.print(expression),
            Statement::Expression(expression) => self.evaluate_expression_statement(expression),
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
            } => {
                self.evaluate_binary_expression(left, right, operator)
            }
            Expression::Grouping(expression) => {
                self.evaluate_expression(expression)
            },
            Expression::Literal(LiteralValue::Identifier(identifier)) => {
                self.environment.resolve(identifier).clone()
            }
            Expression::Literal(literal_value) => {
                literal_value.clone()
            },
            Expression::Unary { right, operator } => {
                self.evaluate_unary_expression(right, operator)
            },
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

    fn evaluate_unary_expression(
        &self,
        right: &Expression,
        operator: &Operator,
    ) -> LiteralValue {
        match operator {
            Operator::BangEquals => panic!("Invalid unary operator"),
            Operator::EqualsEquals => panic!("Invalid unary operator"),
            Operator::Greater => panic!("Invalid unary operator"),
            Operator::GreaterEqual => panic!("Invalid unary operator"),
            Operator::Less => panic!("Invalid unary operator"),
            Operator::LessEqual => panic!("Invalid unary operator"),
            Operator::Minus => {
                match self.evaluate_expression(right) {
                    LiteralValue::Boolean(_) => panic!("Boolean values cannot be negated"),
                    LiteralValue::String(_) => panic!("String values cannot be negated"),
                    LiteralValue::Number(value) => LiteralValue::Number(-value.clone()),
                    LiteralValue::Nil => panic!("Nil values cannot be negated"),
                    LiteralValue::Identifier(identifier) => panic!("Unexpected unresolved identifier"),
                }
            },
            Operator::Plus => self.evaluate_expression(right),
            Operator::Slash => panic!("Invalid unary operator"),
            Operator::Star => panic!("Invalid unary operator"),
            Operator::Bang => {
                match self.evaluate_expression(right) {
                    LiteralValue::Boolean(value) => LiteralValue::Boolean(!value),
                    LiteralValue::String(value) => LiteralValue::Boolean(value.len() == 0),
                    LiteralValue::Number(value) => LiteralValue::Boolean(value == 0.0),
                    LiteralValue::Nil => LiteralValue::Boolean(true),
                    LiteralValue::Identifier(identifier) => panic!("Unexpected unresolved identifier"),
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{expr, tokens};
    use crate::lib::parser::Parser;
    use crate::lib::scanner::Scanner;
    use crate::lib::environment::Environment;

    use super::*;

    #[test]
    fn one_equals_equals_one() {
        let expression = expr!("1==1");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(true)");
    }

    #[test]
    fn one_equals_equals_two() {
        let expression = expr!("1==2");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn one_equals_equals_true() {
        let expression = expr!("1==true");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn one_bang_equals_one() {
        let expression = expr!("1!=1");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn one_bang_equals_two() {
        let expression = expr!("1!=2");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(true)");
    }

    #[test]
    fn one_greater_two() {
        let expression = expr!("1>2");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn string_star_number() {
        let expression = expr!("\"Hello \"*3");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "String(\"Hello Hello Hello \")");
    }

    #[test]
    fn string_star_negative_number() {
        let expression = expr!("\"Hello \"*-3");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "String(\"\")");
    }

    #[test]
    fn string_star_float() {
        let expression = expr!("\"Hello \"*3.9");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "String(\"Hello Hello Hello \")");
    }

    #[test]
    fn complex_expression() {
        let expression = expr!("!false == 5 > (1 - 2 + 5 / 2) * 100 - 10");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Boolean(false)");
    }

    #[test]
    fn regression_number_multiply_string() {
        let expression = expr!("3*\"Hello \"");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "String(\"Hello Hello Hello \")");
    }

    #[test]
    fn regression_divison_order() {
        let expression = expr!("1+2/4");
        let interpreter = Interpreter {
            current: 0,
            program: Program::new(),
            environment: Environment::new(),
        };

        let result = Interpreter::evaluate_expression(&interpreter, &expression);

        assert_eq!(format!("{result:?}"), "Number(1.5)");
    }
}
