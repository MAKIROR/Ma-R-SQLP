use std::{
    vec::IntoIter,
    iter::Peekable
};
use super::{
    clause_parser::*,
    error::{ParseError, Result},
    datatype::token::*,
    models::{
        ast::*,
        structs::*,
    },
    datatype::keyword::Keyword,
};

pub fn parse_select(t: &Vec<Token>) -> Result<Statement> {
    let tokens = t.clone();
    let mut iter = tokens.into_iter().peekable();

    match_token(&iter.next(), Token::Keyword(Keyword::Select))?;
    let distinct = match parse_optional_args_or(&mut iter, vec![Keyword::All, Keyword::Distinct], Keyword::All) {
        Keyword::Distinct => true,
        _ => false,
    };
    
    let projections = parse_projection(&mut iter)?;
    let table = parse_table(&mut iter)?;
    let filter = parse_where(&mut iter)?;
    let group_by = parse_groupby(&mut iter)?;
    let having = parse_having(&mut iter)?;
    let order_by = parse_orderby(&mut iter)?;

    if let Some(terminator) = iter.next() {
        if !terminator.is_terminator() {
            return Err(ParseError::UnexpectedToken(terminator));
        }
    } else {
        return Err(ParseError::MissingTerminator);
    }

    return Ok(Statement::Select {
        distinct,
        projections,
        table,
        filter,
        group_by,
        having,
        order_by
    });
}

pub fn parse_insert(t: &Vec<Token>) -> Result<ASTNode> {
    let tokens = t.clone();
    let mut iter = tokens.into_iter().peekable();

    match_token(&iter.next(), Token::Keyword(Keyword::Insert))?;

    todo!()
   // TODO:
}

pub fn parse_delete(t: &Vec<Token>) -> Result<ASTNode> {
   todo!()
   // TODO:
}

fn parse_optional_args_or(
    iter: &mut Peekable<IntoIter<Token>>,
    args: Vec<Keyword>,
    default: Keyword,
) -> Keyword {
    if let Some(Token::Keyword(keyword)) = iter.peek() {
        if let Some(nodetype) = args.iter().find(|&a| a.clone() == keyword.clone()) {
            iter.next();
            return nodetype.clone();
        }
    }
    default
}

fn match_token(value: &Option<Token>, expect: Token) -> Result<()> {
    return match value {
        Some(_) => Ok(()),
        None => return Err(ParseError::MissingToken(expect))
    }
}