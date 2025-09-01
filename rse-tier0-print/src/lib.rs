#![no_std]

pub use ::rse_tier0 as tier0;

pub use tier0::Color;

mod color_provider;
pub use color_provider::*;
mod colored;
pub use colored::*;
mod newlined;
pub use newlined::*;
mod plain;
pub use plain::*;
mod printing;
pub use printing::*;
mod then;
pub use then::*;

pub mod prelude {
	pub use crate::{
		ComposeThen,
		ComposeColored, Color, ConstColor,
		ComposeNewlined,
		IntoPlain,
		Printer,
	};
}
