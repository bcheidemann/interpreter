use super::{
    error::CompilerResult,
    utils::{is_alpha, is_alpha_numeric, is_digit},
};

const KW_AND: &str = "and";
const KW_CLASS: &str = "class";
const KW_ELSE: &str = "else";
const KW_FALSE: &str = "false";
const KW_FOR: &str = "for";
const KW_FUN: &str = "fun";
const KW_IF: &str = "if";
const KW_NIL: &str = "nil";
const KW_OR: &str = "or";
const KW_PRINT: &str = "print";
const KW_RETURN: &str = "return";
const KW_SUPER: &str = "super";
const KW_THIS: &str = "this";
const KW_TRUE: &str = "true";
const KW_VAR: &str = "var";
const KW_WHILE: &str = "while";

#[derive(Debug)]
pub enum TokenDirection {
    Left,
    Right,
}

#[derive(Debug)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Function,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    VariableDeclaration,
    While,
}

#[derive(Debug)]
pub enum Token {
    Paren(TokenDirection),
    Brace(TokenDirection),
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    SemiColon,
    Bang,
    BangEquals,
    Equals,
    EqualsEquals,
    LessEqual,
    Less,
    Greater,
    GreaterEqual,
    String(String),
    Number(f32),
    Identifier(String),
    Keyword(Keyword),
}

#[derive(Debug)]
pub struct Tokens(Vec<Token>);

impl Tokens {
    pub fn new() -> Self {
        Tokens(vec![])
    }

    pub fn push(&mut self, token: Token) {
        self.0.push(token);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        self.0.get(index)
    }
}

pub struct Scanner {
    tokens: Tokens,
    source_chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    #[cfg(test)]
    pub fn parse_tokens_from_source(source: &str) -> CompilerResult<Tokens> {
        let mut scanner = Self::from_source(source);
        scanner.scan_tokens()?;
        Ok(scanner.tokens)
    }

