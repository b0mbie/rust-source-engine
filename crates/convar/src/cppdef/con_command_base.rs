use ::core::ffi::{
	c_char, c_int,
};
use ::rse_cpp::class;

use super::CvarDllIdentifier;

pub type RawCvarFlags = c_int;

class! {
	pub ConCommandBase: pub virtual Registrable {
		pub virtual fn is_flag_set(flag: c_int) -> bool;
		pub virtual fn add_flags(flags: c_int);
		pub virtual fn get_name() -> *const c_char;
		pub virtual fn get_help_text() -> *const c_char;
		pub virtual fn is_registered() -> bool;
		pub virtual fn get_dll_identifier() -> CvarDllIdentifier;
		pub virtual fn create_base(name: *const c_char, help_string: *const c_char, flags: c_int);
		pub virtual fn init();
	}
}

class! {
	/// Part of [`ConCommandBase`] that is compatible with both `ConVar` and `ConCommand`.
	// For whatever reason, the VTable for `ConVar` on `windows` doesn't include some functions from `ConCommandBase`.
	// So, we can't use that class!
	pub Registrable {
		pub virtual ~destructors();
		pub virtual fn is_command() -> bool;

		pub next: *mut Registrable;
		pub registered: bool;
		pub name: *const c_char;
		pub help_string: *const c_char;
		pub flags: RawCvarFlags;
	}
}
