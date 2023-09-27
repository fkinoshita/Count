use crate::calc::lexer::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Boolean(bool),
    Number(f64),
    Name(String),
}

impl Atom {
    pub fn number(&self) -> Option<f64> {
        match self {
            Atom::Number(number) => Some(*number),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    None,
    Literal(Atom),
    Unary(Box<Expression>, Token),
    Binary(Box<Expression>, Box<Expression>, Token),
    Logical(Box<Expression>, Box<Expression>, Token),
    Group(Box<Expression>),
    Variable(String, Box<Expression>),
}

pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
    expressions: Vec<Expression>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            index: 0,
            tokens,
            expressions: vec![],
        }
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        while !self.is_at_end() {
            let expr = self.parse_statement();
            self.expressions.push(expr);
        }

        self.expressions.clone()
    }

    fn parse_statement(&mut self) -> Expression {
        if self.r#match(vec![TokenKind::Identifier(self.peek().lexeme())]) {
            return self.parse_identifier();
        }

        self.parse_expression()
    }

    fn parse_identifier(&mut self) -> Expression {
        let identifier = self.previous();

        if self.r#match(vec![TokenKind::Colon]) {
            let value = self.parse_statement();

            if self.check(TokenKind::Newline) {
                self.consume();
            }

            return Expression::Variable(identifier.lexeme(), Box::new(value));
        }

        self.parse_statement()
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expression {
        let mut expression = self.parse_and();

        while self.r#match(vec![TokenKind::Or]) {
            let operator = self.previous();
            let right = self.parse_and();
            expression = Expression::Logical(Box::new(expression), Box::new(right), operator);
        }

        expression
    }

    fn parse_and(&mut self) -> Expression {
        let mut expression = self.parse_equality();

        while self.r#match(vec![TokenKind::And]) {
            let operator = self.previous();
            let right = self.parse_equality();
            expression = Expression::Logical(Box::new(expression), Box::new(right), operator);
        }

        expression
    }

    fn parse_equality(&mut self) -> Expression {
        let mut expression = self.parse_comparison();

        while self.r#match([TokenKind::Equal].to_vec()) {
            let operator = self.previous();
            let right = self.parse_comparison();
            expression = Expression::Binary(Box::new(expression), Box::new(right), operator);
        }

        expression
    }

    fn parse_comparison(&mut self) -> Expression {
        let mut expression = self.parse_term();

        while self.r#match([TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual].to_vec()) {
            let operator = self.previous();
            let right = self.parse_term();
            expression = Expression::Binary(Box::new(expression), Box::new(right), operator);
        }

        expression
    }

    fn parse_term(&mut self) -> Expression {
        let mut expression = self.parse_factor();

        while self.r#match([TokenKind::Minus, TokenKind::Plus].to_vec()) {
            let operator = self.previous();
            let right = self.parse_factor();
            expression = Expression::Binary(Box::new(expression), Box::new(right), operator);
        }

        expression
    }

    fn parse_factor(&mut self) -> Expression {
        let mut expression = self.parse_unary();

        while self.r#match([TokenKind::Over, TokenKind::Times, TokenKind::Mod].to_vec()) {
            let operator = self.previous();
            let right = self.parse_unary();
            expression = Expression::Binary(Box::new(expression), Box::new(right), operator);
        }

        expression
    }

    fn parse_unary(&mut self) -> Expression {
        if self.r#match(vec![TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.parse_unary();
            return Expression::Unary(Box::new(right), operator);
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expression {
        if self.r#match(vec![TokenKind::False]) {
            if self.check(TokenKind::Newline) {
                self.consume();
            }
            return Expression::Literal(Atom::Boolean(false));
        }
        if self.r#match(vec![TokenKind::True]) {
            if self.check(TokenKind::Newline) {
                self.consume();
            }
            return Expression::Literal(Atom::Boolean(true));
        }

        let mut number_value = 0.0;
        let mut string_value = "".to_string();

        match self.peek().kind() {
            TokenKind::Number(x) => {
                number_value = x;
            }
            TokenKind::Identifier(x) => {
                string_value = x;
            }
            _ => {},
        }

        if self.r#match(vec![TokenKind::Identifier(string_value.clone())]) {
            if self.check(TokenKind::Newline) {
                self.consume();
            }
            return Expression::Literal(Atom::Name(string_value.clone()));
        }

        if self.r#match(vec![TokenKind::Number(number_value)]) {
            if self.check(TokenKind::Newline) {
                self.consume();
            }
            return Expression::Literal(Atom::Number(number_value));
        }

        if self.r#match([TokenKind::OpenParen].to_vec()) {
            let expression = self.parse_expression();

            self.consume_with(TokenKind::CloseParen, "expected ')' after expression");

            if self.check(TokenKind::Newline) {
                self.consume();
            }

            return Expression::Group(Box::new(expression));
        }

        if self.check(TokenKind::Newline) {
            self.consume();
        }

        Expression::Literal(Atom::Name(self.previous().lexeme()))
    }

    fn r#match(&mut self, kinds: Vec<TokenKind>) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.consume();

                return true;
            }
        }

        false
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().kind() == kind
    }

    fn consume(&mut self) -> Token {
        if !self.is_at_end() {
            self.index += 1
        }

        self.previous()
    }

    fn consume_with(&mut self, kind: TokenKind, message: &str) -> Token {
        if self.check(kind) {
            return self.consume();
        }

        println!("error at '{:?}': {}", self.peek().kind(), message);

        Token::new(TokenKind::Invalid, "")
    }


    fn peek(&self) -> Token {
        self.tokens.get(self.index).unwrap().clone()
    }

    fn previous(&self) -> Token {
        if self.tokens.len() < 2 {
            return Token::new(TokenKind::Invalid, "");
        }

        self.tokens.get(self.index - 1).unwrap().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind() == TokenKind::Eof
    }
}

