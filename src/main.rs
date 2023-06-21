mod lexer;
mod token;

fn main() {
    let lexer = lexer::Lexer::new("asdfa", 0);
    for _ in lexer {}
    println!("Hello, world!");
}
