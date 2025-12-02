#![no_std]

pub mod cppdef;

pub mod console_base;
pub mod command;
pub mod variable;
pub mod cvar;

mod wrappers;
pub use wrappers::*;

pub mod prelude {
	pub use crate::{
		command::{
			DispatchCommand, Suggestions,
		},
		console_base::CvarDllIdentifier,
	};

	pub use crate::cvar_value;
}
