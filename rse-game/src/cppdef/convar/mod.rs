use ::core::{
	ffi::{
		CStr, c_char, c_float, c_int,
	},
	fmt,
	marker::PhantomPinned,
	mem::size_of,
	slice::from_raw_parts as slice_from_raw_parts,
};
use ::rse_cpp::{
	VtObject, vtable,
};

use super::cvar::CvarDllIdentifier;

mod con_command;
pub use con_command::*;

pub mod flags;

const COMMAND_MAX_ARGC: usize = 64;
const COMMAND_MAX_LENGTH: usize = 512;

#[repr(C)]
pub struct Command {
	argc: c_int,
	argv0_size: c_int,
	// INVARIANT: The inner buffer contains a valid C string.
	arg_string_buffer: [c_char; COMMAND_MAX_LENGTH],
	// INVARIANT: The inner buffer contains a valid C string.
	argv_buffer: [c_char; COMMAND_MAX_LENGTH],
	argv: [*const c_char; COMMAND_MAX_ARGC],
	// Pointers in `argv` can be dependent on the location of `argv_buffer`, which is the case in tier1 code.
	_argv_pin: PhantomPinned,
}

impl Command {
	pub const MAX_COMMAND_LENGTH: usize = COMMAND_MAX_LENGTH - 1;
	
	pub const fn arg_count(&self) -> usize {
		self.argc as _
	}

	pub const fn args(&self) -> &[Arg] {
		unsafe {
			::core::slice::from_raw_parts(self.argv.as_ptr() as *const Arg, self.argc as _)
		}
	}

	pub const fn args_mut(&mut self) -> &mut [Arg] {
		unsafe {
			::core::slice::from_raw_parts_mut(self.argv.as_mut_ptr() as *mut Arg, self.argc as _)
		}
	}

	pub fn arg_string(&self) -> Option<&CStr> {
		if self.argc != 0 {
			let c_str_bytes = unsafe {
				slice_from_raw_parts(
					self.arg_string_buffer.as_ptr() as *const u8,
					self.arg_string_buffer.len() * size_of::<c_char>() / size_of::<u8>(),
				)
			};
			// SAFETY: `arg_string_buffer` contains a valid C string.
			let c_str = unsafe { CStr::from_bytes_until_nul(c_str_bytes).unwrap_unchecked() };
			Some(c_str)
		} else {
			None
		}
	}
}

#[repr(transparent)]
pub struct Arg {
	ptr: *const c_char,
}

impl Arg {
	pub const fn as_c_str(&self) -> &CStr {
		// SAFETY: The inner pointer is always a valid C string.
		unsafe { CStr::from_ptr(self.ptr) }
	}
}

impl fmt::Debug for Arg {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_c_str().fmt(f)
	}
}

#[derive(Debug)]
#[repr(C)]
pub struct ConCommandBase {
	pub vtable: *mut ConCommandBaseVt,
	pub ext: ConCommandBaseExt,
}

#[derive(Debug)]
#[repr(C)]
pub struct ConCommandBaseExt {
	pub next: Option<VtObject<ConCommandBaseVt>>,
	pub registered: bool,
	pub name: *const c_char,
	pub help_string: *const c_char,
	pub flags: c_int,
}

vtable! {
	pub ConCommandBaseVt for VtObject<ConCommandBaseVt> {
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
	pub ConVarVtBase for VtObject<ConVarVt> {}
}
