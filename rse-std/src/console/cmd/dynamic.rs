use ::core::ffi::CStr;
use ::rse_convar::{
	console_base::CvarFlags,
	command::{
		Invocation, Suggestions,
	},
};

use super::{
	GenericConCommand, DispatchCommand,
};

pub type DispatchCallback = fn(&Invocation);
pub type CompleteCallback = fn(&CStr, &mut Suggestions);

#[repr(transparent)]
pub struct ConCommand {
	inner: GenericConCommand<DynConCommand>,
}

impl ConCommand {
	pub const fn new(
		name: &'static CStr, help: Option<&'static CStr>,
		flags: CvarFlags,
		dispatch: DispatchCallback,
		complete: Option<CompleteCallback>,
	) -> Self {
		Self {
			inner: GenericConCommand::new(
				DynConCommand {
					dispatch,
					complete,
				},
				name, help, flags,
			),
		}
	}

	pub fn register(&self) {
		self.inner.register()
	}
}

#[derive(Debug, Clone, Copy, Hash)]
struct DynConCommand {
	pub dispatch: DispatchCallback,
	pub complete: Option<CompleteCallback>,
}

impl DispatchCommand for DynConCommand {
	fn dispatch(&mut self, invocation: &Invocation) {
		(self.dispatch)(invocation)
	}
	fn can_auto_complete(&mut self) -> bool {
		self.complete.is_some()
	}
	fn auto_complete(&mut self, partial: &CStr, suggestions: &mut Suggestions) {
		if let Some(complete) = self.complete {
			complete(partial, suggestions)
		}
	}
}
