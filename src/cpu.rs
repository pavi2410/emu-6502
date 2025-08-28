use crate::mem::Mem;
use crate::opcode::Opcode;

// 6502 Interrupt Vectors
pub const RESET_VECTOR: u16 = 0xFFFC;
pub const IRQ_VECTOR: u16 = 0xFFFE;
pub const NMI_VECTOR: u16 = 0xFFFA;

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

    // Internal cycle counter
    cycles: u32,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: RESET_VECTOR,
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
            cycles: 0,
        }
    }

    /// Reset CPU to initial state
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    fn fetch(&mut self, mem: &Mem) -> u8 {
        let data = mem.0[self.pc as usize];
        self.pc += 1;
        self.cycles += 1;
        data
    }

    /// Execute a single instruction
    pub fn step(&mut self, mem: &Mem) {
        let inst = self.fetch(mem);

        match Opcode::from(inst) {
            Opcode::LDA_IM => {
                self.reg_a = self.fetch(mem);
                self.zero = self.reg_a == 0;
                self.negative = self.reg_a & 0b1000_0000 != 0;
            }
            Opcode::INVALID(op) => {
                println!("Instruction not handled: {:#X}", op);
            }
        }
    }

    /// Execute instructions for the specified number of cycles
    pub fn execute(&mut self, mem: &Mem, target_cycles: u32) {
        let start_cycles = self.cycles;
        while self.cycles - start_cycles < target_cycles {
            self.step(mem);
        }
    }
}
