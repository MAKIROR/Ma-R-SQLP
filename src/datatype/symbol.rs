#[derive(Debug, PartialEq)]
pub enum Symbol {
    Comma,
    Dot,
    Asterisk,
    Plus,
    Minus,
    Slash,
    Percent,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LeftParen,
    RightParen,
    Semicolon,
}

pub fn parse_symbol(s: &str) -> Option<Symbol> {
    match s {
        "," => Some(Symbol::Comma),
        "." => Some(Symbol::Dot),
        "*" => Some(Symbol::Asterisk),
        "+" => Some(Symbol::Plus),
        "-" => Some(Symbol::Minus),
        "/" => Some(Symbol::Slash),
        "%" => Some(Symbol::Percent),
        "=" => Some(Symbol::Equal),
        "!=" => Some(Symbol::NotEqual),
        "<" => Some(Symbol::LessThan),
        ">" => Some(Symbol::GreaterThan),
        "<=" => Some(Symbol::LessThanOrEqual),
        ">=" => Some(Symbol::GreaterThanOrEqual),
        "(" => Some(Symbol::LeftParen),
        ")" => Some(Symbol::RightParen),
        ";" => Some(Symbol::Semicolon),
        _ => None,
    }
}