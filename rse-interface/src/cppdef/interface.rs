use ::core::{
	ffi::{
		c_char, c_int, c_void,
	},
	ptr::NonNull,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ReturnCode(pub c_int);
impl ReturnCode {
	pub const OK: Self = Self(IfaceStatus::Ok as _);
	pub const FAILED: Self = Self(IfaceStatus::Failed as _);
}

pub type RawInterface = NonNull<c_void>;

pub type CreateInterfaceFn = unsafe extern "C" fn(
	name: *const c_char, out_return_code: *mut ReturnCode,
) -> Option<RawInterface>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum IfaceStatus {
	Ok = 0,
	Failed,
}

::rse_cpp::vtable! {
	pub BaseInterfaceVt {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
	}
}
