use once_cell::sync::Lazy;
use regex::Regex;
use std::str::SplitWhitespace;

pub struct Lexer {}

impl Lexer {
    pub fn new () -> Self {
        Lexer {}
    }

    pub fn lex(&self, tokens: SplitWhitespace) {
        macro_rules! regex_match {
            ($token:expr, $($pat:expr => $expr:expr),*) => {
                match () {
                    $( _ if $pat.is_match($token) => $expr, )*
                    _ => println!("token: {}", $token),
                }
            }
        }

        static INT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+$").unwrap());

        tokens.for_each(|token| {
            regex_match!(token,
                INT => println!("INT({})", token)
            );
        })
    }
}

