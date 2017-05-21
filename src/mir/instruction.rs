use parser::ast::Operand;
use parser::keywords::*;

pub enum Address {
    Number(u16),
    Label(String),
}

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
}

pub enum Instruction {
    Acall(Address),
    AddAReg(u8),
    AddADirect(u8),
    AddAIndirReg(u8),
    AddAData(u8),
    AddcAReg(u8),
    AddcADirect(u8),
    AddcAIndirReg(u8),
    AddcAData(u8),
    Ajmp(Address),
    AnlAReg(u8),
    AnlADirect(u8),
    AnlAIndirReg(u8),
    AnlAData(u8),
    AnlDirectA(u8),
    AnlDirectData(u8, u8),
    AnlCBit(u8),
    AnlCNegBit(u8),
    CjneADirRel(u8, Address),
    CjneADataRel(u8, Address),
    CJneRegDataRel(u8, u8, Address),
    CjneIndirRegDataRel(u8, u8, Address),
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
    MovAData(u8),
    MovRegA(u8),
    MovRegDir(u8, u8),
    MovRegData(u8, u8),
    MovDirectA(u8),
    MovDirectReg(u8, u8),
    MovDirectDirect(u8, u8),
    MovDirectIndirReg(u8, u8),
    MovDirectData(u8, u8),
    MovIndirRegA(u8),
    MovIndirRegDirect(u8, u8),
    MovIndirRegData(u8, u8),
    MovCBit(u8),
    MovBitC(u8),
    MovDptrData(Address),
    MovcAIndirAPlusDptr,
    MovcAIndirAPlucPC,
    MovxAIndirReg(u8),
    MovxAIndirDptr,
    MovxIndirRegA(u8),
    MovxIndirDptrA,
    MulAB,
    Nop,
    OrlAReg(u8),
    OrlADirect(u8),
    OrlAIndirReg(u8),
    OrlAData(u8),
    OrlDirectA(u8),
    OrlDirectData(u8, u8),
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
    SubbAData(u8),
    SwapA,
    XchAReg(u8),
    XchADirect(u8),
    XchAIndirReg(u8),
    XchdAIndirReg(u8),
    XrlAReg(u8),
    XrlADirect(u8),
    XrlAIndirReg(u8),
    XrlAData(u8),
    XrlDirectA(u8),
    XrlDirectData(u8, u8),
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
            Instruction::CJneRegDataRel(_, _, _) => 3,
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
            Instruction::MovcAIndirAPlucPC => 1,
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
        match operator {
            Operator::Acall => {
                Self::expect_operands(operator, &operands, 1)?;
                let address = match operands[0] {
                    Operand::Immediate(addr) => {
                        if addr < 0 || addr > 2048 {
                            return Self::invalid_operand(operator, operands[0].clone(), 0);
                        }
                        Address::Number(addr as u16)
                    }
                    Operand::ImmediateId(ref id) => Address::Label(id.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                };
                Ok(Instruction::Acall(address))
            }
            Operator::Add => {
                Self::expect_operands(operator, &operands, 2)?;
                if operands[0] != Operand::Register(Register::A) {
                    return Self::invalid_operand(operator, operands[0].clone(), 0);
                }
                match operands[1] {
                    Operand::Register(Register::R(r)) => Ok(Instruction::AddAReg(r)),
                    Operand::Direct(dir) => Ok(Instruction::AddADirect(dir)),
                    Operand::IndirectReg(Register::R(r)) => {
                        if r > 1 {
                            return Self::invalid_operand(operator, operands[1].clone(), 1);
                        }
                        Ok(Instruction::AddAIndirReg(r))
                    }
                    Operand::Immediate(imm) => {
                        if imm < -128 || imm > 255 {
                            return Self::invalid_operand(operator, operands[1].clone(), 1);
                        }
                        Ok(Instruction::AddAData(imm as u8))
                    }
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Addc => {
                Self::expect_operands(operator, &operands, 2)?;
                if operands[0] != Operand::Register(Register::A) {
                    return Self::invalid_operand(operator, operands[0].clone(), 0);
                }
                match operands[1] {
                    Operand::Register(Register::R(r)) => Ok(Instruction::AddcAReg(r)),
                    Operand::Direct(dir) => Ok(Instruction::AddcADirect(dir)),
                    Operand::IndirectReg(Register::R(r)) => {
                        if r > 1 {
                            return Self::invalid_operand(operator, operands[1].clone(), 1);
                        }
                        Ok(Instruction::AddcAIndirReg(r))
                    }
                    Operand::Immediate(imm) => {
                        if imm < -128 || imm > 255 {
                            return Self::invalid_operand(operator, operands[1].clone(), 1);
                        }
                        Ok(Instruction::AddcAData(imm as u8))
                    }
                    _ => {
                        return Self::invalid_operand(operator, operands[1].clone(), 1);
                    }
                }
            }
            Operator::Ajmp => {
                Self::expect_operands(operator, &operands, 1)?;
                let address = match operands[0] {
                    Operand::Immediate(addr) => {
                        if addr < 0 || addr > 2048 {
                            return Self::invalid_operand(operator, operands[0].clone(), 0);
                        }
                        Address::Number(addr as u16)
                    }
                    Operand::ImmediateId(ref id) => Address::Label(id.clone()),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                };
                Ok(Instruction::Ajmp(address))
            }
            Operator::Anl => {
                Self::expect_operands(operator, &operands, 2)?;
                match operands[0] {
                    Operand::Register(Register::A) |
                    Operand::Direct(_) => (),
                    _ => {
                        return Self::invalid_operand(operator, operands[0].clone(), 0);
                    }
                }
                match (&operands[0], &operands[1]) {
                    (&Operand::Register(Register::A), &Operand::Register(Register::R(r))) => {
                        Ok(Instruction::AnlAReg(r))
                    }
                    (&Operand::Register(Register::A), &Operand::Direct(addr)) => {
                        Ok(Instruction::AnlADirect(addr))
                    }
                    (&Operand::Register(Register::A), &Operand::IndirectReg(Register::R(r))) => {
                        if r > 1 {
                            return Self::invalid_operand(operator, operands[1].clone(), 1);
                        }
                        Ok(Instruction::AnlAIndirReg(r))
                    }
                    (&Operand::Register(Register::A), &Operand::Immediate(imm)) => {
                        if imm < -128 || imm > 255 {
                            return Self::invalid_operand(operator, operands[1].clone(), 1);
                        }
                        Ok(Instruction::AnlAData(imm as u8))
                    }
                    (&Operand::Direct(addr), &Operand::Register(Register::A)) => {
                        Ok(Instruction::AnlADirect(addr))
                    }
                    (&Operand::Direct(addr), &Operand::Immediate(imm)) => {
                        if imm < -128 || imm > 255 {
                            return Self::invalid_operand(operator, operands[1].clone(), 1);
                        }
                        Ok(Instruction::AnlDirectData(addr, imm as u8))
                    }
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
                Ok(Instruction::Nop)
            }
            Operator::Clr => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Cpl => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Da => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Dec => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Div => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Djnz => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Inc => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Jb => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Jbc => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Jc => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Jmp => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Jnb => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Jnc => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Jnz => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Jz => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Lcall => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Ljmp => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Mov => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Movc => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Movx => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Mul => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Nop => {
                Self::expect_operands(operator, &operands, 0)?;
                Ok(Instruction::Nop)
            }
            Operator::Orl => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Pop => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Push => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
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
                Ok(Instruction::Nop)
            }
            Operator::Rlc => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Rr => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Rrc => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Setb => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Sjmp => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Subb => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Swap => {
                Self::expect_operands(operator, &operands, 1)?;
                Ok(Instruction::Nop)
            }
            Operator::Xch => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Xchd => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
            Operator::Xrl => {
                Self::expect_operands(operator, &operands, 2)?;
                Ok(Instruction::Nop)
            }
        }
    }
}
