use ::core::ffi::{
	c_char, c_int,
};
use ::rse_cpp::{
	ptr_compat::PointerFrom,
	VtObjectPtr, WithVTable,
	vtable,
};

use super::CvarDllIdentifier;

pub type ConCommandBase = WithVTable<ConCommandBaseVt, ConCommandBaseExt>;

#[derive(Debug)]
#[repr(C)]
pub struct ConCommandBaseExt {
	pub next: *mut ConCommandBase,
	pub registered: bool,
	pub name: *const c_char,
	pub help_string: *const c_char,
	pub flags: RawCvarFlags,
}

pub type RawCvarFlags = c_int;

#[repr(C)]
pub struct ConCommandBaseVt {
	pub base: ConCommandBaseVtBase,
	pub ext: ConCommandBaseVtExt,
}
unsafe impl PointerFrom<ConCommandBaseVt> for ConCommandBaseVtBase {}

vtable! {
	/// Part of the VTable for [`ConCommandBase`] that is compatible with both [`ConVar`] and [`ConCommand`].
	pub ConCommandBaseVtBase {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
		pub fn is_command() -> bool;
	}
}

vtable! {
	pub ConCommandBaseVtExt for VtObjectPtr<ConCommandBaseVt> {
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
