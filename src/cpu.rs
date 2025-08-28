use crate::mem::Mem;
use crate::opcode::Opcode;

#[derive(Debug)]
pub struct CPU {
    pc: u16, // Program counter
    sp: u16, // Stack pointer

    reg_a: u8, // Accumulator
    reg_x: u8, // Index register
    reg_y: u8, // Index register

    // status flags
    carry: bool,
    zero: bool,
    interrupt_disable: bool,
    decimal_mode: bool,
    break_command: bool,
    overflow: bool,
    negative: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0xFFFC,
            sp: 0x0100,
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            carry: false,
            zero: false,
            interrupt_disable: false,
            decimal_mode: false,
            break_command: false,
            overflow: false,
            negative: false,
        }
    }

    fn fetch(&mut self, mem: &Mem, cycles: &mut u32) -> u8 {
        let data = mem.0[self.pc as usize];
        self.pc += 1;
        *cycles -= 1;
        data
    }

    pub fn execute(&mut self, mem: &Mem, mut cycles: u32) {
        while cycles > 0 {
            let inst = self.fetch(mem, &mut cycles);

            match Opcode::from(inst) {
                Opcode::LDA_IM => {
                    self.reg_a = self.fetch(mem, &mut cycles);
                    self.zero = self.reg_a == 0;
                    self.negative = self.reg_a & 0b1000_0000 != 0;
                }
                Opcode::INVALID(op) => {
                    println!("Instruction not handled: {:#X}", op);
                }
            }
        }
    }
}
