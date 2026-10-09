#![no_std]

pub mod cppdef;
pub use cppdef::{
	RawInterface, ReturnCode,
	CreateInterfaceFn,
};

mod dll_interface_factory;
pub use dll_interface_factory::*;
mod error;
pub use error::*;
mod interface;
pub use interface::*;
mod ptr_interface;
pub use ptr_interface::*;
