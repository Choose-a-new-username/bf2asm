pub mod lexer;
pub mod compiler;

fn main() {
    let tokens = lexer::lex("+++++>++++++[-<++++++++++>]<.");
    let code = compiler::compile(tokens);
    print!("{}", code);
}