mod lex;

use std::fs;
use crate::lex::Lexer;

fn main() {
    let contents = fs::read_to_string("test.blip")
        .expect("Something went wrong reading the file");
    let lexer = Lexer::new();
    lexer.lex(contents.split_whitespace());
}
