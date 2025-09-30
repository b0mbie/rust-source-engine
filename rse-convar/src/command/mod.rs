//! APIs for interacting with Console Commands, or *ConCommands*.

use ::core::ffi::CStr;

use crate::{
	console_base::CvarDllIdentifier,
	Invocation,
};

pub mod low;

/// # Safety
/// `dll_identifier` must return a valid identifier previously returned by
/// `ICvar::AllocateDLLIdentifier`.
pub unsafe trait Command {
	const NAME: &CStr;
	const HELP: Option<&CStr>;
	fn dispatch(&mut self, invokation: &Invocation);

	fn dll_identifier(&mut self) -> CvarDllIdentifier;
}

unsafe impl<'a, T: Command> low::RawCommand<'a> for T {
	fn name(object: &mut low::ConCommandObject<Self>) {
		unsafe { object.as_mut_base().as_mut_inner().name = T::NAME.as_ptr() }
	}
	fn dispatch(object: &mut low::ConCommandObject<Self>, invocation: &Invocation) {
		Command::dispatch(object.as_mut_inner(), invocation)
	}
}
unsafe impl<T: Command> crate::console_base::RawConsoleBase<low::ConCommandObject<'_, T>> for T {
	fn help(object: &mut low::ConCommandObject<T>) {
		unsafe { object.as_mut_base().as_mut_inner().help_string = crate::util::c_str_ptr(T::HELP) }
	}
	fn dll_identifier(object: &mut low::ConCommandObject<T>) -> CvarDllIdentifier {
		T::dll_identifier(object.as_mut_inner())
	}
}
