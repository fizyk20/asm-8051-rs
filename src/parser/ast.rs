use std::vec::Vec;
use super::keywords::{Operator, Register};

pub struct Program(Vec<Line>);

pub enum Line {
    CodeLine(Option<Label>, Operator, Vec<Operand>),
    ValueDefinition(Option<Label>, Vec<Value>)
}

pub struct Label(String);

pub enum Operand {
    Register(Register),
    Direct(u8),
    IndirectReg(Register),
    IndirectSum(Register, Register)
}

pub enum Value {
    Byte(u8),
    String(String)
}
