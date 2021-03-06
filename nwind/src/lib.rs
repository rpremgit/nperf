extern crate byteorder;
extern crate gimli;
extern crate goblin;
#[cfg(feature = "logging")]
#[macro_use]
extern crate log;
extern crate memmap;
extern crate scroll;
extern crate lru;
extern crate speedy;
#[macro_use]
extern crate speedy_derive;

#[cfg(test)]
extern crate env_logger;

#[cfg(not(feature = "logging"))]
macro_rules! trace { ($($token:tt)*) => {} }

#[cfg(not(feature = "logging"))]
macro_rules! debug { ($($token:tt)*) => {} }

#[cfg(not(feature = "logging"))]
macro_rules! warn { ($($token:tt)*) => {} }

#[cfg(not(feature = "logging"))]
macro_rules! info { ($($token:tt)*) => {} }

#[cfg(not(feature = "logging"))]
macro_rules! error { ($($token:tt)*) => {} }

#[macro_use]
mod elf;

mod address_space;
pub mod arch;
mod arm_extab;
mod binary;
mod dwarf;
mod dwarf_regs;
mod frame_descriptions;
pub mod maps;
mod range_map;
mod symbols;
mod types;
pub mod utils;
mod unwind_context;
#[cfg(feature = "local-unwinding")]
mod local_unwinding;

pub use address_space::{
    BinarySource,
    BufferReader,
    Primitive,
    IAddressSpace,
    AddressSpace
};
pub use dwarf_regs::DwarfRegs;
pub use range_map::RangeMap;
pub use binary::{BinaryData, SymbolTable};
pub use symbols::Symbols;
pub use types::{
    BinaryId,
    Bitness,
    UserFrame
};

#[cfg(feature = "local-unwinding")]
pub use local_unwinding::{LocalAddressSpace, UnwindControl};
