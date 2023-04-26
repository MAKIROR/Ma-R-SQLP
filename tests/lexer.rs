use masql::lexer::lex;

#[test]
fn test_lex() {
    let result = format!("{:?}", lex("SELECT * FROM customers;"));
    println!("{}", result);
}