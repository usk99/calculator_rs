mod evaluator;
mod lexer;
mod parser;
mod ui;

fn main() {
    let input = "10 - 3";
    let tokens = lexer::Lexer::new(input).tokenize().unwrap();
    let expr = parser::Parser::new(tokens).parse_expr().unwrap();
    let result = evaluator::eval(&expr).unwrap();
    println!("{}", result);
}
