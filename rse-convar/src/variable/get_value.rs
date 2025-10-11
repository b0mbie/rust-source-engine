use ::core::ffi::{
	CStr, c_float, c_int,
};

use super::ConVarExt;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ValueView<'a>(&'a ConVarExt);
impl<'a> ValueView<'a> {
	pub const fn new(ext: &'a ConVarExt) -> Self {
		Self(ext)
	}

	pub const fn c_str(&self) -> &'a CStr {
		self.0.c_str()
	}

	pub const fn float(&self) -> c_float {
		self.0.float()
	}

	pub const fn int(&self) -> c_int {
		self.0.int()
	}
}

/// Trait for types that can be returned from a [`ConVarExt`].
pub trait GetValue<'a> {
	/// Returns the value that is stored inside of the `con_var`.
	fn get_value(con_var: ValueView<'a>) -> Self;
}

impl<'a> GetValue<'a> for &'a CStr {
	fn get_value(con_var: ValueView<'a>) -> Self {
		con_var.c_str()
	}
}

impl<'a> GetValue<'a> for &'a [u8] {
	fn get_value(con_var: ValueView<'a>) -> Self {
		con_var.c_str().to_bytes()
	}
}

macro_rules! impl_int {
	($($target:ident)+) => {
		$(
			impl<'a> GetValue<'a> for $target {
				fn get_value(con_var: ValueView<'a>) -> Self {
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
				fn get_value(con_var: ValueView<'a>) -> Self {
					con_var.float() as _
				}
			}
		)+
	};
}
impl_float!(f32 f64);
