#[derive(Debug, PartialEq)]
pub enum Opcode {
    ///Moves RS contents to RD, RS being the source register and RD being the destination register
    Move, //rd <- rs
    ///Move if flag is set (bit 7 of the flag)
    MoveT, //rd <- rs
    ///Move if flag is cleared (bit 7 of the flag)
    MoveF, //rd <- rs

    Increment,
    Add,
    AddImmediate,
    Subtract,
    Decrement,
    AND,
    OR,
    XOR,
    NOT,
    ShiftRight,
    ShiftLeft,
    Load,
    LoadImmediate,
    Store,



    UnconditionalJump,  //pc +- rs      //jmp
    SubroutineJump,     //pc +- rs      //jsr
    BranchIfTrue,       //pc +- rs      //bt
    BranchIfFalse,      //pc +- rs      //bf
    BranchOnZero,       //pc +- rs      //brz
    BranchNotZero,      //pc +- rs      //bnz
    BranchOnNegative,   //pc +- rs      //brn

    Illegal,
}
impl Opcode {
    pub const fn value(&self) -> u16 {
        match *self {
            //16bit ops
            Opcode::Move => 0x0001,
            Opcode::MoveT => 0x0002,
            Opcode::MoveF => 0x0004,

            Opcode::Increment => 0x0001,
            Opcode::Decrement => 0x0006,
            Opcode::Add => 0x02,
            Opcode::AddImmediate => 0x0042,
            Opcode::Subtract => 0x0005,
            Opcode::AND => 0x0008,
            Opcode::OR => 0x0009,
            Opcode::XOR => 0x000a,
            Opcode::NOT => 0x000b,
            Opcode::ShiftRight => 0x000d,
            Opcode::ShiftLeft => 0x000e,
            Opcode::Load => 0x10,
            Opcode::LoadImmediate => 0x004c,
            Opcode::Store => 0x20,
            Opcode::BranchOnZero => 0x0060,
            Opcode::BranchOnNegative => 0x0061,
            Opcode::UnconditionalJump => 0x0070,
            Opcode::Illegal => 0xffff,
            _ => 0x0000
        }
    }
    pub const fn mnemonic(&self) -> &str {
        match *self {
            Opcode::Move => "MOV",
            Opcode::Increment => "INC",
            Opcode::Decrement => "DEC",
            Opcode::Add => "ADD",
            Opcode::AddImmediate => "ADI",
            Opcode::Subtract => "SUB",
            Opcode::AND => "AND",
            Opcode::OR => "OR",
            Opcode::XOR => "XOR",
            Opcode::NOT => "NOT",
            Opcode::ShiftRight => "SHR",
            Opcode::ShiftLeft => "SHL",
            Opcode::Load => "LD",
            Opcode::LoadImmediate => "LDI",
            Opcode::Store => "ST",
            Opcode::BranchOnZero => "BRZ",
            Opcode::BranchOnNegative => "BRN",
            Opcode::UnconditionalJump => "JMP",
            Opcode::Illegal => "IGL",
            _ => "",
        }
    }
}
impl From<u16> for Opcode {
    fn from(binary: u16) -> Self {
        return match binary {
            0x0000 => Opcode::Move,
            0x0001 => Opcode::Increment,
            0x0006 => Opcode::Decrement,
            0x0002 => Opcode::Add,
            0x0042 => Opcode::AddImmediate,
            0x0005 => Opcode::Subtract,
            0x0008 => Opcode::AND,
            0x0009 => Opcode::OR,
            0x000a => Opcode::XOR,
            0x000b => Opcode::NOT,
            0x000d => Opcode::ShiftRight,
            0x000e => Opcode::ShiftLeft,
            0x0010 => Opcode::Load,
            0x004c => Opcode::LoadImmediate,
            0x0020 => Opcode::Store,
            0x0060 => Opcode::BranchOnZero,
            0x0061 => Opcode::BranchOnNegative,
            0x0070 => Opcode::UnconditionalJump,
            0xffff => Opcode::Illegal,
            _ => Opcode::Illegal,
        }
    }
}

//vm special features
pub enum MacroOp {

}

pub struct Instruction {
    opcode: Opcode,
}
impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode,
        }
    }
}

#[cfg(test)]
mod tests {
    use Opcode::Increment;
    use super::*;

    #[test]
    fn test_opcode() {
        let opcode = Increment;
        assert_eq!(opcode.mnemonic(), "INC");
        assert_eq!(opcode.value(), 0x01);
    }

    #[test]
    fn test_instruction() {
        let instruction = Instruction::new(Opcode::Increment);
        assert_eq!(instruction.opcode, Opcode::Increment);
    }
}