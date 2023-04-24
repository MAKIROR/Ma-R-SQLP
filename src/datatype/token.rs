use super::{
    keyword::*,
    symbol::*,
};

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    Num(String),
    Comment(String),
}

pub fn collect_until<F>(chars: &mut std::iter::Peekable<std::str::Chars>, condition: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut result = String::new();
    while let Some(&c) = chars.peek() {
        if condition(c) {
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
