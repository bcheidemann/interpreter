use std::ops::{Add, Div, Mul, Sub};

use super::scanner::{Keyword, Token, TokenDirection, Tokens};

#[derive(Debug)]
pub enum Operator {
    BangEquals,
    EqualsEquals,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    Plus,
    Slash,
    Star,
    Bang,
}

impl From<&Token> for Operator {
    fn from(token: &Token) -> Self {
        match token {
            Token::BangEquals => Operator::BangEquals,
            Token::EqualsEquals => Operator::EqualsEquals,
            Token::Greater => Operator::Greater,
            Token::GreaterEqual => Operator::GreaterEqual,
            Token::Less => Operator::Less,
            Token::LessEqual => Operator::LessEqual,
            Token::Minus => Operator::Minus,
            Token::Plus => Operator::Plus,
            Token::Slash => Operator::Slash,
            Token::Star => Operator::Star,
            Token::Bang => Operator::Bang,
            token => panic!("Expected a operator token not {token:?}"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LiteralValue {
    Boolean(bool),
    String(String),
    Number(f32),
    Nil,
}

impl Sub for LiteralValue {
    type Output = LiteralValue;

    fn sub(self, rhs: LiteralValue) -> Self::Output {
        match self {
            LiteralValue::Boolean(_) => panic!("Cannot subtract boolean values"),
            LiteralValue::String(_) => panic!("Cannot subtract string values"),
            LiteralValue::Number(lhs_value) => match rhs {
                LiteralValue::Number(rhs_value) => LiteralValue::Number(rhs_value - lhs_value),
                _ => panic!("Cannot subtract values with different types"),
            },
            LiteralValue::Nil => panic!("Cannot subtract nil values"),
        }
    }
}

impl Add for LiteralValue {
    type Output = LiteralValue;

    fn add(self, rhs: LiteralValue) -> Self::Output {
        match self {
            LiteralValue::Boolean(_) => panic!("Cannot add boolean values"),
            LiteralValue::String(lhs_value) => match rhs {
                LiteralValue::String(rhs_value) => LiteralValue::String(rhs_value + &lhs_value),
                _ => panic!("Cannot add values with different types"),
            },
            LiteralValue::Number(lhs_value) => match rhs {
                LiteralValue::Number(rhs_value) => LiteralValue::Number(rhs_value + lhs_value),
                _ => panic!("Cannot add values with different types"),
            },
            LiteralValue::Nil => panic!("Cannot add nil values"),
        }
    }
}

impl Div for LiteralValue {
    type Output = LiteralValue;

    fn div(self, rhs: LiteralValue) -> Self::Output {
        match self {
            LiteralValue::Boolean(_) => panic!("Cannot divide boolean values"),
            LiteralValue::String(_) => panic!("Cannot divide string values"),
            LiteralValue::Number(lhs_value) => match rhs {
                LiteralValue::Number(rhs_value) => LiteralValue::Number(rhs_value / lhs_value),
                _ => panic!("Cannot divide values with different types"),
            },
            LiteralValue::Nil => panic!("Cannot divide nil values"),
        }
    }
}

impl Mul for LiteralValue {
    type Output = LiteralValue;

    fn mul(self, rhs: LiteralValue) -> Self::Output {
        match self {
            LiteralValue::Boolean(_) => panic!("Cannot multiply boolean values"),
            LiteralValue::String(lhs_value) => match rhs {
                LiteralValue::Number(rhs_value) => {
                    LiteralValue::String(lhs_value.repeat(rhs_value as usize))
                }
                _ => panic!("Strings can only be multiplied by a number"),
            },
            LiteralValue::Number(lhs_value) => match rhs {
                LiteralValue::Number(rhs_value) => LiteralValue::Number(rhs_value / lhs_value),
                _ => panic!("Cannot multiply values with different types"),
            },
            LiteralValue::Nil => panic!("Cannot multiply nil values"),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: Operator,
    },
    Grouping(Box<Expression>),
    Literal(LiteralValue),
    Unary {
        right: Box<Expression>,
        operator: Operator,
    },
}

pub struct Parser<'a> {
    tokens: &'a Tokens,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn parse_expr_from_tokens(tokens: &'a Tokens) -> Expression {
        let mut parser = Self { tokens, current: 0 };
        parser.parse()
    }

    pub fn new(tokens: &'a Tokens) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expression {
        self.expression()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_then_advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.tokens.get(self.current - 1)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while matches!(self.peek(), Some(Token::BangEquals | Token::EqualsEquals)) {
            let operator: Operator = self.peek().unwrap().into();
            self.advance();
            let right = Box::new(self.comparison());
            let left = Box::new(expr);

            expr = Expression::Binary {
                left,
                right,
                operator: operator.into(),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        while matches!(
            self.peek(),
            Some(Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual)
        ) {
            let operator: Operator = self.peek().unwrap().into();
            self.advance();
            let right = Box::new(self.term());
            let left = Box::new(expr);

            let _result = format!("{left:?}, {operator:?}, {right:?}");

            expr = Expression::Binary {
                left,
                right,
                operator: operator.into(),
            };
        }

        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while matches!(self.peek(), Some(Token::Minus | Token::Plus)) {
            let operator: Operator = self.peek().unwrap().into();
            self.advance();
            let right = Box::new(self.factor());
            let left = Box::new(expr);

            expr = Expression::Binary {
                left,
                right,
                operator: operator.into(),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while matches!(self.peek(), Some(Token::Slash | Token::Star)) {
            let operator: Operator = self.peek().unwrap().into();
            self.advance();
            let right = Box::new(self.unary());
            let left = Box::new(expr);

            expr = Expression::Binary {
                left,
                right,
                operator: operator.into(),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        let current = self.peek();
        if matches!(current, Some(Token::Bang | Token::Minus | Token::Plus)) {
            let operator: Operator = current.unwrap().into();
            self.advance();
            let right = Box::new(self.unary());

            return Expression::Unary { right, operator };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expression {
        match self.peek_then_advance() {
            Some(Token::Keyword(Keyword::False)) => {
                Expression::Literal(LiteralValue::Boolean(false))
            }
            Some(Token::Keyword(Keyword::True)) => Expression::Literal(LiteralValue::Boolean(true)),
            Some(Token::Keyword(Keyword::Nil)) => Expression::Literal(LiteralValue::Nil),
            Some(Token::Number(number)) => {
                Expression::Literal(LiteralValue::Number(number.clone()))
            }
            Some(Token::String(string)) => Expression::Literal(LiteralValue::String(
                string[1..string.len() - 1].to_string(),
            )),
            Some(Token::Paren(TokenDirection::Left)) => {
                let expr = self.expression();
                match self.peek() {
                    Some(Token::Paren(TokenDirection::Right)) => {
                        Expression::Grouping(Box::new(expr))
                    }
                    _ => panic!("Expected ')' after expression"),
                }
            }
            None => panic!("TODO: Handle EOF"),
            _ => panic!("Syntax error??"),
        }
    }
}

#[macro_export]
macro_rules! expr {
    ($source:expr) => {
        Parser::parse_expr_from_tokens(&tokens!($source).unwrap())
    };
}

#[cfg(test)]
mod tests {
    use crate::{lib::scanner::Scanner, tokens};

    use super::*;

    #[test]
    fn expr_macro() {
        let expression = expr!("true");

        assert_eq!(format!("{expression:?}"), "Literal(Boolean(true))");
    }

    #[test]
    fn boolean() {
        let tokens = tokens!("true").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse();

        assert_eq!(format!("{result:?}"), "Literal(Boolean(true))");
    }

    #[test]
    fn grouping() {
        let tokens = tokens!("(true)").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse();

        assert_eq!(format!("{result:?}"), "Grouping(Literal(Boolean(true)))");
    }

    #[test]
    fn grouping_comparison() {
        let tokens = tokens!("(true < false)").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse();

        assert_eq!(format!("{result:?}"), "Grouping(Binary { left: Literal(Boolean(true)), right: Literal(Boolean(false)), operator: Less })");
    }

    #[test]
    fn comparison() {
        let tokens = tokens!("123 > 321").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse();

        assert_eq!(format!("{result:?}"), "Binary { left: Literal(Number(123.0)), right: Literal(Number(321.0)), operator: Greater }");
    }

    #[test]
    fn not_negative_number() {
        let tokens = tokens!("!-99").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse();

        assert_eq!(format!("{result:?}"), "Unary { right: Unary { right: Literal(Number(99.0)), operator: Minus }, operator: Bang }");
    }

    #[test]
    fn complex_expression() {
        let tokens =
            tokens!("123 * 2 - 456 < 42 + 99").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse();

        assert_eq!(format!("{result:?}"), "Binary { left: Binary { left: Binary { left: Literal(Number(123.0)), right: Literal(Number(2.0)), operator: Star }, right: Literal(Number(456.0)), operator: Minus }, right: Binary { left: Literal(Number(42.0)), right: Literal(Number(99.0)), operator: Plus }, operator: Less }");
    }
}
