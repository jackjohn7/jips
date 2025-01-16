use jips::tokens::Tokens;

fn main() -> Result<(), String> {
    let content = include_str!("../examples/add.asm");
    Tokens::lex(content.to_owned())?.parse()?;

    Ok(())
}
