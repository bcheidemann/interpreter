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

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum LiteralValue {
    Boolean(bool),
    String(String),
    Number(f32),
    Identifier(String),
    Nil,
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Boolean(value) => format!("{value}"),
            LiteralValue::String(value) => format!("{value}"),
            LiteralValue::Number(value) => format!("{value}"),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Identifier(identifier) => format!("{identifier}"),
        }
    }
}

impl Sub for LiteralValue {
    type Output = LiteralValue;

    fn sub(self, rhs: LiteralValue) -> Self::Output {
        match self {
            LiteralValue::Boolean(_) => panic!("Cannot subtract boolean values"),
            LiteralValue::String(_) => panic!("Cannot subtract string values"),
            LiteralValue::Number(lhs_value) => match rhs {
                LiteralValue::Number(rhs_value) => LiteralValue::Number(lhs_value - rhs_value),
                _ => panic!("Cannot subtract values with different types"),
            },
            LiteralValue::Nil => panic!("Cannot subtract nil values"),
            LiteralValue::Identifier(_) => panic!("Cannot subtract unresolved identifier"),
        }
    }
}

impl Add for LiteralValue {
    type Output = LiteralValue;

    fn add(self, rhs: LiteralValue) -> Self::Output {
        match self {
            LiteralValue::Boolean(lhs_value) => match rhs {
                LiteralValue::String(rhs_value) => {
                    LiteralValue::String(format!("{lhs_value}{rhs_value}"))
                }
                _ => panic!("Boolean values can only be added with string values"),
            },
            LiteralValue::String(lhs_value) => match rhs {
                LiteralValue::String(rhs_value) => LiteralValue::String(lhs_value + &rhs_value),
                LiteralValue::Boolean(rhs_value) => {
                    LiteralValue::String(format!("{lhs_value}{rhs_value}"))
                }
                LiteralValue::Number(rhs_value) => {
                    LiteralValue::String(format!("{lhs_value}{rhs_value}"))
                }
                LiteralValue::Identifier(_) => panic!("Cannot add unresolved identifier to string"),
                LiteralValue::Nil => LiteralValue::String(format!("{lhs_value}nil")),
            },
            LiteralValue::Number(lhs_value) => match rhs {
                LiteralValue::Number(rhs_value) => LiteralValue::Number(lhs_value + rhs_value),
                LiteralValue::String(rhs_value) => {
                    LiteralValue::String(format!("{lhs_value}{rhs_value}"))
                }
                _ => panic!("Cannot add values with different types"),
            },
            LiteralValue::Nil => match rhs {
                LiteralValue::String(rhs_value) => LiteralValue::String(format!("nil{rhs_value}")),
                _ => panic!("Nil values can only be added with string values"),
            },
            LiteralValue::Identifier(_) => panic!("Cannot add unresolved identifier"),
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
                LiteralValue::Number(rhs_value) => LiteralValue::Number(lhs_value / rhs_value),
                _ => panic!("Cannot divide values with different types"),
            },
            LiteralValue::Nil => panic!("Cannot divide nil values"),
            LiteralValue::Identifier(_) => panic!("Cannot divide unresolved identifier"),
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
                LiteralValue::Number(rhs_value) => LiteralValue::Number(lhs_value * rhs_value),
                LiteralValue::String(rhs_value) => {
                    LiteralValue::String(rhs_value.repeat(lhs_value as usize))
                }
                _ => panic!("Cannot multiply values with different types"),
            },
            LiteralValue::Nil => panic!("Cannot multiply nil values"),
            LiteralValue::Identifier(_) => panic!("Cannot multiply unresolved identifier"),
        }
    }
}

#[derive(Debug)]
pub struct Program(Vec<Declaration>);

impl Program {
    pub fn add_declaration(&mut self, declaration: Declaration) {
        self.0.push(declaration);
    }

    pub fn get_declarations(&self) -> &Vec<Declaration> {
        &self.0
    }
}

#[derive(Debug)]
pub enum Declaration {
    VariableAssignment {
        identifier: String,
        value: Expression,
    },
    Statement(Statement),
    Block(Block),
}

#[derive(Debug)]
pub struct Block(Vec<Declaration>);

impl Block {
    pub fn get_declarations(&self) -> &Vec<Declaration> {
        &self.0
    }
}

#[derive(Debug)]
pub enum Statement {
    Print(Expression),
    If { condition: Expression, declaration: Box<Declaration> },
    Expression(Expression),
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
    #[cfg(test)]
    pub fn parse_expr_from_tokens(tokens: &'a Tokens) -> Expression {
        let mut parser = Self { tokens, current: 0 };
        parser.parse_expression()
    }

    #[cfg(test)]
    pub fn parse_expression(&mut self) -> Expression {
        self.expression()
    }

    pub fn new(tokens: &'a Tokens) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Program {
        let mut program = Program(vec![]);

        while self.current < self.tokens.len() {
            program.add_declaration(self.declaration());
        }

        program
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_nth(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current + n)
    }

    fn peek_then_advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.tokens.get(self.current - 1)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn advance_nth(&mut self, n: usize) {
        self.current += n;
    }

    fn consume_semicolon(&mut self) {
        if matches!(self.peek(), Some(Token::SemiColon)) {
            self.advance();
        } else {
            panic!("Expected a semicolon");
        }
    }

