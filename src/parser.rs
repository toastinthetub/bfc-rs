use crate::ast::Instruction;
use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Result<Vec<Instruction>, String> {
    let mut idx = 0;
    parse_inner(tokens, &mut idx)
}

fn parse_inner(tokens: &[Token], idx: &mut usize) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();
    while *idx < tokens.len() {
        match tokens[*idx] {
            Token::Plus | Token::Minus => {
                let mut count = 0;
                while *idx < tokens.len() && matches!(tokens[*idx], Token::Plus | Token::Minus) {
                    if let Token::Plus = tokens[*idx] {
                        count += 1;
                    } else {
                        count -= 1;
                    }
                    *idx += 1;
                }
                if count != 0 {
                    instructions.push(Instruction::AddData(count));
                }
            }
            Token::Right | Token::Left => {
                let mut count = 0;
                while *idx < tokens.len() && matches!(tokens[*idx], Token::Right | Token::Left) {
                    if let Token::Right = tokens[*idx] {
                        count += 1;
                    } else {
                        count -= 1;
                    }
                    *idx += 1;
                }
                if count != 0 {
                    instructions.push(Instruction::MovePtr(count));
                }
            }
            Token::Dot => {
                instructions.push(Instruction::Output);
                *idx += 1;
            }
            Token::Comma => {
                instructions.push(Instruction::Input);
                *idx += 1;
            }
            Token::LBracket => {
                *idx += 1;
                let body = parse_inner(tokens, idx)?;
                instructions.push(Instruction::Loop(body));
            }
            Token::RBracket => {
                *idx += 1;
                return Ok(instructions);
            }
        }
    }
    Ok(instructions)
}
