#![no_std]

#[cfg(feature = "macros")]
#[doc(hidden)]
pub use ::concat_idents;

pub mod cppdef;

mod util;

pub mod console_base;
pub mod command;
pub mod variable;

mod wrappers;
pub use wrappers::*;

pub mod prelude {
	pub use crate::{
		command::{
			DispatchCommand, Suggestions,
		},
		console_base::CvarDllIdentifier,
	};

	#[cfg(feature = "macros")]
	pub use crate::cvar_value;
}
