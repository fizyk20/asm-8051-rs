use super::{Instruction, InstructionError};
use parser::ast::{Line, LineBody, Program};
use std::collections::HashMap;

#[derive(Debug)]
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

    fn intel_hex(addr: u16, bytes: Vec<u8>) -> String {
        let mut result = format!(":{:02X}{:04X}00", bytes.len() as u8, addr);
        let mut sum: u8 = (bytes.len() as u8)
            .wrapping_add((addr / 256) as u8)
            .wrapping_add((addr % 256) as u8);
        for b in bytes.into_iter() {
            result.push_str(&format!("{:02X}", b));
            sum = sum.wrapping_add(b);
        }
        result.push_str(&format!("{:02X}\n", (0x100 - sum as u16) as u8));
        result
    }

    pub fn gen_intel_hex(&self) -> String {
        let mut result = String::new();
        for &(addr, ref instruction) in self.instructions.iter() {
            let bytes = instruction
                .to_bytes(&self.labels, addr)
                .unwrap_or_else(|e| panic!("ERROR: {:?}", e));
            result.push_str(&Self::intel_hex(addr, bytes));
        }
        result.push_str(":00000001FF\n");
        result
    }
}
