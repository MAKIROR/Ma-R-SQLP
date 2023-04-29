use super::datatype::token::*;

pub fn lex(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(&token) = chars.peek() {
        match token {
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
            }
            '-' if chars.peek().map_or(false, |&c| c == '-') => {
                chars.next();
                chars.next();
                let comment = collect_until(&mut chars, |c| c == '\n').trim().to_string();
                tokens.push(Token::Comment(comment));
            }
            '\'' | '"' => {
                if let Some(quote) = chars.next() {
                    let literal = collect_until(&mut chars, |c| c == quote);
                    tokens.push(Token::Identifier(literal));
                    chars.next();
                }
            }
            token if token.is_ascii_digit() => {
                let num = collect_until(&mut chars, |c| !c.is_ascii_digit() && c != '.');
                tokens.push(Token::Num(num));
            }
            token if token.is_symbol() => {
                let symbol = collect_until(&mut chars, |c| !c.is_symbol());
                if let Some(s) = symbol.as_symbol() {
                    tokens.push(Token::Symbol(s));
                }
            }
            _ => {
                let text = collect_until(&mut chars, |c| !c.is_alphanumeric() && c != '_');
                if let Some(keyword) = text.as_keyword() {
                    tokens.push(Token::Keyword(keyword));
                } else {
                    tokens.push(Token::Identifier(text));
                }
                chars.next();
            }
        }
    }
    tokens
}