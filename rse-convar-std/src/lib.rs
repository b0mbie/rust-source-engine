#![no_std]

use ::rse_convar::variable::{
	low::StaticConVarObject,
	Variable, params_for,
};

pub use ::rse_convar as convar;

mod c_buffer;
use c_buffer::*;
mod variable;
pub use variable::*;

pub const fn static_con_var<T>(variable: T) -> StaticConVarObject<StdVariable<T>>
where
	T: Variable,
{
	StaticConVarObject::new(StdVariable::new(variable), params_for::<T>())
}

pub mod prelude {
	pub use ::rse_convar::prelude::*;
	pub use crate::{
		StdVariable, static_con_var,
	};
}
