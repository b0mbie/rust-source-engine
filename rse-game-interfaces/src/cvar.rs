use ::core::ffi::{
	CStr, c_float, c_int,
};
use ::rse_convar::{
	cppdef::ConVar,
	console_base::{
		CvarDllIdentifier,
		AsRegistrable, Registrable,
	},
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
	/// Returns a unique identifier that must be used for console variables and commands
	/// that are registered with this interface.
	fn allocate_dll_identifier(&mut self) -> CvarDllIdentifier {
		unsafe { virtual_call!(self.as_object() => cvar.allocate_dll_identifier()) }
	}

	/// Returns `true` if ConVars that must be set on the material thread can be set now.
	/// 
	/// Generally, this function will return `true`
	/// if the material thread is the thread setting a ConVar.
	/// If this function returns `false` while a ConVar is being set,
	/// that ConVar should call [`queue_material_thread_set`](CvarImpl::queue_material_thread_set) on itself
	/// to queue the assignment to be done again on the material thread.
	fn is_material_thread_set_allowed(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => cvar.is_material_thread_set_allowed()) }
	}

	/// Queues the given `con_var` to be set to `value` on the material thread.
	/// 
	/// This function should typically be called when
	/// [`is_material_thread_set_allowed`](CvarImpl::is_material_thread_set_allowed) returns `false`,
	/// but doing otherwise should be sound.
	/// It is advised against doing so, however.
	/// 
	/// # Safety
	/// `con_var` must be valid when the material thread sets it.
	unsafe fn queue_material_thread_set<V>(&mut self, con_var: *mut ConVar, value: V)
	where
		V: QueueMaterialThreadValue,
	{
		unsafe { V::queue_material_thread_set(self, con_var, value) }
	}

	/// Register the console variable or command `registrable` with this interface,
	/// linking it into the global linked list.
	/// 
	/// # Safety
	/// `registrable` *must* point to a [`ConCommandBase`]
	/// that can be registered with the `ICvar` interface.
	/// 
	/// Registered console commands and variables *must* eventually be unregistered with
	/// [`unregister_raw`](CvarImpl::unregister_raw) or [`unregister_all`](CvarImpl::unregister_all).
	unsafe fn register_raw(&mut self, registrable: Registrable) {
		unsafe { virtual_call!(self.as_object() => cvar.register_con_command(registrable)) }
	}

	/// Unregister the console variable or command `registrable` with this interface,
	/// unlinking it from the global linked list.
	/// 
	/// # Safety
	/// `registrable` must have been registered with this interface.
	unsafe fn unregister_raw(&mut self, registrable: Registrable) {
		unsafe { virtual_call!(self.as_object() => cvar.unregister_con_command(registrable)) }
	}

	/// Unregister all console variables and commands associated with the given `dll_identifier`.
	/// 
	/// # Safety
	/// `dll_identifier` must be a [`CvarDllIdentifier`] that was returned by a call to
	/// [`allocate_dll_identifier`](CvarImpl::allocate_dll_identifier).
	unsafe fn unregister_all(&mut self, dll_identifier: CvarDllIdentifier) {
		unsafe { virtual_call!(self.as_object() => cvar.unregister_con_commands(dll_identifier)) }
	}

	/// Register the console variable or command `registrable` with this interface,
	/// linking it into the global linked list.
	/// 
	/// # Safety
	/// Registered console commands and variables *must* eventually be unregistered with
	/// [`unregister`](CvarImpl::unregister) or [`unregister_all`](CvarImpl::unregister_all).
	unsafe fn register<C>(&mut self, registrable: &mut C)
	where
		C: AsRegistrable,
	{
		unsafe { self.register_raw(registrable.as_registrable()) }
	}

	/// Unregister the console variable or command `registrable` with this interface,
	/// unlinking it from the global linked list.
	/// 
	/// # Safety
	/// `registrable` must have been registered with this interface.
	unsafe fn unregister<C>(&mut self, registrable: &mut C)
	where
		C: AsRegistrable,
	{
		unsafe { self.unregister_raw(registrable.as_registrable()) }
	}
}
impl<T: AsObject<CvarVt>> CvarImpl for T {}

pub trait QueueMaterialThreadValue {
	/// # Safety
	/// `con_var` must be valid when the material thread sets it.
	unsafe fn queue_material_thread_set<Cvar>(cvar: &mut Cvar, con_var: *mut ConVar, value: Self)
	where
		Cvar: ?Sized + CvarImpl;
}

impl QueueMaterialThreadValue for Option<&CStr> {
	unsafe fn queue_material_thread_set<Cvar>(cvar: &mut Cvar, con_var: *mut ConVar, value: Self)
	where
		Cvar: ?Sized + CvarImpl,
	{
		unsafe {
			virtual_call!(
				cvar.as_object()
				=> cvar.queue_material_thread_set_value_string(
					con_var, value.map(move |s| s.as_ptr()).unwrap_or_default(),
				)
			)
		}
	}
}

impl QueueMaterialThreadValue for &CStr {
	unsafe fn queue_material_thread_set<Cvar>(cvar: &mut Cvar, con_var: *mut ConVar, value: Self)
	where
		Cvar: ?Sized + CvarImpl,
	{
		unsafe {
			virtual_call!(cvar.as_object() => cvar.queue_material_thread_set_value_string(con_var, value.as_ptr()))
		}
	}
}

impl QueueMaterialThreadValue for c_float {
	unsafe fn queue_material_thread_set<Cvar>(cvar: &mut Cvar, con_var: *mut ConVar, value: Self)
	where
		Cvar: ?Sized + CvarImpl,
	{
		unsafe {
			virtual_call!(cvar.as_object() => cvar.queue_material_thread_set_value_float(con_var, value))
		}
	}
}

impl QueueMaterialThreadValue for c_int {
	unsafe fn queue_material_thread_set<Cvar>(cvar: &mut Cvar, con_var: *mut ConVar, value: Self)
	where
		Cvar: ?Sized + CvarImpl,
	{
		unsafe {
			virtual_call!(cvar.as_object() => cvar.queue_material_thread_set_value_int(con_var, value))
		}
	}
}

::rse_cpp::owned_vt_object_wrapper! {
	pub struct Cvar for CvarVt;
}
unsafe impl ::rse_interface::Interface for Cvar {
	const IDENTIFIER: &CStr = CVAR_INTERFACE_VERSION;
}
impl InterfaceOfFactory for Cvar {
	type Factory = AppSystemFactory;
}