    fn consume_brace(&mut self, direction: TokenDirection) {
        match direction {
            TokenDirection::Left => if matches!(self.peek(), Some(Token::Brace(TokenDirection::Left))) {
                self.advance()
            } else {
                panic!("Expected left brace")
            },
            TokenDirection::Right => if matches!(self.peek(), Some(Token::Brace(TokenDirection::Right))) {
                self.advance()
            } else {
                panic!("Expected right brace")
            },
        }
    }

    fn declaration(&mut self) -> Declaration {
        match self.peek() {
            Some(Token::Brace(TokenDirection::Left)) => Declaration::Block(self.block()),
            Some(Token::Identifier(_)) => self.identifier(),
            _ => self.statement_declaration(),
        }
    }

    fn block(&mut self) -> Block {
        self.consume_brace(TokenDirection::Left);
        
        let mut declarations = vec![];
        while !matches!(self.peek(), Some(Token::Brace(TokenDirection::Right))) {
            declarations.push(self.declaration());
        }
        self.consume_brace(TokenDirection::Right);

        Block(declarations)
    }

    fn identifier(&mut self) -> Declaration {
        self.variable_assignment()
    }

    fn variable_assignment(&mut self) -> Declaration {
        if let Some(Token::Identifier(identifier)) = self.peek() {
            let identifier = identifier.to_string();
            if let Some(Token::Equals) = self.peek_nth(1) {
                self.advance_nth(2);
                let assignment = Declaration::VariableAssignment {
                    identifier,
                    value: self.expression(),
                };
                self.consume_semicolon();
                return assignment;
            }
        }

        self.statement_declaration()
    }

    fn statement_declaration(&mut self) -> Declaration {
        Declaration::Statement(self.statement())
    }

    fn statement(&mut self) -> Statement {
        match self.peek() {
            Some(Token::Keyword(Keyword::Print)) => self.print(),
            Some(Token::Keyword(Keyword::If)) => self.if_statement(),
            _ => self.expression_statement(),
        }
    }

    fn print(&mut self) -> Statement {
        self.advance();
        let expr = self.expression();
        self.consume_semicolon();
        Statement::Print(expr)
    }

    fn if_statement(&mut self) -> Statement {
        self.advance();
        let condition = self.expression();
        let declaration = Box::new(self.declaration());
        Statement::If { condition, declaration }
    }

    fn expression_statement(&mut self) -> Statement {
        let expr = self.expression();
        self.consume_semicolon();
        Statement::Expression(expr)
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
            Some(Token::Identifier(identifier)) => {
                Expression::Literal(LiteralValue::Identifier(identifier.to_string()))
            }
            Some(Token::Paren(TokenDirection::Left)) => {
                let expr = self.expression();
                match self.peek_then_advance() {
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

#[cfg(test)]
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

        let result = parser.parse_expression();

        assert_eq!(format!("{result:?}"), "Literal(Boolean(true))");
    }

    #[test]
    fn grouping() {
        let tokens = tokens!("(true)").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_expression();

        assert_eq!(format!("{result:?}"), "Grouping(Literal(Boolean(true)))");
    }

    #[test]
    fn grouping_comparison() {
        let tokens = tokens!("(true < false)").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_expression();

        assert_eq!(format!("{result:?}"), "Grouping(Binary { left: Literal(Boolean(true)), right: Literal(Boolean(false)), operator: Less })");
    }

    #[test]
    fn comparison() {
        let tokens = tokens!("123 > 321").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_expression();

        assert_eq!(format!("{result:?}"), "Binary { left: Literal(Number(123.0)), right: Literal(Number(321.0)), operator: Greater }");
    }

    #[test]
    fn not_negative_number() {
        let tokens = tokens!("!-99").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_expression();

        assert_eq!(format!("{result:?}"), "Unary { right: Unary { right: Literal(Number(99.0)), operator: Minus }, operator: Bang }");
    }

    #[test]
    fn complex_expression() {
        let tokens =
            tokens!("123 * 2 - 456 < 42 + 99").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_expression();

        assert_eq!(format!("{result:?}"), "Binary { left: Binary { left: Binary { left: Literal(Number(123.0)), right: Literal(Number(2.0)), operator: Star }, right: Literal(Number(456.0)), operator: Minus }, right: Binary { left: Literal(Number(42.0)), right: Literal(Number(99.0)), operator: Plus }, operator: Less }");
    }

    #[test]
    fn regression_lhs_grouped_binary_expressions() {
        let tokens = tokens!("(1)+2").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_expression();

        assert_eq!(format!("{result:?}"), "Binary { left: Grouping(Literal(Number(1.0))), right: Literal(Number(2.0)), operator: Plus }");
    }

    #[test]
    fn print() {
        let tokens = tokens!("print 42;").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse();

        assert_eq!(
            format!("{result:?}"),
            "Program([Statement(Print(Literal(Number(42.0))))])"
        );
    }

    #[test]
    fn print_twice() {
        let tokens =
            tokens!("print 42; print true;").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse();

        assert_eq!(
            format!("{result:?}"),
            "Program([Statement(Print(Literal(Number(42.0)))), Statement(Print(Literal(Boolean(true))))])"
        );
    }

    #[test]
    fn expression_statement() {
        let tokens = tokens!("42;").expect("Scanner should not fail to parse source");
        let mut parser = Parser::new(&tokens);

        let result = parser.parse();

        assert_eq!(
            format!("{result:?}"),
            "Program([Statement(Expression(Literal(Number(42.0))))])"
        );
    }
}
