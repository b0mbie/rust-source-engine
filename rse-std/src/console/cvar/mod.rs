use ::core::ffi::{
	CStr, c_float,
};
use ::rse_convar::{
	cppdef::ConVar,
	console_base::RegistrableMut,
};
use ::rse_game_interfaces::{
	cvar::{
		Cvar, CvarImpl,
		QueueMaterialThreadValue,
	},
	InterfaceFactories,
};

static mut CVAR: Option<Cvar> = None;

pub unsafe fn call_global_change_callbacks(registered: *mut ConVar, old_string: &CStr, old_float: c_float) {
	#[allow(static_mut_refs)]
	unsafe {
		if let Some(cvar) = CVAR.as_ref() {
			cvar.call_global_change_callbacks(registered, old_string, old_float);
		}
	}
}

pub unsafe fn is_material_thread_set_allowed() -> bool {
	#[allow(static_mut_refs)]
	unsafe {
		if let Some(cvar) = CVAR.as_ref() {
			cvar.is_material_thread_set_allowed()
		} else {
			false
		}
	}
}

pub unsafe fn queue_material_thread_set<V: QueueMaterialThreadValue>(con_var: *mut ConVar, value: V) {
	#[allow(static_mut_refs)]
	unsafe {
		if let Some(cvar) = CVAR.as_mut() {
			cvar.queue_material_thread_set(con_var, value);
		}
	}
}

pub unsafe fn register_raw(registrable: RegistrableMut) -> bool {
	crate::threads::MAIN_THREAD.try_run(move || {
		#[allow(static_mut_refs)]
		unsafe {
			if let Some(cvar) = CVAR.as_mut() {
				cvar.register_raw(registrable);
				true
			} else {
				false
			}
		}
	}).unwrap_or(false)
}

/// # Safety
/// This function must be called from the main thread.
/// 
/// A call to this function must eventually be followed by a call to [`detach`].
pub unsafe fn attach(factories: InterfaceFactories<'_>) {
	match factories.create_interface::<Cvar>() {
		Ok(mut iface) => {
			unsafe { set_dll_identifier(iface.allocate_dll_identifier()) };
			unsafe { CVAR = Some(iface) };
		}
		Err(error) => {
			::rse_tier0::con_warn!("{error}");
		}
	}
}

/// # Safety
/// This function must be called from the main thread.
pub unsafe fn detach() {
	let dll_id = dll_identifier();
	#[allow(static_mut_refs)]
	unsafe {
		if dll_id >= FIRST_INIT_DLL_ID
			&& let Some(cvar) = CVAR.as_mut()
		{
			cvar.unregister_all(dll_id);
			reset_dll_identifier();
		}
	}
}

mod dll_id;
pub use dll_id::*;