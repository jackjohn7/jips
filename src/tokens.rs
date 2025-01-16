use logos::Logos;

use crate::parser::Program;

#[derive(Logos)]
pub enum Token {
    #[regex(r#":([a-zA-Z])"#, |lex| lex.slice().to_owned())]
    Directive(String),
    #[regex(r#"([a-z][a-zA-Z0-9]*):"#, |lex| lex.slice().to_owned())]
    Label(String),
    #[regex(r#"([a-z][a-zA-Z0-9]*)"#, |lex| lex.slice().to_owned())]
    Instruction(String),
    #[regex(r#"\$([.]*)"#, |lex| lex.slice().to_owned())]
    Register(String),
    #[regex(r#"([0-9]*)"#, |lex| lex.slice().to_owned().parse::<i32>().unwrap())]
    Literal(i32),
    #[regex(r#"#([.])*"#, |lex| lex.slice().to_owned())]
    Comment(String),
    #[token(",")]
    Comma,
    #[token("\n")]
    NewLine,
}

pub struct Tokens {
    tokens: Vec<Token>,
}

pub fn tokenize(input: String) -> Result<Tokens, String> {
    let tokens = Token::lexer(&input)
        .collect::<Result<Vec<Token>, ()>>()
        .map_err(|_| "Failed to tokenize input")?;
    return Ok(Tokens { tokens });
}

impl Tokens {
    pub fn parse(&self) -> Result<Program, String> {
        todo!()
    }

    pub fn lex(input: String) -> Result<Self, String> {
        tokenize(input)
    }
}
