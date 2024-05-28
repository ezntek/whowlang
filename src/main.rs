use whowlang::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new(std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap());
    let res = lexer.lex();
    for elem in res {
        println!("{:?}", elem);
    }
}
