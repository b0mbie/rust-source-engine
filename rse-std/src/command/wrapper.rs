use ::core::ffi::CStr;
use ::rse_convar::{
	console_base::{
		CvarDllIdentifier, CvarFlags,
	},
	command::{
		Suggestions,
		DllCommand, Command,
	},
	Invocation,
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

unsafe impl<T> DllCommand for StdCommand<T>
where
	T: Command,
{
	fn dll_identifier(&mut self) -> CvarDllIdentifier {
		crate::cvar::dll_identifier()
	}
}

impl<T> Command for StdCommand<T>
where
	T: Command,
{
	const NAME: &CStr = T::NAME;
	const HELP: Option<&CStr> = T::HELP;
	const FLAGS: CvarFlags = T::FLAGS;
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
