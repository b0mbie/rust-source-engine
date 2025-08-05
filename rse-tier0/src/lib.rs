#![no_std]

pub use ::rse_game::cppdef::Color;

pub mod fmt_adapters;

mod c_format;
pub use c_format::*;
mod errors;
pub use errors::*;
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
		dev_msg, dev_warn, con_msg, con_warn, con_color_msg,
	};

	pub use crate::{
		fmt_adapters::ConstColor,
		Tier0Spew, Tier0SpewGroups,
		Tier0Errors,
		Logger, LevelLogger, ColorLogger, ColorLevelLogger,
		msg, warn, log, color_msg,
	};
}
