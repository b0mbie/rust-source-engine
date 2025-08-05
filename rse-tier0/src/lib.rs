#![no_std]

pub use ::rse_game::cppdef::Color;

pub mod fmt_adapters;

mod c_format;
pub use c_format::*;
mod level;
pub use level::*;
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
		dev_msg, dev_warn, con_msg, con_warn,
	};

	pub use crate::{
		Tier0Spew, Tier0SpewGroups,
		Logger, LevelLogger, ColorLogger, ColorLevelLogger,
		msg, warn, log,
	};
}
