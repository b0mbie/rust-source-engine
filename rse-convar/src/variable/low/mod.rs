use ::core::ffi::{
	CStr, c_float, c_int,
};

use crate::console_base::RawConsoleBase;

mod object;
pub use object::*;
mod params;
pub use params::*;
mod wrapper;
pub use wrapper::*;

pub trait RawVariable<'a>
where
	Self: Sized,
	Self: RawConsoleBase<ConVarObject<'a, Self>>,
{
	fn set_c_str(object: &mut ConVarObject<'a, Self>, value: Option<&CStr>);
	fn set_float_forced(object: &mut ConVarObject<'a, Self>, value: c_float);
	fn set_int(object: &mut ConVarObject<'a, Self>, value: c_int);

	fn set_float(object: &mut ConVarObject<'a, Self>, value: c_float) {
		Self::set_float_forced(object, value)
	}

	fn change_string_value(object: &mut ConVarObject<'a, Self>, new_value: Option<&CStr>, old_value: c_float);
}
