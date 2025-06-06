#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    MovePtr(isize),
    AddData(i32),
    Output,
    Input,
    Loop(Vec<Instruction>),
    Nop,
}

impl Instruction {
    pub fn is_nop(&self) -> bool {
        matches!(self, Instruction::Nop)
    }
}
