#![no_std]

pub use ::rse_convar as convar;

mod c_buffer;
use c_buffer::*;
mod variable;
pub use variable::*;

pub mod prelude {
	pub use ::rse_convar::prelude::*;
	pub use crate::StdVariable;
}
