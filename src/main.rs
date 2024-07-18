use whowlang::{lexer::Lexer, parser::Parser};

fn main() {
    let mut lexer = Lexer::new(std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap());
    let res = lexer.lex();
    res.iter().for_each(|t| println!("{:?}", t));
    let parser = Parser::new(res);
    let parsed = parser.parse();
    let json = Parser::to_json(&parsed);
    let s = json.to_string();
    println!("{s}");
}
