use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    Sum(String),
    Avg(String),
    Count(String),
    Max(String),
    Min(String),
    Concat(Vec<String>),
}

pub fn parse_function(s: &str) -> Option<Function> {
    match s.trim().to_uppercase() {
        s if s.starts_with("SUM(") && s.ends_with(")") => {
            let column = &s[4..s.len() - 1];
            return Some(Function::Sum(column.to_string()));
        }
        s if s.starts_with("AVG(") && s.ends_with(")") => {
            let column = &s[4..s.len() - 1];
            return Some(Function::Avg(column.to_string()));
        }
        s if s.starts_with("COUNT(") && s.ends_with(")") => {
            let column = &s[6..s.len() - 1];
            return Some(Function::Count(column.to_string()));
        }    
        s if s.starts_with("MAX(") && s.ends_with(")") => {
            let column = &s[4..s.len() - 1];
            return Some(Function::Max(column.to_string()));
        }
        s if s.starts_with("MIN(") && s.ends_with(")") => {
            let column = &s[4..s.len() - 1];
            return Some(Function::Min(column.to_string()));
        }
        s if s.starts_with("CONCAT(") && s.ends_with(")") => {
            let column = &s[7..s.len() - 1];
            let columns: Vec<String> = column.split(',').map(|s| s.trim().to_string()).collect();
            return Some(Function::Concat(columns));
        }
        _ => None
    }
}

pub fn is_function(s: &str) -> bool {
    match s.trim().to_uppercase().as_str() {
        "SUM"
        | "AVG"
        | "COUNT"
        | "MAX"
        | "MIN"
        | "CONCAT" => true,
        _ => false
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}