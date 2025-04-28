mod lexer;
mod parser;
mod codegen;

fn main() {
    let source = "let x = 5;";
    let lexed = lexer::tokenize(source);
    println!("{:?}", lexed);
}
