//! APIs for interacting with Console Commands, or *ConCommands*.

use ::core::ffi::CStr;

use crate::{
	console_base::CvarDllIdentifier,
	Invocation,
};

pub mod low;

pub use low::Suggestions;

#[cfg(feature = "macros")]
mod macros;

/// Returns a new [`ConCommandObject`](low::ConCommandObject) that delegates execution to `T`.
pub const fn con_command<T>(command: T) -> low::ConCommandObject<'static, T>
where
	T: Command,
{
	low::ConCommandObject::new(command, T::NAME, T::HELP)
}

/// # Safety
/// `dll_identifier` must return a valid identifier previously returned by
/// `ICvar::AllocateDLLIdentifier`.
pub unsafe trait Command {
	const NAME: &CStr;
	const HELP: Option<&CStr> = None;
	fn dispatch(&mut self, invocation: &Invocation);
	fn can_auto_complete(&mut self) -> bool {
		false
	}
	fn auto_complete(&mut self, partial: &CStr, suggestions: &mut Suggestions) {
		let _ = partial;
		let _ = suggestions;
	}
	fn dll_identifier(&mut self) -> CvarDllIdentifier;
}

unsafe impl<'a, T: Command> low::RawCommand<'a> for T {
	fn name(object: &mut low::ConCommandObject<'a, Self>) {
		let _ = object;
		// unsafe { object.as_mut_base().as_mut_inner().name = T::NAME.as_ptr() }
	}
	fn dispatch(object: &mut low::ConCommandObject<'a, Self>, invocation: &Invocation) {
		object.inner.dispatch(invocation)
	}
	fn can_auto_complete(object: &mut low::ConCommandObject<'a, Self>) -> bool {
		object.inner.can_auto_complete()
	}
	fn auto_complete_suggest(
		object: &mut low::ConCommandObject<'a, Self>,
		partial: &CStr,
		suggestions: &mut Suggestions,
	) -> low::SuggestionCount {
		object.inner.auto_complete(partial, suggestions);
		suggestions.count()
	}
}
unsafe impl<'a, T: Command> crate::console_base::RawConsoleBase<low::ConCommandObject<'a, T>> for T {
	fn init(object: &mut low::ConCommandObject<'a, T>) {
		let _ = object;
	}
	fn help(object: &mut low::ConCommandObject<'a, T>) {
		let _ = object;
		// unsafe { object.as_mut_base().as_mut_inner().help_string = crate::util::c_str_ptr(T::HELP) }
	}
	fn dll_identifier(object: &mut low::ConCommandObject<'a, T>) -> CvarDllIdentifier {
		object.inner.dll_identifier()
	}
}
