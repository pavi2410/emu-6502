use num_enum::{FromPrimitive, IntoPrimitive};

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Opcode {
    LDA_IM = 0xA9,
    #[num_enum(catch_all)]
    INVALID(u8),
}
