pub struct Mem(pub [u8; 64 * 1024]);

impl Mem {
    pub fn new() -> Self {
        Self([0; 64 * 1024])
    }
}
