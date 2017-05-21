use super::{Instruction, InstructionError};
use parser::ast::{Line, LineBody, Program};
use std::collections::HashMap;

pub struct Mir {
    labels: HashMap<String, u16>,
    instructions: Vec<(u16, Instruction)>,
}

impl Mir {
    pub fn from_program(program: Program) -> Result<Self, InstructionError> {
        let mut labels = HashMap::new();
        let mut instructions = Vec::new();
        let mut current_address = 0;
        for line in program.lines.into_iter() {
            let (label, body) = match line {
                Line::OrgLine { address } => {
                    current_address = address;
                    continue;
                }
                Line::ProgramLine { label, body } => (label, body),
            };
            if let Some(label) = label {
                labels.insert(label.0, current_address);
            }
            if let Some(body) = body {
                match body {
                    LineBody::CodeLine { operator, operands } => {
                        let instruction = Instruction::from_code(operator, operands)?;
                        let len = instruction.bytes();
                        instructions.push((current_address, instruction));
                        current_address += len;
                    }
                    LineBody::ValueDefinition { values } => {
                        let mut bytes = vec![];
                        for value in values {
                            bytes.extend(value.into_bytes());
                        }
                        let len = bytes.len() as u16;
                        instructions.push((current_address, Instruction::Bytes(bytes)));
                        current_address += len;
                    }
                }
            }
        }
        Ok(Mir {
               labels: labels,
               instructions: instructions,
           })
    }
}
