#[allow(unused_imports)]
use core::arch::asm;
use hippomenes_derive::CSRAccess;
// Field definitions
#[derive(CSRAccess)]
#[width = 1]
#[offset = 0]
#[address = 0xB20]
pub enum Pending {}
#[derive(CSRAccess)]
#[width = 1]
#[offset = 1]
#[address = 0xB20]
pub enum Enabled {}
#[derive(CSRAccess)]
#[width = 3]
#[offset = 2]
#[address = 0xB20]
pub enum Priority {}

// marker
pub struct Interrupt0;
pub struct Bits;

impl Bits {
    read_csr_as_usize!(0xB20);
    write_csr_as!(0xB20);
    set!(0xB20);
    clear!(0xB20);
}
#[allow(non_snake_case)]
pub mod Timestamp {
    pub struct Bits;
    impl Bits {
        read_csr_as_usize!(0xB40);
        write_csr_as!(0xB40);
        set!(0xB40);
        clear!(0xB40);
    }
}
