use masql::lexer::lex;
use masql::statement_parse::parse_insert;

#[test]
fn test_insert() {
    let result = lex("INSERT INTO students (name, age) VALUES ('John Doe', 21) ;");
    println!("{:?}", result);
    let ast = parse_insert(&result);
    if let Err(e) = ast {
        println!("{}", e);
    } else if let Ok(r) = ast {
        println!("{:?}", r);
    }
}