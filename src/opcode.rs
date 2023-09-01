
#[repr(u8)]
pub enum Opcode {
    LDA_IM = 0xA9,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0xA9 => Opcode::LDA_IM,
            _ => unimplemented!("Opcode not implemented: {:#X}", v),
        }
    }
}