use ::core::{
	cell::UnsafeCell,
	ffi::CStr,
};
use ::rse_convar::{
	cppdef::ConCommand,
	console_base::{
		RegistrableMut,
		CvarFlags,
	},
	command::low::ConCommandObject,
};

use super::DispatchCommand;

mod wrapper;
use wrapper::StdCommand;

#[repr(transparent)]
pub struct GenericConCommand<'str, T> {
	con_command: UnsafeCell<ConCommandObject<'str, StdCommand<T>>>,
}

unsafe impl<'str, T: Sync> Sync for GenericConCommand<'str, T> {}

impl<'str, T> GenericConCommand<'str, T>
where
	T: DispatchCommand,
{
	pub const fn new(
		inner: T,
		name: &'str CStr, help: Option<&'str CStr>, flags: CvarFlags,
	) -> Self {
		Self {
			con_command: UnsafeCell::new(ConCommandObject::new(
				StdCommand::new(inner),
				name, help, flags,
			)),
		}
	}

	pub const fn as_inner(&self) -> &ConCommand {
		unsafe { (*self.con_command.get()).as_inner() }
	}

	pub fn register(&'static self) -> bool {
		unsafe { crate::con::cvar::register_raw(self.as_registrable()) }
	}

	pub fn as_registrable(&self) -> RegistrableMut {
		unsafe { (*self.con_command.get()).as_registrable() }
	}
}
