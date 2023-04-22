#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Num(String),
    Comment(String),
    Symbol(String),
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
    fn is_keyword(&self) -> bool;
    fn is_symbol(&self) -> bool;
}

impl SqlCharExt for char {
    fn is_keyword(&self) -> bool {
        false
    }

    fn is_symbol(&self) -> bool {
        [',', '(', ')', '=', '<', '>', '*'].contains(self)
    }
}

impl SqlCharExt for String {
    fn is_keyword(&self) -> bool {
        ["SELECT", "FROM", "WHERE", "AND", "OR", "NOT"].contains(&self.as_str())
    }

    fn is_symbol(&self) -> bool {
        false
    }
}
