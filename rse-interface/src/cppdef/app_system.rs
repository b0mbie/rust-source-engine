use ::core::ffi::{
	c_char, c_void,
};
use ::rse_cpp::vtable;

use super::CreateInterfaceFn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum InitStatus {
	Failed = 0,
	Ok,
}

vtable! {
	pub AppSystemVt {
		pub fn connect(factory: CreateInterfaceFn) -> bool;
		pub fn disconnect();

		pub fn query_interface(name: *const c_char) -> *mut c_void;

		pub fn init() -> InitStatus;
		pub fn shutdown();
	}
}
