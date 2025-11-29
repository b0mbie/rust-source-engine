use ::rust_alloc::{
	borrow::Cow,
	ffi::CString,
};
use ::core::ffi::CStr;

use super::{
	GenericConVar, CStrLock,
};

/// Trait for types that can be returned from [`GenericConVar<T>`].
pub trait GetValue<'a> {
	/// Returns the value that is stored inside of the `con_var`.
	fn get_value<T>(con_var: &'a GenericConVar<T>) -> Self;
}

macro_rules! impl_from_int {
	($($target:ty)*) => {
		$(
			impl<'a> GetValue<'a> for $target {
				fn get_value<T>(con_var: &'a GenericConVar<T>) -> Self {
					let int = con_var.int();
					let clamped = if size_of_val(&int) > size_of::<$target>() {
						int.clamp(<$target>::MIN as _, <$target>::MAX as _)
					} else if <$target>::MIN == 0 {
						int.max(0)
					} else {
						int
					};
					clamped as _
				}
			}
		)*
	};
}
impl_from_int!(
	u8 i8
	u16 i16
	u32 i32
	u64 i64
	usize isize
);

macro_rules! impl_from_float {
	($($target:ty)*) => {
		$(
			impl<'a> GetValue<'a> for $target {
				fn get_value<T>(con_var: &'a GenericConVar<T>) -> Self {
					con_var.float() as _
				}
			}
		)*
	};
}
impl_from_float!(f32 f64);

impl<'a> GetValue<'a> for CStrLock<'a> {
	fn get_value<T>(con_var: &'a GenericConVar<T>) -> Self {
		con_var.c_str()
	}
}

impl<'a> GetValue<'a> for CString {
	fn get_value<T>(con_var: &'a GenericConVar<T>) -> Self {
		con_var.c_str().get().into()
	}
}

impl<'a> GetValue<'a> for Cow<'static, CStr> {
	fn get_value<T>(con_var: &'a GenericConVar<T>) -> Self {
		Cow::Owned(con_var.c_str().get().into())
	}
}

impl<'a> GetValue<'a> for Box<str> {
	fn get_value<T>(con_var: &'a GenericConVar<T>) -> Self {
		con_var.c_str().to_string_lossy().into()
	}
}

impl<'a> GetValue<'a> for String {
	fn get_value<T>(con_var: &'a GenericConVar<T>) -> Self {
		con_var.c_str().to_string_lossy().into()
	}
}

impl<'a> GetValue<'a> for bool {
	fn get_value<T>(con_var: &'a GenericConVar<T>) -> Self {
		con_var.int() != 0
	}
}
