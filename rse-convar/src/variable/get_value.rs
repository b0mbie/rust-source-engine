use ::core::ffi::{
	CStr, c_float, c_int,
};

use super::ConVarExt;

/// Trait for types that can be returned from a [`ConVarExt`].
pub trait GetValue<'a> {
	/// Returns the value that is stored inside of the `con_var`.
	fn get_value(con_var: &'a ConVarExt) -> Self;
}

impl<'a> GetValue<'a> for &'a CStr {
	fn get_value(con_var: &'a ConVarExt) -> Self {
		con_var.c_str()
	}
}

impl<'a> GetValue<'a> for &'a [u8] {
	fn get_value(con_var: &'a ConVarExt) -> Self {
		con_var.c_str().to_bytes()
	}
}

impl<'a> GetValue<'a> for &'a c_float {
	fn get_value(con_var: &'a ConVarExt) -> Self {
		&con_var.as_inner().value_float
	}
}

impl<'a> GetValue<'a> for &'a c_int {
	fn get_value(con_var: &'a ConVarExt) -> Self {
		&con_var.as_inner().value_int
	}
}

macro_rules! impl_int {
	($($target:ident)+) => {
		$(
			impl<'a> GetValue<'a> for $target {
				fn get_value(con_var: &'a ConVarExt) -> Self {
					con_var.int() as _
				}
			}
		)+
	};
}
impl_int!(u8 i8);
impl_int!(u16 i16);
impl_int!(u32 i32);
impl_int!(u64 i64);
impl_int!(usize isize);

macro_rules! impl_float {
	($($target:ident)+) => {
		$(
			impl<'a> GetValue<'a> for $target {
				fn get_value(con_var: &'a ConVarExt) -> Self {
					con_var.float() as _
				}
			}
		)+
	};
}
impl_float!(f32 f64);
