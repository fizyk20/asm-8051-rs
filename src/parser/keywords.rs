use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Acall,
    Add,
    Addc,
    Ajmp,
    Anl,
    Call,
    Cjne,
    Clr,
    Cpl,
    Da,
    Dec,
    Div,
    Djnz,
    Inc,
    Jb,
    Jbc,
    Jc,
    Jmp,
    Jnb,
    Jnc,
    Jnz,
    Jz,
    Lcall,
    Ljmp,
    Mov,
    Movc,
    Movx,
    Mul,
    Nop,
    Orl,
    Pop,
    Push,
    Ret,
    Reti,
    Rl,
    Rlc,
    Rr,
    Rrc,
    Setb,
    Sjmp,
    Subb,
    Swap,
    Xch,
    Xchd,
    Xrl,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Operator, ()> {
        match s.to_lowercase().as_ref() {
            "acall" => Ok(Operator::Acall),
            "add" => Ok(Operator::Add),
            "addc" => Ok(Operator::Addc),
            "ajmp" => Ok(Operator::Ajmp),
            "anl" => Ok(Operator::Anl),
            "call" => Ok(Operator::Call),
            "cjne" => Ok(Operator::Cjne),
            "clr" => Ok(Operator::Clr),
            "cpl" => Ok(Operator::Cpl),
            "da," => Ok(Operator::Da),
            "dec" => Ok(Operator::Dec),
            "div" => Ok(Operator::Div),
            "djnz" => Ok(Operator::Djnz),
            "inc" => Ok(Operator::Inc),
            "jb," => Ok(Operator::Jb),
            "jbc" => Ok(Operator::Jbc),
            "jc," => Ok(Operator::Jc),
            "jmp" => Ok(Operator::Jmp),
            "jnb" => Ok(Operator::Jnb),
            "jnc" => Ok(Operator::Jnc),
            "jnz" => Ok(Operator::Jnz),
            "jz," => Ok(Operator::Jz),
            "lcall" => Ok(Operator::Lcall),
            "ljmp" => Ok(Operator::Ljmp),
            "mov" => Ok(Operator::Mov),
            "movc" => Ok(Operator::Movc),
            "movx" => Ok(Operator::Movx),
            "mul" => Ok(Operator::Mul),
            "nop" => Ok(Operator::Nop),
            "orl" => Ok(Operator::Orl),
            "pop" => Ok(Operator::Pop),
            "push" => Ok(Operator::Push),
            "ret" => Ok(Operator::Ret),
            "reti" => Ok(Operator::Reti),
            "rl," => Ok(Operator::Rl),
            "rlc" => Ok(Operator::Rlc),
            "rr," => Ok(Operator::Rr),
            "rrc" => Ok(Operator::Rrc),
            "setb" => Ok(Operator::Setb),
            "sjmp" => Ok(Operator::Sjmp),
            "subb" => Ok(Operator::Subb),
            "swap" => Ok(Operator::Swap),
            "xch" => Ok(Operator::Xch),
            "xchd" => Ok(Operator::Xchd),
            "xrl" => Ok(Operator::Xrl),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Register {
    R(u8),
    A,
    B,
    C,
    PC,
    SP,
    DPTR,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Register, ()> {
        let re = Regex::new(r"^R(\d)$").unwrap();
        if let Some(caps) = re.captures(s) {
            let reg_num: u8 = caps.at(1).unwrap().parse().unwrap();
            if reg_num < 8 {
                return Ok(Register::R(reg_num));
            } else {
                return Err(());
            }
        }

        match s.to_lowercase().as_ref() {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "pc" => Ok(Register::PC),
            "sp" => Ok(Register::SP),
            "dptr" => Ok(Register::DPTR),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direct {
    Port(u8),
    PortBit(u8, u8),
}

impl FromStr for Direct {
    type Err = ();

    fn from_str(s: &str) -> Result<Direct, ()> {
        let port_re = Regex::new(r"^P(\d)$").unwrap();
        let port_bit_re = Regex::new(r"^P(\d)\.(\d)$").unwrap();

        if let Some(caps) = port_re.captures(s) {
            let port_num: u8 = caps.at(1).unwrap().parse().unwrap();
            if port_num < 7 {
                return Ok(Direct::Port(port_num));
            } else {
                return Err(());
            }
        }

        if let Some(caps) = port_bit_re.captures(s) {
            let port_num: u8 = caps.at(1).unwrap().parse().unwrap();
            let bit_num: u8 = caps.at(2).unwrap().parse().unwrap();
            if port_num < 7 && bit_num < 8 {
                return Ok(Direct::PortBit(port_num, bit_num));
            } else {
                return Err(());
            }
        }

        Err(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Definition {
    DefineByte,
    DefineWord,
}

impl FromStr for Definition {
    type Err = ();

    fn from_str(s: &str) -> Result<Definition, ()> {
        match s {
            "db" => Ok(Definition::DefineByte),
            "dw" => Ok(Definition::DefineWord),
            _ => Err(()),
        }
    }
}
