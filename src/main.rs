mod math;
mod util;
mod net;
mod io;

use crate::util::instruction::*;

const VM_NAME: &str = "HinokiBox";

fn main() {
    static VM_VFS: &str = "/sugidisk.bxfs";
    println!("{}", VM_VFS);
}

trait SetHiLo {
    fn set_lo(&mut self, value: u8);
    fn set_hi(&mut self, value: u8);
}
impl SetHiLo for u16 {
    fn set_lo(&mut self, value: u8) {
        *self &= !0xff;
        *self |= value as u16;
    }
    fn set_hi(&mut self, value: u8) {
        *self &= !0xff00;
        *self |= (value as u16) << 8;
    }
}

pub struct VM {
    registers: [u16; 32],
    flags: [u16; 32],
    pc: usize,
    elf: Vec<u8>,
}
impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            flags: [0; 32],
            pc: 0,
            elf: vec![],
        }
    }
    pub fn run(&mut self) {
        let mut is_done: bool = false;
        while !is_done {
            //is_done = self.execute_instruction();
        }
    }
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.next_16_bits()); //(((self.elf[self.pc] as u16) << 8) | self.elf[self.pc + 1] as u16);

        //increment program counter
        self.pc += 1;
        return opcode;
    }

    //program traversal
    fn next_8_bits(&mut self) -> u8 {
        let byte = self.elf[self.pc];

        //increment pc
        self.pc += 1;
        return byte;
    }
    fn next_16_bits(&mut self) -> u16 {
        let mut word: u16 = 0xffff;
        word.set_hi(self.elf[self.pc]);
        word.set_lo(self.elf[self.pc + 1]);

        //increment pc
        self.pc += 2;
        return word;
    }

    //meat and potatoes, vm functions implemented here
    fn execute_instruction(&mut self) -> bool {
        //if there are no instructions left to execute, terminate
        if self.pc >= self.elf.len() {
            return false; //stop vm, no more instructions to execute
        }
        //doesn't have to be exhaustive with defs in instruction.rs
        match self.decode_opcode() {
            Opcode::Illegal => {
                println!("IGL: Illegal instruction, Terminating VM!");
                return false; //abort vm
            },
            Opcode::Load => {
                let register = self.next_16_bits() as usize;
                let data = self.next_16_bits() as u16;
                self.registers[register] = data;
            },
            //opcode -> operand A -> operand B -> register
            Opcode::AddImmediate => {
                let register1 = self.registers[self.next_16_bits() as usize];
                let register2 = self.registers[self.next_16_bits() as usize];
                self.registers[self.next_16_bits() as usize] = register1 + register2;
            },
            _ => {
                println!("???: Unrecognized instruction, Terminating VM!");
                return false; //abort vm
            }
        }
        return true; //continues if matched to anything but IGL and undefined
    }
}

//hex values map to the values specified in instruction.rs