use masql::lexer::lex;
use masql::statement_parse::*;

#[test]
fn test_lex() {
    let result = lex("SELECT * FROM customers;");
    println!("{:?}", result);
    if let Err(e) = parse_select(&result) {
        println!("{}", e);
    }
}