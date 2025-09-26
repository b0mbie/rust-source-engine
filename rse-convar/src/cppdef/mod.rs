use ::core::{
	ffi::{
		c_char, c_float, c_int,
	},
	ptr::NonNull,
};
use ::rse_cpp::{
	VtObjectPtr, vtable,
};

/// Type for a DLL identifier that's used to mark ConVars and ConCommands.
pub type CvarDllIdentifier = c_int;

mod con_command;
pub use con_command::*;
mod command;
pub use command::*;

pub mod flags;

#[derive(Debug)]
#[repr(C)]
pub struct ConCommandBase {
	pub vtable: NonNull<ConCommandBaseVt>,
	pub ext: ConCommandBaseExt,
}

#[derive(Debug)]
#[repr(C)]
pub struct ConCommandBaseExt {
	pub next: Option<VtObjectPtr<ConCommandBaseVt>>,
	pub registered: bool,
	pub name: *const c_char,
	pub help_string: *const c_char,
	pub flags: c_int,
}

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

// TODO: Add a `typeinfo` field for `ConVar`.
// This is because of a `dynamic_cast` to `ConVar_ServerBounded` in `ConVar_PrintDescription`.
// It can just be null,
// in which case the cast will just always fail due to a simple pointer comparison failure in Itanium.
#[repr(C)]
pub struct ConVarVt {
	pub con_command_base: ConCommandBaseVt,
	pub iface: ConVarIfaceVt,
	pub convar: ConVarVtBase,
}

vtable! {
	pub ConVarIfaceVt {
		pub fn set_value_string(value: *const c_char);
		pub fn set_value_float(value: *const c_float);
		pub fn set_value_int(value: *const c_int);
		pub fn get_name() -> *const c_char;
		pub fn is_flag_set(flag: c_int) -> bool;
	}
}

// TODO: `ConVar`.
vtable! {
	pub ConVarVtBase for VtObjectPtr<ConVarVt> {}
}
