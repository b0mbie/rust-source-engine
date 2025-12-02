use ::core::{
	ffi::CStr,
	pin::Pin,
};
use ::rse_convar::{
	console_base::{
		RawConsoleBase,
		CvarDllIdentifier, CvarFlags,
	},
	command::{
		low::{
			RawCommand, ConCommandObject,
		},
		SuggestionCount,
	},
};

use super::super::{
	Suggestions, Invocation, DispatchCommand,
};

#[repr(transparent)]
pub struct StdCommand<T> {
	pub inner: T,
}

impl<T> StdCommand<T> {
	pub const fn new(inner: T) -> Self {
		Self {
			inner,
		}
	}
}

unsafe impl<'a, T> RawCommand<'a> for StdCommand<T>
where
	T: DispatchCommand,
{
	fn name(object: Pin<&mut ConCommandObject<'a, Self>>) {
		let _ = object;
		// unsafe { object.as_mut_base().as_mut_inner().name = T::NAME.as_ptr() }
	}
	fn dispatch(object: Pin<&mut ConCommandObject<'a, Self>>, invocation: &Invocation) {
		unsafe { object.get_unchecked_mut().inner.dispatch(invocation) }
	}
	fn can_auto_complete(object: Pin<&mut ConCommandObject<'a, Self>>) -> bool {
		unsafe { object.get_unchecked_mut().inner.can_auto_complete() }
	}
	fn auto_complete_suggest(
		object: Pin<&mut ConCommandObject<'a, Self>>,
		partial: &CStr,
		suggestions: &mut Suggestions,
	) -> SuggestionCount {
		unsafe { object.get_unchecked_mut().inner.auto_complete(partial, suggestions) };
		suggestions.count()
	}
}

unsafe impl<'a, T> RawConsoleBase<ConCommandObject<'a, Self>> for StdCommand<T>
where
	T: DispatchCommand,
{
	fn help(object: Pin<&mut ConCommandObject<'a, Self>>) {
		let _ = object;
		// unsafe { object.as_mut_base().as_mut_inner().help_string = crate::util::c_str_ptr(T::HELP) }
	}
	fn add_flags(object: Pin<&mut ConCommandObject<'a, Self>>, flags: CvarFlags) {
		unsafe { object.get_unchecked_mut().as_mut_base().add_flags(flags) }
	}
	fn is_registered(object: Pin<&mut ConCommandObject<'a, Self>>) -> bool {
		object.as_base().is_registered()
	}
	fn dll_identifier(object: Pin<&mut ConCommandObject<'a, Self>>) -> CvarDllIdentifier {
		let _ = object;
		crate::con::cvar::dll_identifier()
	}
}

impl<T> DispatchCommand for StdCommand<T>
where
	T: DispatchCommand,
{
	fn dispatch(&mut self, invocation: &Invocation) {
		self.inner.dispatch(invocation)
	}
	fn can_auto_complete(&mut self) -> bool {
		self.inner.can_auto_complete()
	}
	fn auto_complete(&mut self, partial: &CStr, suggestions: &mut Suggestions) {
		self.inner.auto_complete(partial, suggestions)
	}
}
