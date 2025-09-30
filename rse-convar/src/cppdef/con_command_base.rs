use ::core::ffi::{
	c_char, c_int,
};
use ::rse_cpp::{
	VtObjectPtr, WithVTable,
	vtable,
};

use super::CvarDllIdentifier;

pub type ConCommandBase = WithVTable<ConCommandBaseVt, ConCommandBaseExt>;

#[derive(Debug)]
#[repr(C)]
pub struct ConCommandBaseExt {
	pub next: Option<VtObjectPtr<ConCommandBaseVt>>,
	pub registered: bool,
	pub name: *const c_char,
	pub help_string: *const c_char,
	pub flags: CvarFlags,
}

pub type CvarFlags = c_int;

vtable! {
	pub ConCommandBaseVt for VtObjectPtr<ConCommandBaseVt> {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
		pub fn is_command() -> bool;
		pub fn is_flag_set(flag: c_int) -> bool;
		pub fn add_flags(flags: c_int);
		pub fn get_name() -> *const c_char;
		pub fn get_help_text() -> *const c_char;
		pub fn is_registered() -> bool;
		pub fn get_dll_identifier() -> CvarDllIdentifier;
		pub fn create_base(name: *const c_char, help_string: *const c_char, flags: c_int);
		pub fn init();
	}
}
