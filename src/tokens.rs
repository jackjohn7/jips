use crate::parser::Program;

struct Token;

pub struct Tokens {
    tokens: Vec<Token>,
}

pub fn tokenize(_: String) -> Result<Tokens, String> {
    todo!()
}

impl Tokens {
    pub fn parse(&self) -> Result<Program, String> {
        todo!()
    }

    pub fn lex(input: String) -> Result<Self, String> {
        tokenize(input)
    }
}
