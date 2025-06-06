#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Right,
    Left,
    Dot,
    Comma,
    LBracket,
    RBracket,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    for c in source.chars() {
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '>' => tokens.push(Token::Right),
            '<' => tokens.push(Token::Left),
            '.' => tokens.push(Token::Dot),
            ',' => tokens.push(Token::Comma),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            _ => (), // ignore literally anything else
        }
    }
    Ok(tokens)
}
