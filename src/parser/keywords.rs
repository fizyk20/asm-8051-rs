#[derive(Clone, Debug)]
pub enum Operator {
    Mov,
    Jmp,
    Call,
    Ret
    // TODO
}

#[derive(Clone, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    SP,
    DPTR
    // TODO
}

