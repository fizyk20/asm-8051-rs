use parser::ast::Operand;
use parser::keywords::{Operator, Register as Reg};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Address {
    Number(u16),
    Label(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Number(u8),
    Identifier(String),
}

impl Address {
    pub fn to_u16(&self, identifiers: &HashMap<String, i32>) -> Result<u16, InstructionError> {
        let x = match *self {
            Address::Number(x) => Ok(x as i32),
            Address::Label(ref s) => {
                identifiers
                    .get(s)
                    .map(|x| *x)
                    .ok_or(InstructionError::UnknownLabel(s.clone()))
            }
        }?;
        if x > 65535 || x < 0 {
            Err(InstructionError::InvalidWord(x))
        } else {
            Ok(x as u16)
        }
    }
}

impl Data {
    pub fn to_u8(&self, identifiers: &HashMap<String, i32>) -> Result<u8, InstructionError> {
        let x = match *self {
            Data::Number(x) => Ok(x as i32),
            Data::Identifier(ref s) => {
                identifiers
                    .get(s)
                    .map(|x| *x)
                    .ok_or(InstructionError::UnknownLabel(s.clone()))
            }
        }?;
        if x > 255 || x < 0 {
            Err(InstructionError::InvalidByte(x))
        } else {
            Ok(x as u8)
        }
    }
}

#[derive(Clone, Debug)]
pub enum InstructionError {
    InvalidNumOperands {
        operator: Operator,
        num: u8,
        expected: u8,
    },
    InvalidOperand {
        operator: Operator,
        operand: Operand,
        pos: u8,
    },
    InvalidByte(i32),
    InvalidWord(i32),
    DuplicateIdentifier(String),
    UnknownLabel(String),
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Acall(Address),
    AddAReg(u8),
    AddADirect(u8),
    AddAIndirReg(u8),
    AddAData(Data),
    AddcAReg(u8),
    AddcADirect(u8),
    AddcAIndirReg(u8),
    AddcAData(Data),
    Ajmp(Address),
    AnlAReg(u8),
    AnlADirect(u8),
    AnlAIndirReg(u8),
    AnlAData(Data),
    AnlDirectA(u8),
    AnlDirectData(u8, Data),
    AnlCBit(u8),
    AnlCNegBit(u8),
    CjneADirRel(u8, Address),
    CjneADataRel(Data, Address),
    CjneRegDataRel(u8, Data, Address),
    CjneIndirRegDataRel(u8, Data, Address),
    ClrA,
    ClrC,
    ClrBit(u8),
    CplA,
    CplC,
    CplBit(u8),
    DaA,
    DecA,
    DecReg(u8),
    DecDirect(u8),
    DecIndirReg(u8),
    DivAB,
    DjnzRegRel(u8, Address),
    DjnzDirectRel(u8, Address),
    IncA,
    IncReg(u8),
    IncDirect(u8),
    IncIndirReg(u8),
    IncDptr,
    JbBitRel(u8, Address),
    JbcBitRel(u8, Address),
    JcRel(Address),
    JmpIndirAPlusDptr,
    JnbBitRel(u8, Address),
    JncRel(Address),
    JnzRel(Address),
    JzRel(Address),
    Lcall(Address),
    Ljmp(Address),
    MovAReg(u8),
    MovADirect(u8),
    MovAIndirReg(u8),
    MovAData(Data),
    MovRegA(u8),
    MovRegDir(u8, u8),
    MovRegData(u8, Data),
    MovDirectA(u8),
    MovDirectReg(u8, u8),
    MovDirectDirect(u8, u8),
    MovDirectIndirReg(u8, u8),
    MovDirectData(u8, Data),
    MovIndirRegA(u8),
    MovIndirRegDirect(u8, u8),
    MovIndirRegData(u8, Data),
    MovCBit(u8),
    MovBitC(u8),
    MovDptrData(Address),
    MovcAIndirAPlusDptr,
    MovcAIndirAPlusPc,
    MovxAIndirReg(u8),
    MovxAIndirDptr,
    MovxIndirRegA(u8),
    MovxIndirDptrA,
    MulAB,
    Nop,
    OrlAReg(u8),
    OrlADirect(u8),
    OrlAIndirReg(u8),
    OrlAData(Data),
    OrlDirectA(u8),
    OrlDirectData(u8, Data),
    OrlCBit(u8),
    OrlCNegBit(u8),
    PopDirect(u8),
    PushDirect(u8),
    Ret,
    Reti,
    RlA,
    RlcA,
    RrA,
    RrcA,
    SetbC,
    SetbBit(u8),
    Sjmp(Address),
    SubbAReg(u8),
    SubbADirect(u8),
    SubbAIndirReg(u8),
    SubbAData(Data),
    SwapA,
    XchAReg(u8),
    XchADirect(u8),
    XchAIndirReg(u8),
    XchdAIndirReg(u8),
    XrlAReg(u8),
    XrlADirect(u8),
    XrlAIndirReg(u8),
    XrlAData(Data),
    XrlDirectA(u8),
    XrlDirectData(u8, Data),
    Bytes(Vec<u8>),
}

impl Instruction {
    pub fn bytes(&self) -> u16 {
        match *self {
            Instruction::Acall(_) => 2,
            Instruction::AddAReg(_) => 1,
            Instruction::AddADirect(_) => 2,
            Instruction::AddAIndirReg(_) => 1,
            Instruction::AddAData(_) => 2,
            Instruction::AddcAReg(_) => 1,
            Instruction::AddcADirect(_) => 2,
            Instruction::AddcAIndirReg(_) => 1,
            Instruction::AddcAData(_) => 2,
            Instruction::Ajmp(_) => 2,
            Instruction::AnlAReg(_) => 1,
            Instruction::AnlADirect(_) => 2,
            Instruction::AnlAIndirReg(_) => 1,
            Instruction::AnlAData(_) => 2,
            Instruction::AnlDirectA(_) => 2,
            Instruction::AnlDirectData(_, _) => 3,
            Instruction::AnlCBit(_) => 2,
            Instruction::AnlCNegBit(_) => 2,
            Instruction::CjneADirRel(_, _) => 3,
            Instruction::CjneADataRel(_, _) => 3,
            Instruction::CjneRegDataRel(_, _, _) => 3,
            Instruction::CjneIndirRegDataRel(_, _, _) => 3,
            Instruction::ClrA => 1,
            Instruction::ClrC => 1,
            Instruction::ClrBit(_) => 2,
            Instruction::CplA => 1,
            Instruction::CplC => 1,
            Instruction::CplBit(_) => 2,
            Instruction::DaA => 1,
            Instruction::DecA => 1,
            Instruction::DecReg(_) => 1,
            Instruction::DecDirect(_) => 2,
            Instruction::DecIndirReg(_) => 1,
            Instruction::DivAB => 1,
            Instruction::DjnzRegRel(_, _) => 2,
            Instruction::DjnzDirectRel(_, _) => 3,
            Instruction::IncA => 1,
            Instruction::IncReg(_) => 1,
            Instruction::IncDirect(_) => 2,
            Instruction::IncIndirReg(_) => 1,
            Instruction::IncDptr => 1,
            Instruction::JbBitRel(_, _) => 3,
            Instruction::JbcBitRel(_, _) => 3,
            Instruction::JcRel(_) => 2,
            Instruction::JmpIndirAPlusDptr => 1,
            Instruction::JnbBitRel(_, _) => 3,
            Instruction::JncRel(_) => 2,
            Instruction::JnzRel(_) => 2,
            Instruction::JzRel(_) => 2,
            Instruction::Lcall(_) => 3,
            Instruction::Ljmp(_) => 3,
            Instruction::MovAReg(_) => 1,
            Instruction::MovADirect(_) => 2,
            Instruction::MovAIndirReg(_) => 1,
            Instruction::MovAData(_) => 2,
            Instruction::MovRegA(_) => 1,
            Instruction::MovRegDir(_, _) => 2,
            Instruction::MovRegData(_, _) => 2,
            Instruction::MovDirectA(_) => 2,
            Instruction::MovDirectReg(_, _) => 2,
            Instruction::MovDirectDirect(_, _) => 3,
            Instruction::MovDirectIndirReg(_, _) => 2,
            Instruction::MovDirectData(_, _) => 3,
            Instruction::MovIndirRegA(_) => 1,
            Instruction::MovIndirRegDirect(_, _) => 2,
            Instruction::MovIndirRegData(_, _) => 2,
            Instruction::MovCBit(_) => 2,
            Instruction::MovBitC(_) => 2,
            Instruction::MovDptrData(_) => 3,
            Instruction::MovcAIndirAPlusDptr => 1,
            Instruction::MovcAIndirAPlusPc => 1,
            Instruction::MovxAIndirReg(_) => 1,
            Instruction::MovxAIndirDptr => 1,
            Instruction::MovxIndirRegA(_) => 1,
            Instruction::MovxIndirDptrA => 1,
            Instruction::MulAB => 1,
            Instruction::Nop => 1,
            Instruction::OrlAReg(_) => 1,
            Instruction::OrlADirect(_) => 2,
            Instruction::OrlAIndirReg(_) => 1,
            Instruction::OrlAData(_) => 2,
            Instruction::OrlDirectA(_) => 2,
            Instruction::OrlDirectData(_, _) => 3,
            Instruction::OrlCBit(_) => 2,
            Instruction::OrlCNegBit(_) => 2,
            Instruction::PopDirect(_) => 2,
            Instruction::PushDirect(_) => 2,
            Instruction::Ret => 1,
            Instruction::Reti => 1,
            Instruction::RlA => 1,
            Instruction::RlcA => 1,
            Instruction::RrA => 1,
            Instruction::RrcA => 1,
            Instruction::SetbC => 1,
            Instruction::SetbBit(_) => 2,
            Instruction::Sjmp(_) => 2,
            Instruction::SubbAReg(_) => 1,
            Instruction::SubbADirect(_) => 2,
            Instruction::SubbAIndirReg(_) => 1,
            Instruction::SubbAData(_) => 2,
            Instruction::SwapA => 1,
            Instruction::XchAReg(_) => 1,
            Instruction::XchADirect(_) => 2,
            Instruction::XchAIndirReg(_) => 1,
            Instruction::XchdAIndirReg(_) => 1,
            Instruction::XrlAReg(_) => 1,
            Instruction::XrlADirect(_) => 2,
            Instruction::XrlAIndirReg(_) => 1,
            Instruction::XrlAData(_) => 2,
            Instruction::XrlDirectA(_) => 2,
            Instruction::XrlDirectData(_, _) => 3,
            Instruction::Bytes(ref v) => v.len() as u16,
        }
    }

    fn expect_operands(operator: Operator,
                       operands: &Vec<Operand>,
                       num: u8)
                       -> Result<(), InstructionError> {
        if operands.len() as u8 == num {
            Ok(())
        } else {
            Err(InstructionError::InvalidNumOperands {
                    operator: operator,
                    num: operands.len() as u8,
                    expected: num,
                })
        }
    }

    fn invalid_operand(operator: Operator,
                       operand: Operand,
                       pos: u8)
                       -> Result<Instruction, InstructionError> {
        Err(InstructionError::InvalidOperand {
                operator: operator,
                operand: operand,
                pos: pos,
            })
    }

    pub fn from_code(operator: Operator,
                     operands: Vec<Operand>)
                     -> Result<Instruction, InstructionError> {
        use self::Operand::*;
        match operator {
            Operator::Acall => {
                Self::expect_operands(operator, &operands, 1)?;
                let address = match operands[0] {
                    Immediate(addr) if addr >= 0 && addr <= 2048 => Address::Number(addr as u16),
                    ImmediateId(ref id) => Address::Label(id.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                };
                Ok(Instruction::Acall(address))
            }
            Operator::Add => {
                Self::expect_operands(operator, &operands, 2)?;
                if operands[0] != Operand::Register(Reg::A) {
                    return Self::invalid_operand(operator, operands[0].clone(), 0);
                }
                match operands[1] {
                    Register(Reg::R(r)) => Ok(Instruction::AddAReg(r)),
                    Direct(dir) => Ok(Instruction::AddADirect(dir)),
                    IndirectReg(Reg::R(r)) if r < 2 => Ok(Instruction::AddAIndirReg(r)),
                    Immediate(imm) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::AddAData(Data::Number(imm as u8)))
                    }
                    ImmediateId(ref id) => Ok(Instruction::AddAData(Data::Identifier(id.clone()))),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Addc => {
                Self::expect_operands(operator, &operands, 2)?;
                if operands[0] != Operand::Register(Reg::A) {
                    return Self::invalid_operand(operator, operands[0].clone(), 0);
                }
                match operands[1] {
                    Register(Reg::R(r)) => Ok(Instruction::AddcAReg(r)),
                    Direct(dir) => Ok(Instruction::AddcADirect(dir)),
                    IndirectReg(Reg::R(r)) if r < 2 => Ok(Instruction::AddcAIndirReg(r)),
                    Immediate(imm) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::AddcAData(Data::Number(imm as u8)))
                    }
                    ImmediateId(ref id) => Ok(Instruction::AddcAData(Data::Identifier(id.clone()))),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Ajmp => {
                Self::expect_operands(operator, &operands, 1)?;
                let address = match operands[0] {
                    Immediate(addr) if addr >= 0 && addr <= 2048 => Address::Number(addr as u16),
                    ImmediateId(ref id) => Address::Label(id.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                };
                Ok(Instruction::Ajmp(address))
            }
            Operator::Anl => {
                Self::expect_operands(operator, &operands, 2)?;
                match operands[0] {
                    Register(Reg::A) | Register(Reg::C) | Direct(_) => (),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
                match (&operands[0], &operands[1]) {
                    (&Register(Reg::A), &Register(Reg::R(r))) => Ok(Instruction::AnlAReg(r)),
                    (&Register(Reg::A), &Direct(addr)) => Ok(Instruction::AnlADirect(addr)),
                    (&Register(Reg::A), &IndirectReg(Reg::R(r))) if r < 2 => {
                        Ok(Instruction::AnlAIndirReg(r))
                    }
                    (&Register(Reg::A), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::AnlAData(Data::Number(imm as u8)))
                    }
                    (&Register(Reg::A), &ImmediateId(ref id)) => {
                        Ok(Instruction::AnlAData(Data::Identifier(id.clone())))
                    }
                    (&Direct(addr), &Register(Reg::A)) => Ok(Instruction::AnlDirectA(addr)),
                    (&Direct(addr), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::AnlDirectData(addr, Data::Number(imm as u8)))
                    }
                    (&Direct(addr), &ImmediateId(ref id)) => {
                        Ok(Instruction::AnlDirectData(addr, Data::Identifier(id.clone())))
                    }
                    (&Register(Reg::C), &DirectBit(addr)) => Ok(Instruction::AnlCBit(addr)),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Call => {
                panic!("CALL not yet supported!");
            }
            Operator::Cjne => {
                Self::expect_operands(operator, &operands, 3)?;
                match operands[0] {
                    Register(Reg::A) |
                    Register(Reg::R(_)) |
                    IndirectReg(_) => (),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
                let rel = match operands[2] {
                    ImmediateId(ref addr) => Address::Label(addr.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[2].clone(), 2);
                    }
                };
                match (&operands[0], &operands[1]) {
                    (&Register(Reg::A), &Direct(addr)) => Ok(Instruction::CjneADirRel(addr, rel)),
                    (&Register(Reg::A), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::CjneADataRel(Data::Number(imm as u8), rel))
                    }
                    (&Register(Reg::A), &ImmediateId(ref id)) => {
                        Ok(Instruction::CjneADataRel(Data::Identifier(id.clone()), rel))
                    }
                    (&Register(Reg::R(r)), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::CjneRegDataRel(r, Data::Number(imm as u8), rel))
                    }
                    (&Register(Reg::R(r)), &ImmediateId(ref id)) => {
                        Ok(Instruction::CjneRegDataRel(r, Data::Identifier(id.clone()), rel))
                    }
                    (&IndirectReg(Reg::R(r)), &Immediate(imm)) if imm >= -128 && imm <= 255 &&
                                                                  r < 2 => {
                        Ok(Instruction::CjneIndirRegDataRel(r, Data::Number(imm as u8), rel))
                    }
                    (&IndirectReg(Reg::R(r)), &ImmediateId(ref id)) if r < 2 => {
                        Ok(Instruction::CjneIndirRegDataRel(r, Data::Identifier(id.clone()), rel))
                    }
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Clr => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::A) => Ok(Instruction::ClrA),
                    Register(Reg::C) => Ok(Instruction::ClrC),
                    DirectBit(addr) => Ok(Instruction::ClrBit(addr)),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Cpl => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::A) => Ok(Instruction::CplA),
                    Register(Reg::C) => Ok(Instruction::CplC),
                    DirectBit(addr) => Ok(Instruction::CplBit(addr)),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Da => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::A) => Ok(Instruction::DaA),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Dec => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::A) => Ok(Instruction::DecA),
                    Register(Reg::R(r)) => Ok(Instruction::DecReg(r)),
                    Direct(addr) => Ok(Instruction::DecDirect(addr)),
                    IndirectReg(Reg::R(r)) => Ok(Instruction::DecIndirReg(r)),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Div => {
                Self::expect_operands(operator, &operands, 2)?;
                match operands[0] {
                    Register(Reg::A) => {
                        if operands[1] == Direct(0xF0) {
                            Ok(Instruction::DivAB)
                        } else {
                            return Self::invalid_operand(operator, operands[1].clone(), 1);
                        }
                    }
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Djnz => {
                Self::expect_operands(operator, &operands, 2)?;
                let rel = match operands[1] {
                    ImmediateId(ref addr) => Address::Label(addr.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                };
                match operands[0] {
                    Register(Reg::R(r)) => Ok(Instruction::DjnzRegRel(r, rel)),
                    Direct(addr) => Ok(Instruction::DjnzDirectRel(addr, rel)),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Inc => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::A) => Ok(Instruction::IncA),
                    Register(Reg::R(r)) => Ok(Instruction::IncReg(r)),
                    Direct(addr) => Ok(Instruction::IncDirect(addr)),
                    IndirectReg(Reg::R(r)) => Ok(Instruction::IncIndirReg(r)),
                    Register(Reg::DPTR) => Ok(Instruction::IncDptr),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Jb => {
                Self::expect_operands(operator, &operands, 2)?;
                let rel = match operands[1] {
                    ImmediateId(ref addr) => Address::Label(addr.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                };
                match operands[0] {
                    DirectBit(addr) => Ok(Instruction::JbBitRel(addr, rel)),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Jbc => {
                Self::expect_operands(operator, &operands, 2)?;
                let rel = match operands[1] {
                    ImmediateId(ref addr) => Address::Label(addr.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                };
                match operands[0] {
                    DirectBit(addr) => Ok(Instruction::JbcBitRel(addr, rel)),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Jc => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    ImmediateId(ref addr) => Ok(Instruction::JcRel(Address::Label(addr.clone()))),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Jmp => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    IndirectSum(Reg::A, Reg::DPTR) => Ok(Instruction::JmpIndirAPlusDptr),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Jnb => {
                Self::expect_operands(operator, &operands, 2)?;
                let rel = match operands[1] {
                    ImmediateId(ref addr) => Address::Label(addr.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                };
                match operands[0] {
                    DirectBit(addr) => Ok(Instruction::JnbBitRel(addr, rel)),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Jnc => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    ImmediateId(ref addr) => Ok(Instruction::JncRel(Address::Label(addr.clone()))),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Jnz => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    ImmediateId(ref addr) => Ok(Instruction::JnzRel(Address::Label(addr.clone()))),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Jz => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    ImmediateId(ref addr) => Ok(Instruction::JzRel(Address::Label(addr.clone()))),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Lcall => {
                Self::expect_operands(operator, &operands, 1)?;
                let address = match operands[0] {
                    Immediate(addr) => Address::Number(addr as u16),
                    ImmediateId(ref id) => Address::Label(id.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                };
                Ok(Instruction::Lcall(address))
            }
            Operator::Ljmp => {
                Self::expect_operands(operator, &operands, 1)?;
                let address = match operands[0] {
                    Immediate(addr) => Address::Number(addr as u16),
                    ImmediateId(ref id) => Address::Label(id.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                };
                Ok(Instruction::Ljmp(address))
            }
            Operator::Mov => {
                Self::expect_operands(operator, &operands, 2)?;
                match operands[0] {
                    Register(Reg::A) |
                    Register(Reg::C) |
                    Register(Reg::DPTR) |
                    Register(Reg::R(_)) |
                    Direct(_) => (),
                    DirectBit(_) => (),
                    IndirectReg(Reg::R(r)) if r < 2 => (),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
                match (&operands[0], &operands[1]) {
                    (&Register(Reg::A), &Register(Reg::R(r))) => Ok(Instruction::MovAReg(r)),
                    (&Register(Reg::A), &Direct(addr)) if addr != 0xE0 => {
                        Ok(Instruction::MovADirect(addr))
                    }
                    (&Register(Reg::A), &IndirectReg(Reg::R(r))) if r < 2 => {
                        Ok(Instruction::MovAIndirReg(r))
                    }
                    (&Register(Reg::A), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::MovAData(Data::Number(imm as u8)))
                    }
                    (&Register(Reg::A), &ImmediateId(ref id)) => {
                        Ok(Instruction::MovAData(Data::Identifier(id.clone())))
                    }
                    (&Register(Reg::R(r)), &Register(Reg::A)) => Ok(Instruction::MovRegA(r)),
                    (&Register(Reg::R(r)), &Direct(addr)) => Ok(Instruction::MovRegDir(r, addr)),
                    (&Register(Reg::R(r)), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::MovRegData(r, Data::Number(imm as u8)))
                    }
                    (&Register(Reg::R(r)), &ImmediateId(ref id)) => {
                        Ok(Instruction::MovRegData(r, Data::Identifier(id.clone())))
                    }
                    (&Direct(addr), &Register(Reg::A)) if addr != 0xE0 => {
                        Ok(Instruction::MovDirectA(addr))
                    }
                    (&Direct(addr), &Register(Reg::R(r))) => Ok(Instruction::MovDirectReg(addr, r)),
                    (&Direct(addr), &Direct(addr2)) => {
                        Ok(Instruction::MovDirectDirect(addr, addr2))
                    }
                    (&Direct(addr), &IndirectReg(Reg::R(r))) if r < 2 => {
                        Ok(Instruction::MovDirectIndirReg(addr, r))
                    }
                    (&Direct(addr), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::MovDirectData(addr, Data::Number(imm as u8)))
                    }
                    (&Direct(addr), &ImmediateId(ref id)) => {
                        Ok(Instruction::MovDirectData(addr, Data::Identifier(id.clone())))
                    }
                    (&IndirectReg(Reg::R(r)), &Register(Reg::A)) if r < 2 => {
                        Ok(Instruction::MovIndirRegA(r))
                    }
                    (&IndirectReg(Reg::R(r)), &Direct(addr)) if r < 2 => {
                        Ok(Instruction::MovIndirRegDirect(r, addr))
                    }
                    (&IndirectReg(Reg::R(r)), &Immediate(imm)) if r < 2 && imm >= -128 &&
                                                                  imm <= 255 => {
                        Ok(Instruction::MovIndirRegData(r, Data::Number(imm as u8)))
                    }
                    (&IndirectReg(Reg::R(r)), &ImmediateId(ref id)) if r < 2 => {
                        Ok(Instruction::MovIndirRegData(r, Data::Identifier(id.clone())))
                    }
                    (&Register(Reg::C), &DirectBit(addr)) => Ok(Instruction::MovCBit(addr)),
                    (&DirectBit(addr), &Register(Reg::C)) => Ok(Instruction::MovBitC(addr)),
                    (&Register(Reg::DPTR), &Immediate(addr)) if addr >= 0 && addr < 65536 => {
                        Ok(Instruction::MovDptrData(Address::Number(addr as u16)))
                    }
                    (&Register(Reg::DPTR), &ImmediateId(ref addr)) => {
                        Ok(Instruction::MovDptrData(Address::Label(addr.clone())))
                    }
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Movc => {
                Self::expect_operands(operator, &operands, 2)?;
                match operands[0] {
                    Register(Reg::A) => (),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
                match operands[1] {
                    IndirectSum(Reg::A, Reg::DPTR) => Ok(Instruction::MovcAIndirAPlusDptr),
                    IndirectSum(Reg::A, Reg::PC) => Ok(Instruction::MovcAIndirAPlusPc),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Movx => {
                Self::expect_operands(operator, &operands, 2)?;
                match operands[0] {
                    Register(Reg::A) |
                    IndirectReg(Reg::DPTR) => (),
                    IndirectReg(Reg::R(r)) if r < 2 => (),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
                match (&operands[0], &operands[1]) {
                    (&Register(Reg::A), &IndirectReg(Reg::R(r))) if r < 2 => {
                        Ok(Instruction::MovxAIndirReg(r))
                    }
                    (&Register(Reg::A), &IndirectReg(Reg::DPTR)) => Ok(Instruction::MovxAIndirDptr),
                    (&IndirectReg(Reg::R(r)), &Register(Reg::A)) if r < 2 => {
                        Ok(Instruction::MovxIndirRegA(r))
                    }
                    (&IndirectReg(Reg::DPTR), &Register(Reg::A)) => Ok(Instruction::MovxIndirDptrA),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Mul => {
                Self::expect_operands(operator, &operands, 2)?;
                match operands[0] {
                    Register(Reg::A) => {
                        if operands[1] == Direct(0xF0) {
                            Ok(Instruction::MulAB)
                        } else {
                            return Self::invalid_operand(operator, operands[1].clone(), 1);
                        }
                    }
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Nop => {
                Self::expect_operands(operator, &operands, 0)?;
                Ok(Instruction::Nop)
            }
            Operator::Orl => {
                Self::expect_operands(operator, &operands, 2)?;
                match operands[0] {
                    Register(Reg::A) | Register(Reg::C) | Direct(_) => (),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
                match (&operands[0], &operands[1]) {
                    (&Register(Reg::A), &Register(Reg::R(r))) => Ok(Instruction::OrlAReg(r)),
                    (&Register(Reg::A), &Direct(addr)) => Ok(Instruction::OrlADirect(addr)),
                    (&Register(Reg::A), &IndirectReg(Reg::R(r))) if r < 2 => {
                        Ok(Instruction::OrlAIndirReg(r))
                    }
                    (&Register(Reg::A), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::OrlAData(Data::Number(imm as u8)))
                    }
                    (&Register(Reg::A), &ImmediateId(ref id)) => {
                        Ok(Instruction::OrlAData(Data::Identifier(id.clone())))
                    }
                    (&Direct(addr), &Register(Reg::A)) => Ok(Instruction::OrlDirectA(addr)),
                    (&Direct(addr), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::OrlDirectData(addr, Data::Number(imm as u8)))
                    }
                    (&Direct(addr), &ImmediateId(ref id)) => {
                        Ok(Instruction::OrlDirectData(addr, Data::Identifier(id.clone())))
                    }
                    (&Register(Reg::C), &DirectBit(addr)) => Ok(Instruction::OrlCBit(addr)),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Pop => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Direct(addr) => Ok(Instruction::PopDirect(addr)),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Push => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Direct(addr) => Ok(Instruction::PushDirect(addr)),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Ret => {
                Self::expect_operands(operator, &operands, 0)?;
                Ok(Instruction::Ret)
            }
            Operator::Reti => {
                Self::expect_operands(operator, &operands, 0)?;
                Ok(Instruction::Reti)
            }
            Operator::Rl => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::A) => Ok(Instruction::RlA),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Rlc => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::A) => Ok(Instruction::RlcA),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Rr => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::A) => Ok(Instruction::RrA),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Rrc => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::A) => Ok(Instruction::RrcA),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Setb => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::C) => Ok(Instruction::SetbC),
                    DirectBit(addr) => Ok(Instruction::SetbBit(addr)),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Sjmp => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    ImmediateId(ref addr) => Ok(Instruction::Sjmp(Address::Label(addr.clone()))),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Subb => {
                Self::expect_operands(operator, &operands, 2)?;
                if operands[0] != Operand::Register(Reg::A) {
                    return Self::invalid_operand(operator, operands[0].clone(), 0);
                }
                match operands[1] {
                    Register(Reg::R(r)) => Ok(Instruction::SubbAReg(r)),
                    Direct(dir) => Ok(Instruction::SubbADirect(dir)),
                    IndirectReg(Reg::R(r)) if r < 2 => Ok(Instruction::SubbAIndirReg(r)),
                    Immediate(imm) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::SubbAData(Data::Number(imm as u8)))
                    }
                    ImmediateId(ref id) => Ok(Instruction::SubbAData(Data::Identifier(id.clone()))),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Swap => {
                Self::expect_operands(operator, &operands, 1)?;
                match operands[0] {
                    Register(Reg::A) => Ok(Instruction::SwapA),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
            }
            Operator::Xch => {
                Self::expect_operands(operator, &operands, 2)?;
                if operands[0] != Operand::Register(Reg::A) {
                    return Self::invalid_operand(operator, operands[0].clone(), 0);
                }
                match operands[1] {
                    Register(Reg::R(r)) => Ok(Instruction::XchAReg(r)),
                    Direct(dir) => Ok(Instruction::XchADirect(dir)),
                    IndirectReg(Reg::R(r)) if r < 2 => Ok(Instruction::XchAIndirReg(r)),
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Xchd => {
                Self::expect_operands(operator, &operands, 2)?;
                if operands[0] != Operand::Register(Reg::A) {
                    return Self::invalid_operand(operator, operands[0].clone(), 0);
                }
                match (&operands[0], &operands[1]) {
                    (&Register(Reg::A), &IndirectReg(Reg::R(r))) if r < 2 => {
                        Ok(Instruction::XchdAIndirReg(r))
                    }
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Xrl => {
                Self::expect_operands(operator, &operands, 2)?;
                match operands[0] {
                    Register(Reg::A) | Direct(_) => (),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
                match (&operands[0], &operands[1]) {
                    (&Register(Reg::A), &Register(Reg::R(r))) => Ok(Instruction::XrlAReg(r)),
                    (&Register(Reg::A), &Direct(addr)) => Ok(Instruction::XrlADirect(addr)),
                    (&Register(Reg::A), &IndirectReg(Reg::R(r))) if r < 2 => {
                        Ok(Instruction::XrlAIndirReg(r))
                    }
                    (&Register(Reg::A), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::XrlAData(Data::Number(imm as u8)))
                    }
                    (&Register(Reg::A), &ImmediateId(ref id)) => {
                        Ok(Instruction::XrlAData(Data::Identifier(id.clone())))
                    }
                    (&Direct(addr), &Register(Reg::A)) => Ok(Instruction::XrlDirectA(addr)),
                    (&Direct(addr), &Immediate(imm)) if imm >= -128 && imm <= 255 => {
                        Ok(Instruction::XrlDirectData(addr, Data::Number(imm as u8)))
                    }
                    (&Direct(addr), &ImmediateId(ref id)) => {
                        Ok(Instruction::XrlDirectData(addr, Data::Identifier(id.clone())))
                    }
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
        }
    }

    pub fn to_bytes(&self,
                    identifiers: &HashMap<String, i32>,
                    cur_addr: u16)
                    -> Result<Vec<u8>, InstructionError> {
        match *self {
            Instruction::Acall(ref addr) => {
                let addr = addr.to_u16(identifiers)?;
                Ok(vec![((addr >> 3) & 0xE0) as u8 | 0x11, (addr & 0xFF) as u8])
            }
            Instruction::AddAReg(r) => Ok(vec![0x28 | r]),
            Instruction::AddADirect(dir) => Ok(vec![0x25, dir]),
            Instruction::AddAIndirReg(r) => Ok(vec![0x26 | r]),
            Instruction::AddAData(ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x24, data])
            }
            Instruction::AddcAReg(r) => Ok(vec![0x38 | r]),
            Instruction::AddcADirect(dir) => Ok(vec![0x35, dir]),
            Instruction::AddcAIndirReg(r) => Ok(vec![0x36 | r]),
            Instruction::AddcAData(ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x34, data])
            }
            Instruction::Ajmp(ref addr) => {
                let addr = addr.to_u16(identifiers)?;
                Ok(vec![((addr >> 3) & 0xE0) as u8 | 0x01, (addr & 0xFF) as u8])
            }
            Instruction::AnlAReg(r) => Ok(vec![0x58 | r]),
            Instruction::AnlADirect(dir) => Ok(vec![0x55, dir]),
            Instruction::AnlAIndirReg(r) => Ok(vec![0x56 | r]),
            Instruction::AnlAData(ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x54, data])
            }
            Instruction::AnlDirectA(dir) => Ok(vec![0x52, dir]),
            Instruction::AnlDirectData(dir, ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x53, dir, data])
            }
            Instruction::AnlCBit(bit) => Ok(vec![0x82, bit]),
            Instruction::AnlCNegBit(bit) => Ok(vec![0xB0, bit]),
            Instruction::CjneADirRel(dir, ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 3);
                Ok(vec![0xB5, dir, addr as u8])
            }
            Instruction::CjneADataRel(ref data, ref addr) => {
                let data = data.to_u8(identifiers)?;
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 3);
                Ok(vec![0xB4, data, addr as u8])
            }
            Instruction::CjneRegDataRel(r, ref data, ref addr) => {
                let data = data.to_u8(identifiers)?;
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 3);
                Ok(vec![0xB8 | r, data, addr as u8])
            }
            Instruction::CjneIndirRegDataRel(r, ref data, ref addr) => {
                let data = data.to_u8(identifiers)?;
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 3);
                Ok(vec![0xB6 | r, data, addr as u8])
            }
            Instruction::ClrA => Ok(vec![0xE4]),
            Instruction::ClrC => Ok(vec![0xC3]),
            Instruction::ClrBit(bit) => Ok(vec![0xC2, bit]),
            Instruction::CplA => Ok(vec![0xF4]),
            Instruction::CplC => Ok(vec![0xB3]),
            Instruction::CplBit(bit) => Ok(vec![0xB2, bit]),
            Instruction::DaA => Ok(vec![0xD4]),
            Instruction::DecA => Ok(vec![0x14]),
            Instruction::DecReg(r) => Ok(vec![0x18 | r]),
            Instruction::DecDirect(dir) => Ok(vec![0x15, dir]),
            Instruction::DecIndirReg(r) => Ok(vec![0x16 | r]),
            Instruction::DivAB => Ok(vec![0x84]),
            Instruction::DjnzRegRel(r, ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 2);
                Ok(vec![0xD8 | r, addr as u8])
            }
            Instruction::DjnzDirectRel(dir, ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 3);
                Ok(vec![0xD5, dir, addr as u8])
            }
            Instruction::IncA => Ok(vec![0x04]),
            Instruction::IncReg(r) => Ok(vec![0x08 | r]),
            Instruction::IncDirect(dir) => Ok(vec![0x05, dir]),
            Instruction::IncIndirReg(r) => Ok(vec![0x06 | r]),
            Instruction::IncDptr => Ok(vec![0xA3]),
            Instruction::JbBitRel(bit, ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 3);
                Ok(vec![0x20, bit, addr as u8])
            }
            Instruction::JbcBitRel(bit, ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 3);
                Ok(vec![0x10, bit, addr as u8])
            }
            Instruction::JcRel(ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 2);
                Ok(vec![0x40, addr as u8])
            }
            Instruction::JmpIndirAPlusDptr => Ok(vec![0x73]),
            Instruction::JnbBitRel(bit, ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 3);
                Ok(vec![0x30, bit, addr as u8])
            }
            Instruction::JncRel(ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 2);
                Ok(vec![0x50, addr as u8])
            }
            Instruction::JnzRel(ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 2);
                Ok(vec![0x70, addr as u8])
            }
            Instruction::JzRel(ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 2);
                Ok(vec![0x60, addr as u8])
            }
            Instruction::Lcall(ref addr) => {
                let addr = addr.to_u16(identifiers)?;
                Ok(vec![0x12, (addr / 256) as u8, (addr % 256) as u8])
            }
            Instruction::Ljmp(ref addr) => {
                let addr = addr.to_u16(identifiers)?;
                Ok(vec![0x02, (addr / 256) as u8, (addr % 256) as u8])
            }
            Instruction::MovAReg(r) => Ok(vec![0xE8 | r]),
            Instruction::MovADirect(dir) => Ok(vec![0xE5, dir]),
            Instruction::MovAIndirReg(r) => Ok(vec![0xE6 | r]),
            Instruction::MovAData(ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x74, data])
            }
            Instruction::MovRegA(r) => Ok(vec![0xF8 | r]),
            Instruction::MovRegDir(r, dir) => Ok(vec![0xC8 | r, dir]),
            Instruction::MovRegData(r, ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x78 | r, data])
            }
            Instruction::MovDirectA(dir) => Ok(vec![0xF5, dir]),
            Instruction::MovDirectReg(dir, r) => Ok(vec![0x88 | r, dir]),
            Instruction::MovDirectDirect(dir, dir2) => Ok(vec![0x85, dir2, dir]),
            Instruction::MovDirectIndirReg(dir, r) => Ok(vec![0x86 | r, dir]),
            Instruction::MovDirectData(dir, ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x75, dir, data])
            }
            Instruction::MovIndirRegA(r) => Ok(vec![0xF6 | r]),
            Instruction::MovIndirRegDirect(r, dir) => Ok(vec![0xA6 | r, dir]),
            Instruction::MovIndirRegData(r, ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x76 | r, data])
            }
            Instruction::MovCBit(bit) => Ok(vec![0xA2, bit]),
            Instruction::MovBitC(bit) => Ok(vec![0x92, bit]),
            Instruction::MovDptrData(ref addr) => {
                let addr = addr.to_u16(identifiers)?;
                Ok(vec![0x90, (addr / 256) as u8, (addr % 256) as u8])
            }
            Instruction::MovcAIndirAPlusDptr => Ok(vec![0x93]),
            Instruction::MovcAIndirAPlusPc => Ok(vec![0x83]),
            Instruction::MovxAIndirReg(r) => Ok(vec![0xE2 | r]),
            Instruction::MovxAIndirDptr => Ok(vec![0xE0]),
            Instruction::MovxIndirRegA(r) => Ok(vec![0xF2 | r]),
            Instruction::MovxIndirDptrA => Ok(vec![0xF0]),
            Instruction::MulAB => Ok(vec![0xA4]),
            Instruction::Nop => Ok(vec![0x00]),
            Instruction::OrlAReg(r) => Ok(vec![0x48 | r]),
            Instruction::OrlADirect(dir) => Ok(vec![0x45, dir]),
            Instruction::OrlAIndirReg(r) => Ok(vec![0x46 | r]),
            Instruction::OrlAData(ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x44, data])
            }
            Instruction::OrlDirectA(dir) => Ok(vec![0x42, dir]),
            Instruction::OrlDirectData(dir, ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x43, dir, data])
            }
            Instruction::OrlCBit(bit) => Ok(vec![0x72, bit]),
            Instruction::OrlCNegBit(bit) => Ok(vec![0xA0, bit]),
            Instruction::PopDirect(dir) => Ok(vec![0xD0, dir]),
            Instruction::PushDirect(dir) => Ok(vec![0xC0, dir]),
            Instruction::Ret => Ok(vec![0x22]),
            Instruction::Reti => Ok(vec![0x32]),
            Instruction::RlA => Ok(vec![0x23]),
            Instruction::RlcA => Ok(vec![0x33]),
            Instruction::RrA => Ok(vec![0x03]),
            Instruction::RrcA => Ok(vec![0x13]),
            Instruction::SetbC => Ok(vec![0xD3]),
            Instruction::SetbBit(bit) => Ok(vec![0xD2, bit]),
            Instruction::Sjmp(ref addr) => {
                let addr = addr.to_u16(identifiers)?.wrapping_sub(cur_addr + 2);
                Ok(vec![0x80, addr as u8])
            }
            Instruction::SubbAReg(r) => Ok(vec![0x98 | r]),
            Instruction::SubbADirect(dir) => Ok(vec![0x95, dir]),
            Instruction::SubbAIndirReg(r) => Ok(vec![0x96 | r]),
            Instruction::SubbAData(ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x94, data])
            }
            Instruction::SwapA => Ok(vec![0xC4]),
            Instruction::XchAReg(r) => Ok(vec![0xC8 | r]),
            Instruction::XchADirect(dir) => Ok(vec![0xC5, dir]),
            Instruction::XchAIndirReg(r) => Ok(vec![0xC6 | r]),
            Instruction::XchdAIndirReg(r) => Ok(vec![0xD6 | r]),
            Instruction::XrlAReg(r) => Ok(vec![0x68 | r]),
            Instruction::XrlADirect(dir) => Ok(vec![0x65, dir]),
            Instruction::XrlAIndirReg(r) => Ok(vec![0x66 | r]),
            Instruction::XrlAData(ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x64, data])
            }
            Instruction::XrlDirectA(dir) => Ok(vec![0x62, dir]),
            Instruction::XrlDirectData(dir, ref data) => {
                let data = data.to_u8(identifiers)?;
                Ok(vec![0x63, dir, data])
            }
            Instruction::Bytes(ref b) => Ok(b.clone()),
        }
    }
}
