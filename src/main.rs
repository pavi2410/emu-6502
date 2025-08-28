/*
   Followed a YouTube tutorial by Dave Poo titled "Emulating a CPU in C++ (6502)"
   https://youtu.be/qJgsuQoy9bc

   I tried to make a literal translation of the code from C++ to Rust.
*/

mod cpu;
mod mem;
mod opcode;

use cpu::CPU;
use mem::Mem;
use opcode::Opcode;

fn main() {
    let mut mem = Mem::new();
    let mut cpu = CPU::new();

    mem.0[0xFFFC] = Opcode::LDA_IM.into();
    mem.0[0xFFFD] = 0x42;

    cpu.execute(&mem, 2);

    println!("{:?}", cpu);
}
