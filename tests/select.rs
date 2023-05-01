use masql::lexer::lex;
use masql::statement_parse::parse_select;

#[test]
fn test_insert() {
    let result = lex("SELECT name, age FROM students ;");
    println!("{:?}", result);
    let ast = parse_select(&result);
    if let Err(e) = ast {
        println!("{}", e);
    } else if let Ok(r) = ast {
        println!("{:?}", r);
    }
}