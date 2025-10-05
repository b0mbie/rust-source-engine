use ::core::ffi::CStr;

use crate::{
	console_base::{
		RawConsoleBase, CvarDllIdentifier, CvarFlags,
	},
	Invocation,
};

use super::{
	low::{
		ConCommandObject, RawCommand,
	},
	Suggestions, SuggestionCount,
};

/// Returns a new [`ConCommandObject`] that delegates execution to `T`.
pub const fn con_command<T>(command: T) -> ConCommandObject<'static, T>
where
	T: DllCommand,
{
	ConCommandObject::new(command, T::NAME, T::HELP, T::FLAGS)
}

pub trait Command {
	const NAME: &CStr;
	const HELP: Option<&CStr> = None;
	const FLAGS: CvarFlags = 0;
	fn dispatch(&mut self, invocation: &Invocation);
	fn can_auto_complete(&mut self) -> bool {
		false
	}
	fn auto_complete(&mut self, partial: &CStr, suggestions: &mut Suggestions) {
		let _ = partial;
		let _ = suggestions;
	}
}

/// # Safety
/// `dll_identifier` must return a valid identifier previously returned by
/// `ICvar::AllocateDLLIdentifier`.
pub unsafe trait DllCommand: Command {
	fn dll_identifier(&mut self) -> CvarDllIdentifier;
}

unsafe impl<'a, T: DllCommand> RawCommand<'a> for T {
	fn name(object: &mut ConCommandObject<'a, Self>) {
		let _ = object;
		// unsafe { object.as_mut_base().as_mut_inner().name = T::NAME.as_ptr() }
	}
	fn dispatch(object: &mut ConCommandObject<'a, Self>, invocation: &Invocation) {
		object.inner.dispatch(invocation)
	}
	fn can_auto_complete(object: &mut ConCommandObject<'a, Self>) -> bool {
		object.inner.can_auto_complete()
	}
	fn auto_complete_suggest(
		object: &mut ConCommandObject<'a, Self>,
		partial: &CStr,
		suggestions: &mut Suggestions,
	) -> SuggestionCount {
		object.inner.auto_complete(partial, suggestions);
		suggestions.count()
	}
}

unsafe impl<'a, T: DllCommand> RawConsoleBase<ConCommandObject<'a, T>> for T {
	fn help(object: &mut ConCommandObject<'a, T>) {
		let _ = object;
		// unsafe { object.as_mut_base().as_mut_inner().help_string = crate::util::c_str_ptr(T::HELP) }
	}
	fn add_flags(object: &mut ConCommandObject<'a, T>, flags: CvarFlags) {
		object.as_mut_base().add_flags(flags)
	}
	fn is_registered(object: &mut ConCommandObject<'a, T>) -> bool {
		object.as_base().is_registered()
	}
	fn dll_identifier(object: &mut ConCommandObject<'a, T>) -> CvarDllIdentifier {
		object.inner.dll_identifier()
	}
}

