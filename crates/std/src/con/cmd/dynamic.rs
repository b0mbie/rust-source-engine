use ::core::ffi::CStr;
use ::rse_convar::console_base::{
	CvarFlags,
	RegistrableMut,
};

use super::{
	GenericConCommand, GenericCommand,
	DispatchFn, CompleteFn,
	DispatchPlainFn, DispatchWithFn,
	Callbacks,
};

#[repr(transparent)]
pub struct ConCommand {
	inner: GenericConCommand<'static, DynConCommand>,
}

impl ConCommand {
	pub const fn plain(
		name: &'static CStr, help: Option<&'static CStr>,
		flags: CvarFlags,
		dispatch: DispatchPlainFn,
		complete: Option<CompleteFn>,
	) -> Self {
		Self::with_callbacks(
			name, help, flags,
			Callbacks::Functions {
				dispatch: DispatchFn::Plain(dispatch),
				complete,
			},
		)
	}

	pub const fn with_args(
		name: &'static CStr, help: Option<&'static CStr>,
		flags: CvarFlags,
		dispatch: DispatchWithFn,
		complete: Option<CompleteFn>,
	) -> Self {
		Self::with_callbacks(
			name, help, flags,
			Callbacks::Functions {
				dispatch: DispatchFn::With(dispatch),
				complete,
			},
		)
	}

	pub const fn with_callbacks(
		name: &'static CStr, help: Option<&'static CStr>,
		flags: CvarFlags,
		callbacks: Callbacks,
	) -> Self {
		Self {
			inner: GenericConCommand::new(
				DynConCommand,
				name, help, flags,
				callbacks,
			),
		}
	}

	pub fn register(&'static self) -> bool {
		self.inner.register()
	}

	pub fn as_registrable(&self) -> RegistrableMut {
		self.inner.as_registrable()
	}
}

#[derive(Debug, Clone, Copy, Hash)]
struct DynConCommand;

impl<'a> GenericCommand<'a> for DynConCommand {}
