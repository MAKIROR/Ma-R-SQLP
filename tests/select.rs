use masql::lexer::lex;
use masql::parser::statement_parser::parse_select;

#[test]
fn test_insert() {
    let result = lex("
    SELECT DISTINCT SUM(score) AS score, age
        FROM students, teachers
        WHERE @age = (1+2) * SUM(score) 
        GROUP BY name, age 
        HAVING age > 14
        ORDER BY name ASC, age DESC;
    ");
    println!("{:?}", result);
    let ast = parse_select(&result);
    if let Err(e) = ast {
        println!("{}", e);
    } else if let Ok(r) = ast {
        println!("{:?}", r);
    }
}