#![no_std]

pub mod cppdef;

mod vector;
pub use vector::*;

#[cfg(feature = "tier0")]
mod string;
#[cfg(feature = "tier0")]
pub use string::CString;

pub mod memory;
pub use memory::Memory;
