use ::core::ffi::{
	c_char, c_void,
};
use ::rse_cpp::class;

use super::CreateInterfaceFn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum InitStatus {
	Failed = 0,
	Ok,
}

class! {
	pub IAppSystem {
		pub virtual fn connect(factory: CreateInterfaceFn) -> bool;
		pub virtual fn disconnect();

		pub virtual fn query_interface(name: *const c_char) -> *mut c_void;

		pub virtual fn init() -> InitStatus;
		pub virtual fn shutdown();
	}
}
