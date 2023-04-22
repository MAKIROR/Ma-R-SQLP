use masql::lexer::lex;

#[test]
fn test_lex() {
    println!("{}", 1);
    let result = format!("{:?}", lex("SELECT * FROM customers;"));
    println!("{}", result);
}