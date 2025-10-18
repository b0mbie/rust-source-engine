use ::core::{
	cell::UnsafeCell,
	ffi::CStr,
};
use ::rse_convar::{
	console_base::{
		AsRegistrable, Registrable,
		CvarFlags,
	},
	command::low::ConCommandObject,
};
use ::rse_game_interfaces::cvar::CvarImpl;

use crate::console::cvar::cvar_write;

use super::DispatchCommand;

mod wrapper;
use wrapper::StdCommand;

#[repr(transparent)]
pub struct GenericConCommand<T> {
	con_command: UnsafeCell<ConCommandObject<'static, StdCommand<T>>>,
}

unsafe impl<T: Sync> Sync for GenericConCommand<T> {}

impl<T> GenericConCommand<T>
where
	T: DispatchCommand,
{
	pub const fn new(
		inner: T,
		name: &'static CStr, help: Option<&'static CStr>, flags: CvarFlags,
	) -> Self {
		Self {
			con_command: UnsafeCell::new(ConCommandObject::new(
				StdCommand::new(inner),
				name, help, flags,
			)),
		}
	}

	pub fn register(&self) {
		if let Some(cvar) = cvar_write().as_mut() {
			unsafe { cvar.register_raw(self.as_registrable()) }
		}
	}

	fn as_registrable(&self) -> Registrable {
		unsafe { (*self.con_command.get()).as_registrable() }
	}
}
