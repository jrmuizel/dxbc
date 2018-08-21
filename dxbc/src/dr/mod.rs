#[repr(u32)]
#[derive(Debug)]
pub enum ResourceReturnType {
    NotApplicable = 0,
    UNorm = 1,
    SNorm = 2,
    SInt = 3,
    UInt = 4,
    Float = 5,
    Mixed = 6,
    Double = 7,
    Continued = 8
}

pub mod rdef;
pub mod isgn;
pub mod shex;

pub use self::rdef::*;
pub use self::isgn::*;
pub use self::shex::*;

#[repr(C)]
#[derive(Debug)]
pub struct DxbcHeader {
    pub magic: [u8; 4],
    pub checksum: [u8; 16],
    _unknown: u8,
    pub size: u32,
    pub chunk_count: u32,
}
