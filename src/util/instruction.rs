#[derive(Debug, PartialEq)]
pub enum Opcode {
    ///Moves RS contents to RD, RS being the source register and RD being the destination register
    Move,               //rd <- rs      //mov
    ///Move if flag is set (bit 7 of the flag)
    MoveT,              //rd <- rs      //movt
    ///Move if flag is cleared (bit 7 of the flag)
    MoveF,              //rd <- rs      //movf

    Copy,

    Increment,                          //inc
    Add,
    AddImmediate,
    Subtract,
    Decrement,                          //dec
    AND,
    OR,
    XOR,
    NOT,
    ShiftRight,
    ShiftLeft,
    Load,
    LoadImmediate,
    Store,

    ///Program counter offsets, scaled by 2 for alignment, all apply the prefix register when active
    UnconditionalJump,  //pc +- rs      //jmp
    SubroutineJump,     //pc +- rs      //jsr
    BranchIfTrue,       //pc +- rs      //bt
    BranchIfFalse,      //pc +- rs      //bf
    BranchOnZero,       //pc +- rs      //brz
    BranchNotZero,      //pc +- rs      //bnz
    BranchOnNegative,   //pc +- rs      //brn

    //upper bound (0xffff and down) opcodes meant for later vm special controls
    Illegal,
    Noop,
}
impl Opcode {
    pub const fn value(&self) -> u16 {
        match *self {
            //16bit ops
            Opcode::Move => 0x0001,
            Opcode::MoveT => 0x0002,
            Opcode::MoveF => 0x0004,

            _ => 0x0000
        }
    }
    pub const fn mnemonic(&self) -> &str {
        match *self {
            Opcode::Move => "MOV",

            _ => "",
        }
    }
}
impl From<u16> for Opcode {
    fn from(binary: u16) -> Self {
        return match binary {
            0x0000 => Opcode::Move,

            0xffff => Opcode::Illegal,
            _ => Opcode::Illegal,
        }
    }
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
    fn test_instruction() {
        let instruction = Instruction::new(Opcode::Increment);
        assert_eq!(instruction.opcode, Opcode::Increment);
    }
}
