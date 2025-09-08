#![no_std]

extern crate alloc;

pub use ::rse_game::cppdef::Color;

pub mod fmt_adapters;

mod errors;
pub use errors::*;
mod level;
pub use level::*;
mod mem;
pub use mem::*;
mod mem_alloc;
pub use mem_alloc::*;
mod spew;
pub use spew::*;

#[cfg(feature = "link-dll")]
pub mod linked;

pub mod prelude {
	#[cfg(feature = "link-dll")]
	pub use crate::{
		linked::{
			LinkedTier0,
			con, dev, dev_con,
		},
		dev_msg, dev_warn, con_msg, con_warn, con_color_msg,
	};

	pub use crate::{
		Tier0Spew, Tier0SpewGroups,
		Tier0Errors,
		Color,
		Logger, LevelLogger, ColorLogger, ColorLevelLogger,
		msg, warn, log, color_msg,
		msgln, warnln, logln,
	};
}
