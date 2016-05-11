use std::vec::Vec;

pub struct Program(Vec<Line>);

pub enum Line {
    CodeLine(Option<Label>, Operator, Vec<Operand>),
    ValueDefinition(Option<Label>, Vec<Value>)
}

pub struct Label(String);

pub enum Operator {
    Mov,
    Jmp,
    Call,
    Ret
    // TODO
}

pub enum Operand {
    Register(Register),
    Direct(u8),
    IndirectReg(Register),
    IndirectSum(Register, Register)
}

pub enum Register {
    R0,
    R1,
    R2,
    // TODO
}

pub enum Value {
    Byte(u8),
    String(String)
}
