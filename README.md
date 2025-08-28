# emu-6502

Followed a YouTube tutorial by Dave Poo titled ["Emulating a CPU in C++ (6502)"](https://youtu.be/qJgsuQoy9bc)

I tried to make a literal translation of the code from C++ to Rust.

## About the 6502

The MOS Technology 6502 is an 8-bit microprocessor introduced in 1975, predating Intel's 8086 by 3 years. Despite being an 8-bit processor, it features a 16-bit address bus allowing it to address 64KB of memory.

### Key Specifications:
- **8-bit data bus**: Processes 8 bits of data at a time
- **16-bit address bus**: Can address 65,536 memory locations (64KB)
- **8-bit registers**: Accumulator (A), Index registers (X, Y)
- **151 valid opcodes**: 56 instructions with multiple addressing modes

### Historical Impact:
The 6502 was widely used in influential computers including:
- Apple II (1977)
- Commodore 64 (1982)
- Nintendo Entertainment System (1983)
- BBC Micro


<figure>
  <img src="http://www.erich-foltyn.eu/Technique/images/6502Chip.gif" alt="6502 Pinout">
  <figcaption>
    <strong>Figure 1:</strong> MOS Technology 6502 microprocessor pinout diagram showing the 40-pin DIP package. Notable features include the 16 address lines (A0-A15) providing 64KB addressing capability, 8 bidirectional data lines (D0-D7), and control signals for memory and I/O operations. 
    <br><em>Source: <a href="http://www.erich-foltyn.eu/Technique/6502.html">Erich Foltyn's 6502 Page</a></em>
  </figcaption>
</figure>
