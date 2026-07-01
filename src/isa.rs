#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    // Stack Operations
    Push(i64),
    Pop,
    Dup,
    Swap,

    // Arithmetic Operations
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,

    // Global Memory Operations
    Load(u8),
    Store(u8),

    // Output
    Print,

    // Stop Program
    Halt,
}

impl Op {

    pub fn opcode(&self) -> u8 {
        match self {
            Op::Push(_)  => 0x01,
            Op::Pop      => 0x02,
            Op::Dup      => 0x03,
            Op::Swap     => 0x04,

            Op::Add      => 0x10,
            Op::Sub      => 0x11,
            Op::Mul      => 0x12,
            Op::Div      => 0x13,
            Op::Mod      => 0x14,
            Op::Neg      => 0x15,

            Op::Load(_)  => 0x40,
            Op::Store(_) => 0x41,

            Op::Print    => 0x60,

            Op::Halt     => 0xFF,
        }
    }
}