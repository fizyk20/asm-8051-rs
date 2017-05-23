use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
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
            "jb" => Ok(Operator::Jb),
            "jbc" => Ok(Operator::Jbc),
            "jc" => Ok(Operator::Jc),
            "jmp" => Ok(Operator::Jmp),
            "jnb" => Ok(Operator::Jnb),
            "jnc" => Ok(Operator::Jnc),
            "jnz" => Ok(Operator::Jnz),
            "jz" => Ok(Operator::Jz),
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
            "rl" => Ok(Operator::Rl),
            "rlc" => Ok(Operator::Rlc),
            "rr" => Ok(Operator::Rr),
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
    C,
    PC,
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
            "c" => Ok(Register::C),
            "pc" => Ok(Register::PC),
            "dptr" => Ok(Register::DPTR),
            _ => Err(()),
        }
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
        match s.to_lowercase().as_ref() {
            "db" => Ok(Definition::DefineByte),
            "dw" => Ok(Definition::DefineWord),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DirectLocation {
    Port(u8),
    SP,
    DPL,
    DPH,
    PCON,
    TCON,
    TMOD,
    TL0,
    TL1,
    TH0,
    TH1,
    SCON,
    SBUF,
    IEN0,
    IP0,
    IEN1,
    IP1,
    IRCON,
    CCEN,
    CCL1,
    CCH1,
    CCL2,
    CCH2,
    CCL3,
    CCH3,
    T2CON,
    CRCL,
    CRCH,
    TL2,
    TH2,
    PSW,
    ADCON,
    ADDAT,
    DAPR,
    ACC,
    B,
}

impl FromStr for DirectLocation {
    type Err = ();

    fn from_str(s: &str) -> Result<DirectLocation, ()> {
        let re = Regex::new(r"^p(\d)$").unwrap();
        if let Some(caps) = re.captures(&s.to_lowercase()) {
            let reg_num: u8 = caps.at(1).unwrap().parse().unwrap();
            if reg_num < 7 {
                return Ok(DirectLocation::Port(reg_num));
            } else {
                return Err(());
            }
        }

        match s.to_lowercase().as_ref() {
            "sp" => Ok(DirectLocation::SP),
            "dpl" => Ok(DirectLocation::DPL),
            "dph" => Ok(DirectLocation::DPH),
            "pcon" => Ok(DirectLocation::PCON),
            "tcon" => Ok(DirectLocation::TCON),
            "tmod" => Ok(DirectLocation::TMOD),
            "tl0" => Ok(DirectLocation::TL0),
            "tl1" => Ok(DirectLocation::TL1),
            "th0" => Ok(DirectLocation::TH0),
            "th1" => Ok(DirectLocation::TH1),
            "scon" => Ok(DirectLocation::SCON),
            "sbuf" => Ok(DirectLocation::SBUF),
            "ien0" => Ok(DirectLocation::IEN0),
            "ip0" => Ok(DirectLocation::IP0),
            "ien1" => Ok(DirectLocation::IEN1),
            "ip1" => Ok(DirectLocation::IP1),
            "ircon" => Ok(DirectLocation::IRCON),
            "ccen" => Ok(DirectLocation::CCEN),
            "ccl1" => Ok(DirectLocation::CCL1),
            "cch1" => Ok(DirectLocation::CCH1),
            "ccl2" => Ok(DirectLocation::CCL2),
            "cch2" => Ok(DirectLocation::CCH2),
            "ccl3" => Ok(DirectLocation::CCL3),
            "cch3" => Ok(DirectLocation::CCH3),
            "t2con" => Ok(DirectLocation::T2CON),
            "crcl" => Ok(DirectLocation::CRCL),
            "crch" => Ok(DirectLocation::CRCH),
            "tl2" => Ok(DirectLocation::TL2),
            "th2" => Ok(DirectLocation::TH2),
            "psw" => Ok(DirectLocation::PSW),
            "adcon" => Ok(DirectLocation::ADCON),
            "addat" => Ok(DirectLocation::ADDAT),
            "dapr" => Ok(DirectLocation::DAPR),
            "acc" => Ok(DirectLocation::ACC),
            "b" => Ok(DirectLocation::B),
            _ => Err(()),
        }
    }
}

impl DirectLocation {
    pub fn get_addr(&self) -> u8 {
        match *self {
            DirectLocation::Port(p) => {
                match p {
                    0 => 0x80,
                    1 => 0x90,
                    2 => 0xA0,
                    3 => 0xB0,
                    4 => 0xE8,
                    5 => 0xF8,
                    6 => 0xDB,
                    _ => unreachable!(),
                }
            }
            DirectLocation::SP => 0x81,
            DirectLocation::DPL => 0x82,
            DirectLocation::DPH => 0x83,
            DirectLocation::PCON => 0x87,
            DirectLocation::TCON => 0x88,
            DirectLocation::TMOD => 0x89,
            DirectLocation::TL0 => 0x8A,
            DirectLocation::TL1 => 0x8B,
            DirectLocation::TH0 => 0x8C,
            DirectLocation::TH1 => 0x8D,
            DirectLocation::SCON => 0x98,
            DirectLocation::SBUF => 0x99,
            DirectLocation::IEN0 => 0xA8,
            DirectLocation::IP0 => 0xA9,
            DirectLocation::IEN1 => 0xB8,
            DirectLocation::IP1 => 0xB9,
            DirectLocation::IRCON => 0xC0,
            DirectLocation::CCEN => 0xC1,
            DirectLocation::CCL1 => 0xC2,
            DirectLocation::CCH1 => 0xC3,
            DirectLocation::CCL2 => 0xC4,
            DirectLocation::CCH2 => 0xC5,
            DirectLocation::CCL3 => 0xC6,
            DirectLocation::CCH3 => 0xC7,
            DirectLocation::T2CON => 0xC8,
            DirectLocation::CRCL => 0xCA,
            DirectLocation::CRCH => 0xCB,
            DirectLocation::TL2 => 0xCC,
            DirectLocation::TH2 => 0xCD,
            DirectLocation::PSW => 0xD0,
            DirectLocation::ADCON => 0xD8,
            DirectLocation::ADDAT => 0xD9,
            DirectLocation::DAPR => 0xDA,
            DirectLocation::ACC => 0xE0,
            DirectLocation::B => 0xF0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
    Org,
    Equ,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Keyword, ()> {
        match s.to_lowercase().as_ref() {
            "org" => Ok(Keyword::Org),
            "equ" => Ok(Keyword::Equ),
            _ => Err(()),
        }
    }
}
