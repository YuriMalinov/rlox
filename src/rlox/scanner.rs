use super::error_handler::ErrorHandler;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(&'a str),
    StringValue(&'a str),
    NumberValue(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

fn reserved_words() -> HashMap<&'static str, Token<'static>> {
    let mut map = HashMap::new();
    map.insert("and", Token::And);
    map.insert("or", Token::Or);
    map.insert("if", Token::If);
    map.insert("else", Token::Else);
    map.insert("true", Token::True);
    map.insert("false", Token::False);
    map.insert("var", Token::Var);
    map.insert("fun", Token::Fun);
    map.insert("return", Token::Return);
    map.insert("class", Token::Class);
    map.insert("this", Token::This);
    map.insert("super", Token::Super);
    map.insert("for", Token::For);
    map.insert("while", Token::While);
    map.insert("print", Token::Print);
    map.insert("nil", Token::Nil);
    map
}


#[derive(Debug)]
pub struct TokenInfo<'a> {
    token: Token<'a>,
    line: u32,
}

#[derive(Debug)]
pub struct Scanner<'s> {
    // Input data
    code: &'s str,
    chars: Vec<(usize, char)>,
    error_handler: &'s dyn ErrorHandler,

    // Temp data
    had_errors: bool,
    line: u32,
    start: usize,
    current: usize,
    tokens: Vec<TokenInfo<'s>>,
    reserved_words: HashMap<&'static str, Token<'s>>,
}

impl<'s> Scanner<'s> {
    pub fn new<'ss>(code: &'ss str, error_handler: &'ss dyn ErrorHandler) -> Scanner<'ss> {
        Scanner {
            code,
            chars: code.char_indices().collect(),
            error_handler,
            had_errors: false,
            line: 1,
            start: 0,
            current: 0,
            tokens: vec![],
            reserved_words: reserved_words(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<TokenInfo> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(Token::EOF);
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        let token: Option<Token> = match c {
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '-' => Some(Token::Minus),
            '+' => Some(Token::Plus),
            ';' => Some(Token::Semicolon),
            '*' => Some(Token::Star),

            '!' => Some(if self.matches('=') { Token::BangEqual } else { Token::Bang }),
            '=' => Some(if self.matches('=') { Token::EqualEqual } else { Token::Equal }),
            '<' => Some(if self.matches('=') { Token::LessEqual } else { Token::Less }),
            '>' => Some(if self.matches('=') { Token::GreaterEqual } else { Token::Greater }),

            '/' => if self.matches('/') {
                while self.peek() != '\n' {
                    self.advance();
                }
                None
            } else {
                Some(Token::Slash)
            }

            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }

            '"' => self.string(),

            _ => if c.is_digit(10) {
                self.number()
            } else if c.is_alphabetic() {
                self.identifier()
            } else {
                self.had_errors = true;
                self.error_handler.error(self.line, &format!("Unexpected character {}", c));
                None
            }
        };

        match token {
            Some(t) => self.add_token(t),
            None => {}
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.chars[self.current].1 != expected { return false; }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.chars[self.current].1
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() { return '\0'; }
        if self.current + 1 >= self.chars.len() { return '\0'; }
        self.chars[self.current + 1].1
    }

    fn advance(&mut self) -> char {
        let char = self.chars[self.current].1;
        self.current += 1;
        char
    }

    fn add_token(&mut self, token_type: Token<'s>) {
        self.tokens.push(TokenInfo { line: self.line, token: token_type })
    }

    fn current_text(&self) -> &'s str {
        &self.code[self.chars[self.start].0..self.chars[self.current].0]
    }

    fn string(&mut self) -> Option<Token<'s>> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.advance() == '\n' {
                self.line += 1;
            }
        }

        if self.is_at_end() {
            self.error_handler.error(self.line, "Unterminated string.");
            None
        } else {
            Some(Token::StringValue(self.current_text()))
        }
    }

    fn number(&mut self) -> Option<Token<'s>> {
        while self.peek().is_digit(10) { self.advance(); }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) { self.advance(); }
        }

        match self.current_text().parse::<f64>() {
            Ok(val) => Some(Token::NumberValue(val)),
            Err(err) => {
                self.error_handler.error(self.line, &format!("{}", err));
                None
            }
        }
    }

    fn identifier(&mut self) -> Option<Token<'s>> {
        while self.peek().is_alphanumeric() { self.advance(); }

        let text = self.current_text();
        match self.reserved_words.get(text) {
            Some(token) => Some(token.clone()),
            None => Some(Token::Identifier(text))
        }
    }
}