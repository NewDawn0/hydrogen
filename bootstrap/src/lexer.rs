use crate::keywords::TOKEN_MAP;
use crate::token::Token;
use std::process::exit;

pub struct Lexer<'a> {
    input: &'a str,
    ch: char,
    pos: usize,
    read_pos: usize,
    ident: &'a str,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut out = Self {
            input,
            ch: '\0',
            pos: 0,
            read_pos: 0,
            ident: "",
        };
        if input.len() > 0 {
            // Is save since length is checked
            out.ch = input.chars().nth(0).unwrap()
        } else {
            eprintln!("Nothing in source file to compile\nExiting...");
            exit(1);
        }
        out.read_char();
        out
    }
    pub fn next_token(&mut self) -> (bool, Token) {
        // TODO: fix whitespace skip in strings
        while is_space(self.ch) {
            if self.input.len() <= self.pos {
                // No more tokens
                return (false, Token::Ident(""));
            }
            self.read_char()
        }
        let tok = match is_letter(self.ch) {
            false => Token::Illegal(""),
            true => {
                self.read_ident();
                match TOKEN_MAP.get(self.ident) {
                    Some(tok) => tok.clone(),
                    None => Token::Ident(self.ident),
                }
            }
        };
        self.read_char();
        (true, tok)
    }
    fn read_ident(&mut self) {
        let pos = self.pos;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.ident = &self.input[pos..self.pos];
    }
    pub fn read_char(&mut self) {
        if self.read_pos < self.input.len() {
            // Save since length is checked
            self.ch = self.input.chars().nth(self.read_pos).unwrap()
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }
}

// Utility comparator functions
fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || '0' <= ch && ch <= '9' || ch == '_'
}
fn is_space(ch: char) -> bool {
    match ch {
        '\n' | '\t' | '\r' | ' ' => true,
        _ => false,
    }
}
