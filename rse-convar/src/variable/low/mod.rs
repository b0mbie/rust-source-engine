use ::core::{
	ffi::{
		CStr, c_float, c_int,
	},
	pin::Pin,
};

use crate::console_base::RawConsoleBase;

mod object;
pub use object::*;
mod object_static;
pub use object_static::*;

pub trait RawVariable<'str>
where
	Self: Sized,
	Self: RawConsoleBase<ConVarObject<'str, Self>>,
{
	fn set_c_str(object: Pin<&mut ConVarObject<'str, Self>>, value: Option<&CStr>);
	fn set_float_forced(object: Pin<&mut ConVarObject<'str, Self>>, value: c_float);
	fn set_int(object: Pin<&mut ConVarObject<'str, Self>>, value: c_int);

	fn set_float(object: Pin<&mut ConVarObject<'str, Self>>, value: c_float) {
		Self::set_float_forced(object, value)
	}

	fn change_string_value(object: Pin<&mut ConVarObject<'str, Self>>, new_value: Option<&CStr>, old_value: c_float) {
		let _ = object;
		let _ = new_value;
		let _ = old_value;
	}
	fn clamp_value(object: Pin<&mut ConVarObject<'str, Self>>, value: &mut c_float) -> bool {
		let _ = object;
		let _ = value;
		false
	}
}
