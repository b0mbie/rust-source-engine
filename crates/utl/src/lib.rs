#![no_std]

extern crate alloc;

pub mod cppdef;

pub mod vector;
pub use vector::Vector;

#[cfg(feature = "tier0")]
mod string;
#[cfg(feature = "tier0")]
pub use string::CString;

pub mod memory;
pub use memory::Memory;
