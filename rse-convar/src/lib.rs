#![no_std]

#[cfg(feature = "macros")]
#[doc(hidden)]
pub use ::concat_idents;

pub mod cppdef;

mod util;

pub mod console_base;
pub mod command;
pub mod variable;

mod invocation;
pub use invocation::*;

pub mod prelude {
	pub use crate::{
		command::{
			DllCommand, Suggestions,
		},
		console_base::CvarDllIdentifier,
		variable::{
			Variable, NewValue, OldValue,
		},
	};

	#[cfg(feature = "macros")]
	pub use crate::cvar_value;
}
