use crate::tokens::*;

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
                let quote = chars.next().unwrap();
                let literal = collect_until(&mut chars, |c| c == quote);
                tokens.push(Token::Identifier(literal));
                chars.next();
            }
            token if token.is_ascii_digit() => {
                let num = collect_until(&mut chars, |c| !c.is_ascii_digit() && c != '.');
                tokens.push(Token::Num(num));
                chars.next();
            }
            token if token.is_symbol() => {
                let symbol = collect_until(&mut chars, |c| !c.is_symbol());
                tokens.push(Token::Symbol(symbol));
                chars.next();
            }
            _ => {
                let text = collect_until(&mut chars, |c| !c.is_alphanumeric() && c != '_');
                if text.is_keyword() {
                    tokens.push(Token::Keyword(text));
                } else {
                    tokens.push(Token::Identifier(text));
                }
                chars.next();
            }
        }
    }
    tokens
}