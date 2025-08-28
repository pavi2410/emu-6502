pub struct Mem(pub [u8; 64 * 1024]);

impl Mem {
    pub fn new() -> Self {
        Self([0; 64 * 1024])
    }

    /// Read a byte from the specified address
    pub fn read(&self, address: u16) -> u8 {
        self.0[address as usize]
    }

    /// Write a byte to the specified address
    pub fn write(&mut self, address: u16, value: u8) {
        self.0[address as usize] = value;
    }

    /// Read a byte from base address + offset
    pub fn read_offset(&self, base: u16, offset: u16) -> u8 {
        let address = base.wrapping_add(offset);
        self.0[address as usize]
    }

    /// Write a byte to base address + offset
    pub fn write_offset(&mut self, base: u16, offset: u16, value: u8) {
        let address = base.wrapping_add(offset);
        self.0[address as usize] = value;
    }
}
