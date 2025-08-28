/*
   Followed a YouTube tutorial by Dave Poo titled "Emulating a CPU in C++ (6502)"
   https://youtu.be/qJgsuQoy9bc

   I tried to make a literal translation of the code from C++ to Rust.
*/

mod cpu;
mod mem;
mod opcode;

use cpu::CPU;
use cpu::RESET_VECTOR;
use mem::Mem;
use opcode::Opcode;

fn main() {
    let mut mem = Mem::new();
    let mut cpu = CPU::new();

    let program = [Opcode::LDA_IM.into(), 0x42];
    for (offset, &value) in program.iter().enumerate() {
        mem.write_offset(RESET_VECTOR, offset as u16, value);
    }

    cpu.execute(&mem, 2);

    println!("{:?}", cpu);
}
