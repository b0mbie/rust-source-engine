#![no_std]

pub use ::rse_game::cppdef::Color;

pub mod fmt_adapters;

mod c_format;
pub use c_format::*;
mod level;
pub use level::*;
mod traits;
pub use traits::*;

#[cfg(feature = "link-dll")]
pub mod linked;

pub mod prelude {
	#[cfg(feature = "link-dll")]
	pub use crate::linked::{
		LinkedTier0,
		con, dev, dev_con,
	};

	pub use crate::{
		Tier0, Tier0Spew,
		Logger, LevelLogger, ColorLogger, ColorLevelLogger,
		msg, warn, log,
	};
}
