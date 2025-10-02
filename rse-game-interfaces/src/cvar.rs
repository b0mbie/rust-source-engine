use ::core::ffi::CStr;
use ::rse_convar::console_base::{
	CvarDllIdentifier, AsConCommandBase,
};
use ::rse_cpp::{
	AsObject,
	virtual_call,
};

use crate::cppdef::{
	CVAR_INTERFACE_VERSION, CvarVt,
};

use super::{
	InterfaceOfFactory, AppSystemFactory,
};

pub trait CvarImpl: AsObject<CvarVt> {
	fn allocate_dll_identifier(&mut self) -> CvarDllIdentifier {
		unsafe { virtual_call!(self.as_object() => cvar.allocate_dll_identifier()) }
	}
	fn is_material_thread_set_allowed(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => cvar.is_material_thread_set_allowed()) }
	}

	/// # Safety
	/// Registered console commands and variables *must* eventually be unregistered with
	/// [`unregister`](CvarImpl::unregister) or [`unregister_all`](CvarImpl::unregister_all).
	unsafe fn register<C>(&mut self, command: &mut C)
	where
		C: AsConCommandBase,
	{
		unsafe { virtual_call!(self.as_object() => cvar.register_con_command(command.as_con_command_base())) }
	}

	/// # Safety
	/// `command` must have been registered with this interface.
	unsafe fn unregister<C>(&mut self, command: &mut C)
	where
		C: AsConCommandBase,
	{
		unsafe { virtual_call!(self.as_object() => cvar.unregister_con_command(command.as_con_command_base())) }
	}

	/// # Safety
	/// `dll_identifier` must be a [`CvarDllIdentifier`] that was returned by a call to
	/// [`allocate_dll_identifier`](CvarImpl::allocate_dll_identifier).
	unsafe fn unregister_all(&mut self, dll_identifier: CvarDllIdentifier) {
		unsafe { virtual_call!(self.as_object() => cvar.unregister_con_commands(dll_identifier)) }
	}
}
impl<T: AsObject<CvarVt>> CvarImpl for T {}

::rse_cpp::owned_vt_object_wrapper! {
	pub struct Cvar for CvarVt;
}
unsafe impl ::rse_interface::Interface for Cvar {
	const IDENTIFIER: &CStr = CVAR_INTERFACE_VERSION;
}
impl InterfaceOfFactory for Cvar {
	type Factory = AppSystemFactory;
}