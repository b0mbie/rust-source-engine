use ::core::{
	ffi::{
		c_char, c_int,
	},
	fmt,
};
use ::rse_cpp::{
	ptr_compat::PointerFrom,
	RefConst, RefMut, VtObjectPtr, vtable,
	test_bits, with_bits,
	WithVTable,
};
use ::rse_utl::cppdef::{
	UtlVector, UtlString,
};

use super::{
	ConCommandBaseVt, ConCommandBaseExt, Command,
};

pub type ConCommand = WithVTable<ConCommandVt, ConCommandExt>;

#[derive(Debug)]
#[repr(C)]
pub struct ConCommandExt {
	pub base: ConCommandBaseExt,
	pub command_callback: CommandCallback,
	pub completion_callback: CompletionCallback,
	pub bits: ConCommandBits,
}
unsafe impl PointerFrom<ConCommandExt> for ConCommandBaseExt {}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ConCommandBits(u8);

impl ConCommandBits {
	const HAS_COMPLETION_CALLBACK: u8 = 1 << 0;
	const USING_NEW_COMMAND_CALLBACK: u8 = 1 << 1;
	const USING_COMMAND_CALLBACK_INTERFACE: u8 = 1 << 2;

	pub const fn new() -> Self {
		Self(0)
	}

	pub const fn has_completion_callback(&self) -> bool {
		test_bits!(self, Self::HAS_COMPLETION_CALLBACK)
	}

	pub const fn with_completion_callback(self) -> Self {
		with_bits!(self, Self::HAS_COMPLETION_CALLBACK)
	}

	pub const fn using_new_command_callback(&self) -> bool {
		test_bits!(self, Self::USING_NEW_COMMAND_CALLBACK)
	}

	pub const fn use_new_command_callback(self) -> Self {
		with_bits!(self, Self::USING_NEW_COMMAND_CALLBACK)
	}

	pub const fn using_command_callback_interface(&self) -> bool {
		test_bits!(self, Self::USING_COMMAND_CALLBACK_INTERFACE)
	}

	pub const fn use_command_callback_interface(self) -> Self {
		with_bits!(self, Self::USING_COMMAND_CALLBACK_INTERFACE)
	}
}

#[repr(C)]
pub struct ConCommandVt {
	pub base: ConCommandBaseVt,
	pub con_command: ConCommandVtBase,
}
unsafe impl PointerFrom<ConCommandVt> for ConCommandBaseVt {}

vtable! {
	pub ConCommandVtBase for VtObjectPtr<ConCommandVt> {
		pub fn auto_complete_suggest(partial: *const c_char, commands: RefMut<UtlVector<UtlString>>) -> c_int;
		pub fn can_auto_complete() -> bool;
		pub fn dispatch(command: RefConst<Command>);
	}
}

pub type CommandCallbackFnV1 = unsafe extern "C" fn();
pub type CommandCallbackFn = unsafe extern "C" fn(command: RefConst<Command>);

#[repr(C)]
pub union CommandCallback {
	pub v1: CommandCallbackFnV1,
	pub new: CommandCallbackFn,
	pub interface: VtObjectPtr<CommandCallbackVt>,
	pub not_used: (),
}

impl fmt::Debug for CommandCallback {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("CommandCallback")
	}
}

pub const COMMAND_COMPLETION_MAX_ITEMS: usize = 64;
pub const COMMAND_COMPLETION_ITEM_LENGTH: usize = 64;

/// Type for the output of a command completion callback.
pub type CompletionArray = [CompletionArrayItem; COMMAND_COMPLETION_MAX_ITEMS];

/// Type for one item of the output of a command completion callback.
pub type CompletionArrayItem = [c_char; COMMAND_COMPLETION_ITEM_LENGTH];

const _ASSERT_COMPLETION_ARRAY_PTR_IS_THIN: () = {
	assert!(size_of::<*mut CompletionArray>() == size_of::<*mut ()>(), "`CompletionArray` pointers must be thin");
};

pub type CompletionCallbackFn = unsafe extern "C" fn(
	partial: *const c_char,
	out_commands: *mut CompletionArray,
) -> c_int;

#[repr(C)]
pub union CompletionCallback {
	pub function: CompletionCallbackFn,
	pub interface: Option<VtObjectPtr<CommandCompletionCallbackVt>>,
	pub not_used: (),
}

impl fmt::Debug for CompletionCallback {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("CompletionCallback")
	}
}

vtable! {
	pub CommandCallbackVt {
		pub fn command_callback(command: RefConst<Command>);
	}
}

vtable! {
	pub CommandCompletionCallbackVt {
		pub fn command_completion_callback(partial: *const c_char, commands: RefMut<UtlVector<UtlString>>) -> c_int;
	}
}
