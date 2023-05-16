use super::datatype::token::*;

fn collect_until<F>(chars: &mut std::iter::Peekable<std::str::Chars>, condition: F) -> String
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

pub fn lex(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(&token) = chars.peek() {
        match token {
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
            }
            '-' if chars.nth(1).map_or(false, |c| c == '-') => {
                chars.next();
                chars.next();
                let _ = collect_until(&mut chars, |c, _| c == '\n').trim().to_string();
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
                let text = collect_until(&mut chars, |c, _| !c.is_alphanumeric() && c != '_');
                tokens.push(Token::Variable(text));
            }
            token if token.is_ascii_digit() => {
                let num = collect_until(&mut chars, |c, _| !c.is_ascii_digit() && c != '.');
                tokens.push(Token::Number(num));
            }
            token if token.is_symbol() => {
                
                let mut symbol = token.to_string();

                if !token.has_next(&mut chars) {
                    chars.next();
                } else {
                    symbol = collect_until(&mut chars, |c, _| !c.is_symbol() );
                }

                if let Some(s) = symbol.as_symbol() {
                    tokens.push(Token::Symbol(s));
                }
            }
            _ => {
                let text = collect_until(&mut chars, |c, result| !c.is_alphanumeric() && c != '_' && !result.has_suffix() );
                if let Some(function) = text.as_function() {
                    tokens.push(Token::Function(function));
                } else if let Some(keyword) = text.as_keyword() {
                    tokens.push(Token::Keyword(keyword));
                } else {
                    tokens.push(Token::Identifier(text));
                }
            }
        }
    }
    tokens
}