#![no_std]

pub mod cppdef;
pub use cppdef::{
	RawInterface, ReturnCode,
	CreateInterfaceFn,
};

mod dll_interface_factory;
pub use dll_interface_factory::*;
mod interface;
pub use interface::*;
mod vtable_interface;
pub use vtable_interface::*;
