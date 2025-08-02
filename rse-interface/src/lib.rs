#![no_std]

pub mod cppdef;

mod dll_interface_factory;
pub use dll_interface_factory::*;
mod interface;
pub use interface::*;
