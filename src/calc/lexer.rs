use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Number(f64),
    True,
    False,

    Percent,
    Dot,
    Colon,
    OpenParen,
    CloseParen,

    Plus,
    Minus,
    Times,
    Over,
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Mod,

    Whitespace,
    Newline,
    Eof,
    Invalid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: &str) -> Self {
        Self { kind, lexeme: lexeme.to_string() }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }

    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }
}

pub struct Lexer {
    input: String,
    tokens: Vec<Token>,
    index: usize,
    start: usize,
    keywords: HashMap<String, TokenKind>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("true".to_string(), TokenKind::True);
        keywords.insert("false".to_string(), TokenKind::False);
        keywords.insert("plus".to_string(), TokenKind::Plus);
        keywords.insert("minus".to_string(), TokenKind::Minus);
        keywords.insert("times".to_string(), TokenKind::Times);
        keywords.insert("over".to_string(), TokenKind::Over);
        keywords.insert("equals".to_string(), TokenKind::Equal);
        keywords.insert("and".to_string(), TokenKind::And);
        keywords.insert("or".to_string(), TokenKind::Or);
        keywords.insert("mod".to_string(), TokenKind::Mod);

        Self {
            input,
            tokens: vec![],
            index: 0,
            start: 0,
            keywords,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.index;

            let kind = self.consume_token();
            self.tokens.push(Token::new(kind, &self.input[self.start..self.index]));
        }

        self.tokens.push(Token::new(TokenKind::Eof, "\0"));

        self.tokens()
    }

    fn tokens(&self) -> Vec<Token> {
        self.tokens.iter()
            .filter(|t| t.kind() != TokenKind::Whitespace)
            .map(|t| t.clone())
            .collect()
    }

    fn consume_token(&mut self) -> TokenKind {
        let c = self.consume();

        match c {
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '%' => TokenKind::Percent,
            '.' => TokenKind::Dot,
            ':' => TokenKind::Colon,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Times,
            '/' => TokenKind::Over,
            '=' => TokenKind::Equal,
            '<' => {
                if self.consume_if('=') {
                    return TokenKind::LessEqual;
                }
                TokenKind::Less
            },
            '>' => {
                if self.consume_if('=') {
                    return TokenKind::GreaterEqual;
                }
                TokenKind::Greater
            },
            '\n' => TokenKind::Newline,
            c if c.is_whitespace() => TokenKind::Whitespace,
            c if c.is_digit(10) => self.consume_number(),
            c if c.is_alphabetic() => self.consume_identifier(),
            _ => TokenKind::Invalid,
        }
    }

    fn consume_number(&mut self) -> TokenKind {
        while self.peek().is_digit(10) {
            self.consume();
        }

        if self.peek() == '.' && self.peek_ahead(1).is_digit(10) {
            self.consume();

            while self.peek().is_digit(10) {
                self.consume();
            }
        }

        if let Ok(value) = self.input[self.start..self.index].parse() {
            return TokenKind::Number(value);
        }

        TokenKind::Invalid
    }

    fn consume_identifier(&mut self) -> TokenKind {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.consume();
        }

        let text = self.input[self.start..self.index].to_string();

        if let Some(kind) = self.keywords.get(&text) {
            return kind.clone();
        }

        TokenKind::Identifier(text)
    }

    fn consume(&mut self) -> char {
        let c = self.peek();
        self.index += 1;
        c
    }

    fn consume_if(&mut self, expected_character: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.input.as_bytes()[self.index] as char != expected_character {
            return false;
        }

        self.index += 1;

        true
    }

    fn peek(&mut self) -> char {
        self.peek_ahead(0)
    }

    fn peek_ahead(&mut self, offset: usize) -> char {
        if self.index + offset >= self.input.len() {
            return '\0';
        }

        self.input.as_bytes()[self.index + offset] as char
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.input.len()
    }
}

