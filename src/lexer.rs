use super::datatype::{
    token::*,
    keyword::KeywordExt,
};

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
                let comment = collect_until(&mut chars, |c, _| c == '\n').trim().to_string();
                tokens.push(Token::Comment(comment));
            }
            '\'' | '"' => {
                if let Some(quote) = chars.next() {
                    let literal = collect_until(&mut chars, |c, _| c == quote);
                    tokens.push(Token::Identifier(literal));
                    chars.next();
                }
            }
            '@' => {
                chars.next();
                let text = collect_until(&mut chars, |c, result| !c.is_alphanumeric() && c != '_');
                tokens.push(Token::Variable(text));
            }
            token if token.is_terminator() => {
                tokens.push(Token::Symbol(token.as_symbol().take().unwrap().clone()));
                chars.next();
            }
            token if token.is_ascii_digit() => {
                let num = collect_until(&mut chars, |c, _| !c.is_ascii_digit() && c != '.');
                tokens.push(Token::Number(num));
            }
            token if token.is_symbol() => {
                let symbol = collect_until(&mut chars, |c, _| !c.is_symbol() || c.is_terminator() );
                if let Some(s) = symbol.as_symbol() {
                    tokens.push(Token::Symbol(s));
                }
            }
            _ => {
                let text = collect_until(&mut chars, |c, result| !c.is_alphanumeric() && c != '_' && !result.has_suffix() );
                if text.is_function() {
                    if let Some('(') = chars.peek() {
                        let full_text = format!("{}{}", text, collect_to(&mut chars, |c, _| c == ')'));
                        if let Some(function) = full_text.as_function() {
                            tokens.push(Token::Function(function));
                        }
                    }
                }
                else if let Some(keyword) = text.as_keyword() {
                    tokens.push(Token::Keyword(keyword));
                } else {
                    tokens.push(Token::Identifier(text));
                }
            }
        }
    }
    tokens
}