use ::core::{
	ffi::CStr,
	mem::transmute,
	ops::{
		Deref, DerefMut,
	},
	pin::Pin,
};
use ::rse_convar::{
	cppdef::{
		CommandCallback, CompletionCallback,
		CommandCallbackFnV1, CommandCallbackFn, CompletionCallbackFn,
	},
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

use super::{
	super::{
		Suggestions, Invocation, GenericCommand,
	},
	DispatchPlainFn, DispatchWithFn, CompleteFn,
};

pub(super) const fn dispatch_fn(f: DispatchPlainFn) -> CommandCallback {
	CommandCallback {
		v1: unsafe { transmute::<DispatchPlainFn, CommandCallbackFnV1>(f) }
	}
}

pub(super) const fn dispatch_with_fn(f: DispatchWithFn) -> CommandCallback {
	CommandCallback {
		new: unsafe { transmute::<DispatchWithFn, CommandCallbackFn>(f) }
	}
}

pub(super) const fn complete_fn(f: CompleteFn) -> CompletionCallback {
	CompletionCallback {
		function: unsafe { transmute::<CompleteFn, CompletionCallbackFn>(f) }
	}
}

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

impl<T> Deref for StdCommand<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl<T> DerefMut for StdCommand<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

unsafe impl<'str, T> RawCommand<'str> for StdCommand<T>
where
	T: GenericCommand<'str>,
{
	fn name(object: Pin<&mut ConCommandObject<'str, Self>>) {
		let _ = object;
		// unsafe { object.as_mut_base().as_mut_inner().name = T::NAME.as_ptr() }
	}
	fn dispatch(object: Pin<&mut ConCommandObject<'str, Self>>, invocation: &Invocation) {
		let data = &object.as_inner().data;
		if data.bits.using_command_callback_interface() {
			// TODO: Use command callback interface!?
		} else if data.bits.using_new_command_callback() {
			let f = unsafe { transmute::<CommandCallbackFn, DispatchWithFn>(data.command_callback.new) };
			f(invocation)
		} else {
			let f = unsafe { transmute::<CommandCallbackFnV1, DispatchPlainFn>(data.command_callback.v1) };
			f()
		}
	}
	fn can_auto_complete(object: Pin<&mut ConCommandObject<'str, Self>>) -> bool {
		object.as_inner().data.bits.has_completion_callback()
	}
	fn auto_complete_suggest(
		object: Pin<&mut ConCommandObject<'str, Self>>,
		partial: &CStr,
		suggestions: &mut Suggestions,
	) -> SuggestionCount {
		let data = &object.as_inner().data;
		if data.bits.using_command_callback_interface() {
			// TODO: Use command callback interface!
			SuggestionCount::ZERO
		} else {
			let f = unsafe { transmute::<CompletionCallbackFn, CompleteFn>(data.completion_callback.function) };
			f(partial, suggestions);
			suggestions.count()
		}
	}
}

unsafe impl<'str, T> RawConsoleBase<ConCommandObject<'str, Self>> for StdCommand<T> {
	fn help(object: Pin<&mut ConCommandObject<'str, Self>>) {
		let _ = object;
		// unsafe { object.as_mut_base().as_mut_inner().help_string = crate::util::c_str_ptr(T::HELP) }
	}
	fn add_flags(object: Pin<&mut ConCommandObject<'str, Self>>, flags: CvarFlags) {
		unsafe { object.get_unchecked_mut().as_mut_base().add_flags(flags) }
	}
	fn is_registered(object: Pin<&mut ConCommandObject<'str, Self>>) -> bool {
		object.as_base().is_registered()
	}
	fn dll_identifier(object: Pin<&mut ConCommandObject<'str, Self>>) -> CvarDllIdentifier {
		let _ = object;
		crate::con::cvar::raw::dll_identifier()
	}
}
