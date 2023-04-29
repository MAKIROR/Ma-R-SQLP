use masql::lexer::lex;
use masql::statement_parse::*;

#[test]
fn test_lex() {
    let result = lex("SELECT * FROM customers WHERE age = 21;");
    println!("{:?}", result);
    let ast = parse_select(&result);
    if let Err(e) = ast {
        println!("{}", e);
    } else if let Ok(r) = ast {
        println!("{:?}", r);
    }
}