    pub fn from_source(source: &str) -> Self {
        Self {
            tokens: Tokens::new(),
            source_chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> CompilerResult<&Tokens> {
        while !self.is_at_end() {
            self.scan_token()?;
        }

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> CompilerResult<()> {
        self.start = self.current;
        let char = self.advance();

        let token = {
            if let Some(char) = char {
                match char {
                    ' ' | '\r' | '\t' => None,
                    '\n' => {
                        self.line += 1;
                        None
                    }
                    '(' => Some(Token::Paren(TokenDirection::Left)),
                    ')' => Some(Token::Paren(TokenDirection::Right)),
                    '{' => Some(Token::Brace(TokenDirection::Left)),
                    '}' => Some(Token::Brace(TokenDirection::Right)),
                    ',' => Some(Token::Comma),
                    '.' => Some(Token::Dot),
                    '-' => Some(Token::Minus),
                    '+' => Some(Token::Plus),
                    '*' => Some(Token::Star),
                    ';' => Some(Token::SemiColon),
                    '!' => {
                        if self.match_next('=') {
                            self.current += 1;
                            Some(Token::BangEquals)
                        } else {
                            Some(Token::Bang)
                        }
                    }
                    '=' => {
                        if self.match_next('=') {
                            self.current += 1;
                            Some(Token::EqualsEquals)
                        } else {
                            Some(Token::Equals)
                        }
                    }
                    '<' => {
                        if self.match_next('=') {
                            self.current += 1;
                            Some(Token::LessEqual)
                        } else {
                            Some(Token::Less)
                        }
                    }
                    '>' => {
                        if self.match_next('=') {
                            self.current += 1;
                            Some(Token::GreaterEqual)
                        } else {
                            Some(Token::Greater)
                        }
                    }
                    '/' => {
                        if self.match_next('/') {
                            loop {
                                match self.advance() {
                                    Some('\n') => break,
                                    Some(_) => {}
                                    None => break,
                                }
                            }
                            None
                        } else {
                            Some(Token::Slash)
                        }
                    }
                    '"' => {
                        loop {
                            match self.advance() {
                                Some('"') => break,
                                Some('\n') => {
                                    self.line += 1;
                                }
                                Some(_) => {}
                                None => return Err("Unterminated string".to_string()),
                            }
                        }
                        Some(Token::String(
                            self.source_chars[self.start..self.current]
                                .into_iter()
                                .collect(),
                        ))
                    }
                    char => {
                        if is_digit(char) {
                            loop {
                                match self.advance() {
                                    Some('.') => {}
                                    Some(char) => {
                                        if !is_digit(char) {
                                            break;
                                        }
                                    }
                                    None => break,
                                }
                            }
                            self.current -= 1;
                            Some(Token::Number(
                                self.source_chars[self.start..self.current]
                                    .into_iter()
                                    .collect::<String>()
                                    .parse()
                                    .expect("Failed to parse float"),
                            ))
                        } else if is_alpha(char) {
                            loop {
                                match self.advance() {
                                    Some(char) => {
                                        if !is_alpha_numeric(char) {
                                            break;
                                        }
                                    }
                                    None => break,
                                }
                            }
                            self.current -= 1;
                            let alpha_numeric: String = self.source_chars[self.start..self.current]
                                .into_iter()
                                .collect();
                            Some(match &alpha_numeric[..] {
                                KW_AND => Token::Keyword(Keyword::And),
                                KW_CLASS => Token::Keyword(Keyword::Class),
                                KW_ELSE => Token::Keyword(Keyword::Else),
                                KW_FALSE => Token::Keyword(Keyword::False),
                                KW_FOR => Token::Keyword(Keyword::For),
                                KW_FUN => Token::Keyword(Keyword::Function),
                                KW_IF => Token::Keyword(Keyword::If),
                                KW_NIL => Token::Keyword(Keyword::Nil),
                                KW_OR => Token::Keyword(Keyword::Or),
                                KW_PRINT => Token::Keyword(Keyword::Print),
                                KW_RETURN => Token::Keyword(Keyword::Return),
                                KW_SUPER => Token::Keyword(Keyword::Super),
                                KW_THIS => Token::Keyword(Keyword::This),
                                KW_TRUE => Token::Keyword(Keyword::True),
                                KW_VAR => Token::Keyword(Keyword::VariableDeclaration),
                                KW_WHILE => Token::Keyword(Keyword::While),
                                _ => Token::Identifier(alpha_numeric),
                            })
                        } else {
                            return Err(format!(
                                "Unexpected character ({}) on line {}",
                                char, self.line
                            ));
                        }
                    }
                }
            } else {
                None
            }
        };

        if let Some(token) = token {
            self.add_token(token);
        }

        Ok(())
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_chars.len()
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.peek();
        self.current += 1;
        char
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source_chars[self.current])
        }
    }

    fn match_next(&self, maybe_next: char) -> bool {
        match self.peek() {
            Some(next) => next == maybe_next,
            None => false,
        }
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! tokens {
    ($source:expr) => {
        Scanner::parse_tokens_from_source($source)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_chars() {
        let mut scanner = Scanner::from_source("(){},.-+*;");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");

        assert_eq!(format!("{tokens:?}"), "Tokens([Paren(Left), Paren(Right), Brace(Left), Brace(Right), Comma, Dot, Minus, Plus, Star, SemiColon])");
    }

    #[test]
    fn maybe_single_chars() {
        let mut scanner = Scanner::from_source("> < = !");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");

        assert_eq!(
            format!("{tokens:?}"),
            "Tokens([Greater, Less, Equals, Bang])"
        );
    }

    #[test]
    fn double_chars() {
        let mut scanner = Scanner::from_source(">= <= == != /");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");

        assert_eq!(
            format!("{tokens:?}"),
            "Tokens([GreaterEqual, LessEqual, EqualsEquals, BangEquals, Slash])"
        );
    }

    #[test]
    fn comments() {
        let mut scanner = Scanner::from_source("!\n// Hello World!\n!");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");

        assert_eq!(format!("{tokens:?}"), "Tokens([Bang, Bang])");
    }

    #[test]
    fn unexpected_char() {
        let mut scanner = Scanner::from_source("\n#");

        let error = scanner
            .scan_tokens()
            .expect_err("Scanner should return an error when it encounters an unexpected error");

        assert_eq!(error, "Unexpected character (#) on line 2");
    }

    #[test]
    fn unexpected_char_after_number_new_line() {
        let mut scanner = Scanner::from_source("123\n#");

        let error = scanner
            .scan_tokens()
            .expect_err("Scanner should return an error when it encounters an unexpected error");

        assert_eq!(error, "Unexpected character (#) on line 2");
    }

    #[test]
    fn ignore_white_space() {
        let mut scanner = Scanner::from_source("\n\t!\r");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");

        assert_eq!(format!("{tokens:?}"), "Tokens([Bang])");
    }

    #[test]
    fn string() {
        let mut scanner = Scanner::from_source("\"Hello World!\"");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");
        let token = tokens.0.get(0).expect("Expected a token");

        assert_eq!(
            format!("{tokens:?}"),
            "Tokens([String(\"\\\"Hello World!\\\"\")])"
        );

        match token {
            Token::String(value) => {
                assert_eq!(value, "\"Hello World!\"");
            }
            _ => panic!("Expected a string token"),
        }
    }

    #[test]
    fn multi_line_string() {
        let mut scanner = Scanner::from_source("\"Hello\nWorld!\"");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");
        let token = tokens.0.get(0).expect("Expected a token");

        assert_eq!(
            format!("{tokens:?}"),
            "Tokens([String(\"\\\"Hello\\nWorld!\\\"\")])"
        );

        match token {
            Token::String(value) => {
                assert_eq!(value, "\"Hello\nWorld!\"");
            }
            _ => panic!("Expected a string token"),
        }
    }

    #[test]
    fn integer() {
        let mut scanner = Scanner::from_source("123");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");
        let token = tokens.0.get(0).expect("Expected a token");

        assert_eq!(format!("{tokens:?}"), "Tokens([Number(123.0)])");

        match token {
            Token::Number(value) => {
                assert_eq!(*value, 123.0);
            }
            _ => panic!("Expected a string token"),
        }
    }

    #[test]
    fn float() {
        let mut scanner = Scanner::from_source("123.456");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");
        let token = tokens.0.get(0).expect("Expected a token");

        assert_eq!(format!("{tokens:?}"), "Tokens([Number(123.456)])");

        match token {
            Token::Number(value) => {
                assert_eq!(*value, 123.456);
            }
            _ => panic!("Expected a string token"),
        }
    }

    #[test]
    fn number_equal_equal_number() {
        let mut scanner = Scanner::from_source("123==456");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");

        assert_eq!(
            format!("{tokens:?}"),
            "Tokens([Number(123.0), EqualsEquals, Number(456.0)])"
        );
    }

    #[test]
    fn number_trailing_dot() {
        let mut scanner = Scanner::from_source("123.");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");

        assert_eq!(format!("{tokens:?}"), "Tokens([Number(123.0)])");
    }

    #[test]
    fn identifier() {
        let mut scanner = Scanner::from_source("Hello World!");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");
        let token_0 = tokens.0.get(0).expect("Expected a token");
        let token_1 = tokens.0.get(1).expect("Expected a token");

        assert_eq!(
            format!("{tokens:?}"),
            "Tokens([Identifier(\"Hello\"), Identifier(\"World\"), Bang])"
        );

        match token_0 {
            Token::Identifier(value) => {
                assert_eq!(value, "Hello");
            }
            _ => panic!("Expected an identifier token"),
        }

        match token_1 {
            Token::Identifier(value) => {
                assert_eq!(value, "World");
            }
            _ => panic!("Expected an identifier token"),
        }
    }

    #[test]
    fn keywords() {
        let mut scanner = Scanner::from_source("Hello super World!");

        let tokens = scanner
            .scan_tokens()
            .expect("Scanner should not fail to parse source");

        assert_eq!(
            format!("{tokens:?}"),
            "Tokens([Identifier(\"Hello\"), Keyword(Super), Identifier(\"World\"), Bang])"
        );
    }
}
