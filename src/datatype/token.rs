use std::fmt;
use super::{
    keyword::*,
    symbol::*,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    Number(String),
    Comment(String),
}

pub fn collect_until<F>(chars: &mut std::iter::Peekable<std::str::Chars>, condition: F) -> String
where
    F: Fn(char, String) -> bool,
{
    let mut result = String::new();

    while let Some(&c) = chars.peek() {
        if condition(c, result.clone()) {
            break;
        }
        result.push(c);
        chars.next();
    }
    result
}

pub trait SqlCharExt {
    fn is_symbol(&self) -> bool;
    fn as_symbol(&self) -> Option<Symbol>;
    fn is_terminator(&self) -> bool;
}

impl SqlCharExt for char {
    fn is_symbol(&self) -> bool {
        if let Some(_) = parse_symbol(&self.to_string().as_str()) {
            return true
        }
        false
    }
    fn as_symbol(&self) -> Option<Symbol> {
        if let Some(symbol) = parse_symbol(&self.to_string().as_str()) {
            return Some(symbol)
        }
        None
    }
    fn is_terminator(&self) -> bool {
        match self {
            ';' | '/' => true,
            _ => false,
        }
    }
}

pub trait SqlStringExt {
    fn is_keyword(&self) -> bool;
    fn as_keyword(&self) -> Option<Keyword>;
    fn as_symbol(&self) -> Option<Symbol>;
}

impl SqlStringExt for String {
    fn is_keyword(&self) -> bool {
        if let Some(_) = parse_keyword(&self.as_str()) {
            return true
        }
        false
    }
    fn as_keyword(&self) -> Option<Keyword> {
        if let Some(keyword) = parse_keyword(&self) {
            return Some(keyword)
        }
        None
    }
    fn as_symbol(&self) -> Option<Symbol> {
        if let Some(symbol) = parse_symbol(&self.as_str()) {
            return Some(symbol)
        }
        None
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword(keyword) => write!(f, "{}", keyword),
            Token::Symbol(symbol) => write!(f, "{}", symbol),
            Token::Identifier(identifier) => write!(f, "{}", identifier),
            Token::Number(num) => write!(f, "{}", num),
            Token::Comment(comment) => write!(f, "{}", comment),
        }
    }
}

impl Token {
    pub fn is_operator(&self) -> bool {
        match self {
            Token::Symbol(Symbol::Comma)
            | Token::Symbol(Symbol::Dot)
            | Token::Symbol(Symbol::Asterisk)
            | Token::Symbol(Symbol::Plus)
            | Token::Symbol(Symbol::Minus)
            | Token::Symbol(Symbol::Slash)
            | Token::Symbol(Symbol::Percent)
            | Token::Symbol(Symbol::LeftParen)
            | Token::Symbol(Symbol::RightParen) => true,
            _ => false,
        }
    }

    pub fn as_symbol(&self) -> Option<Symbol> {
        match self {
            Token::Symbol(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn as_keyword(&self) -> Option<Keyword> {
        match self {
            Token::Keyword(k) => Some(k.clone()),
            _ => None,
        }
    }

    pub fn is_terminator(&self) -> bool {
        match self {
            Token::Symbol(Symbol::Semicolon)
            | Token::Symbol(Symbol::Slash) => true,
            _ => false
        }
    }
}