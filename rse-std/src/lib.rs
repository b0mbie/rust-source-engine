pub use ::rse_convar as convar;

pub(crate) mod atomic;
pub(crate) mod c_buffer;

pub mod cvar;

pub mod command;
pub mod plugin;
pub mod variable;

pub mod prelude {
	pub use ::rse_convar::prelude::*;
	pub use crate::variable::{
		ConVar, con_var,
	};
}
