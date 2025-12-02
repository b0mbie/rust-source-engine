use ::core::{
	ffi::{
		c_char, c_int,
	},
	marker::PhantomPinned,
};

pub const COMMAND_MAX_ARGC: usize = 64;
pub const COMMAND_MAX_LENGTH: usize = 512;

#[repr(C)]
pub struct Command {
	pub argc: c_int,
	pub argv0_size: c_int,
	// INVARIANT: The inner buffer contains a valid C string.
	pub arg_string_buffer: [c_char; COMMAND_MAX_LENGTH],
	// INVARIANT: The inner buffer contains a valid C string.
	pub argv_buffer: [c_char; COMMAND_MAX_LENGTH],
	pub argv: [*const c_char; COMMAND_MAX_ARGC],
	// Pointers in `argv` can be dependent on the location of `argv_buffer`, which is the case in tier1 code.
	pub argv_pin: PhantomPinned,
}
