use crate::prelude::utils::errors::handle_error;
use crate::token::{Literal, Token, TokenType};

/// ...
const KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

pub struct Scanner {
    current: usize,
    line: u64,
    source: String,
    start: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new (source: String) -> Self {
        Scanner {
            current: 0,
            line: 1,
            source,
            start: 0,
            tokens: vec![],
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme here
            self.start = self.current;
            self.scan_token();
        }

        let eof = Token {
            lexeme: String::new(),
            line: self.line,
            literal: Literal::None,
            token_type: TokenType::EOF,
        };
        self.tokens.push(eof);

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        let mut token_type: Option<TokenType> = None;

        match c {
            // single character tokens
            '(' => token_type = Some(TokenType::LeftParen),
            ')' => token_type = Some(TokenType::RightParen),
            '{' => token_type = Some(TokenType::LeftBrace),
            '}' => token_type = Some(TokenType::RightBrace),
            ',' => token_type = Some(TokenType::Comma),
            '.' => token_type = Some(TokenType::Dot),
            '-' => token_type = Some(TokenType::Minus),
            '+' => token_type = Some(TokenType::Plus),
            ';' => token_type = Some(TokenType::Semicolon),
            '*' => token_type = Some(TokenType::Star),

            // single or multi-character tokens
            '!' => {
                token_type = if self.advance_if_matched('=') {
                    Some(TokenType::BangEqual)
                } else {
                    Some(TokenType::Bang)
                };
            },
            '=' => {
                token_type = if self.advance_if_matched('=') {
                    Some(TokenType::EqualEqual)
                } else {
                    Some(TokenType::Equal)
                };
            },
            '<' => {
                token_type = if self.advance_if_matched('=') {
                    Some(TokenType::LessEqual)
                } else {
                    Some(TokenType::Less)
                };
            },
            '>' => {
                token_type = if self.advance_if_matched('=') {
                    Some(TokenType::GreaterEqual)
                } else {
                    Some(TokenType::Greater)
                };
            },
            '/' => {
                if self.advance_if_matched('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    token_type = Some(TokenType::Slash);
                }
            }

            // whitespace
            ' ' | '\r' | '\t' => {},

            // newlines
            '\n' => self.line += 1,

            // strings
            '"' => token_type = Some(self.string()),

            _ => {
                if self.is_digit(c) {
                    token_type = Some(self.number());
                } else if self.is_alpha(c) {
                    token_type = Some(self.identifier());
                } else {
                    handle_error("Unexpected character.", self.line);
                }
            },
        }

        if let Some(tt) = token_type {
            let lexeme: String = self.source
                .chars()
                .skip(self.start)
                .take(self.current - self.start)
                .collect();
            let literal = match tt {
                TokenType::Number => Literal::Number(lexeme.parse::<f64>().unwrap()),
                TokenType::String => Literal::String(lexeme.clone()),
                _ => Literal::None,
            };

            let token = Token {
                line: self.line,
                literal,
                lexeme,
                token_type: tt,
            };
            self.tokens.push(token);
        }
    }

    fn identifier(&mut self) -> TokenType {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        // TODO: avoid duplicating this logic...
        let text = &self.source[self.start..self.current];

        match KEYWORDS.get(text) {
            Some(t) => *t,
            None => TokenType::Identifier,
        }
    }

    fn number(&mut self) -> TokenType {
        while self.is_digit(self.peek()) { self.advance(); }

        // look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // consume the "."
            self.advance();

            while self.is_digit(self.peek()) { self.advance(); }
        }

        TokenType::Number
    }

    fn string(&mut self) -> TokenType {
        while self.peek() != '"' && !self.is_at_end() {
            // lox supports multiline strings
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        TokenType::String
    }

    fn advance_if_matched(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.source.chars().nth(self.current).unwrap() != expected { return false }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn is_alpha(&self, c: char) -> bool {
        let code = c as u32;

        (code > 64 && code < 91) // uppercase
            || (code > 96 && code < 123) // lowercase
            || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool{
        let code = c as u32;

        code > 47 && code < 58
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        // TODO: its ok if we handle out of bounds

        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }
}
