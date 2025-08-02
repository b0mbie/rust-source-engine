use ::core::ffi::{
	CStr, c_char, c_float, c_int,
};
use ::rse_cpp::{
	vtable, RefConst, VtObjectMut, VtObjectRef,
};

use super::{
	app_system::AppSystemVt,
	convar::{
		ConVarVt, ConCommandBaseVt, ConCommandVt,
	},
	Color,
};

/// Type for a DLL identifier that's used to mark ConVars and ConCommands.
pub type CvarDllIdentifier = c_int;

pub type FnChangeCallback = unsafe extern "C-unwind" fn(
	var: VtObjectMut<ConVarVt>, old_string: *const c_char, old_value: c_float,
);

pub const CVAR_INTERFACE_VERSION: &CStr = c"VEngineCvar004";
vtable! {
	pub CvarVtBase for VtObjectMut<CvarVt> {
		pub fn allocate_dll_identifier() -> CvarDllIdentifier;
		pub fn register_con_command(command_base: VtObjectMut<ConCommandBaseVt>);
		pub fn unregister_con_command(command_base: VtObjectMut<ConCommandBaseVt>);
		pub fn unregister_con_commands(id: CvarDllIdentifier);
		pub fn get_command_line_value(name: *const c_char) -> *const c_char;
		pub fn find_command_base(name: *const c_char) -> Option<VtObjectMut<ConCommandBaseVt>>;
		pub fn find_command_base_const(name: *const c_char) -> Option<VtObjectRef<ConCommandBaseVt>>;
		pub fn find_var(name: *const c_char) -> Option<VtObjectMut<ConVarVt>>;
		pub fn find_var_const(name: *const c_char) -> Option<VtObjectRef<ConVarVt>>;
		pub fn find_command(name: *const c_char) -> Option<VtObjectMut<ConCommandVt>>;
		pub fn find_command_const(name: *const c_char) -> Option<VtObjectRef<ConCommandVt>>;
		pub fn get_commands() -> VtObjectMut<ConCommandBaseVt>;
		pub fn get_commands_const() -> VtObjectRef<ConCommandBaseVt>;
		pub fn install_global_change_callback(callback: FnChangeCallback);
		pub fn remove_global_change_callback(callback: FnChangeCallback);
		pub fn call_global_change_callbacks(var: VtObjectMut<ConVarVt>, old_string: *const c_char, old_value: c_float);
		pub fn install_console_display_func(display_func: VtObjectMut<ConsoleDisplayFuncVt>);
		pub fn remove_console_display_func(display_func: VtObjectMut<ConsoleDisplayFuncVt>);
		pub fn console_color_printf(color: RefConst<Color>, format: *const c_char, ...);
		pub fn console_printf(format: *const c_char, ...);
		pub fn console_dprintf(format: *const c_char, ...);
		pub fn revert_flagged_convars(flag: c_int);
		pub fn install_cvar_query(query: VtObjectMut<CvarQueryVt>);
		pub fn is_material_thread_set_allowed() -> bool;
		pub fn queue_material_thread_set_value_string(convar: VtObjectMut<ConVarVt>, value: *const c_char);
		pub fn queue_material_thread_set_value_int(convar: VtObjectMut<ConVarVt>, value: c_int);
		pub fn queue_material_thread_set_value_float(convar: VtObjectMut<ConVarVt>, value: c_float);
		pub fn has_queued_material_thread_convar_sets() -> bool;
		pub fn process_queued_material_thread_convar_sets() -> c_int;
	}
}

#[repr(C)]
pub struct CvarVt {
	pub base: AppSystemVt,
	pub cvar: CvarVtBase,
}

vtable! {
	pub ConsoleDisplayFuncVt {
		pub fn color_print(color: RefConst<Color>, message: *const c_char);
		pub fn print(message: *const c_char);
		pub fn dprint(message: *const c_char);
	}
}

pub const CVAR_QUERY_INTERFACE_VERSION: &CStr = c"VCvarQuery001";
vtable! {
	pub CvarQueryVtBase for VtObjectMut<CvarQueryVt> {
		pub fn are_convars_linkable(child: VtObjectRef<ConVarVt>, parent: VtObjectRef<ConVarVt>) -> bool;
	}
}

#[repr(C)]
pub struct CvarQueryVt {
	pub base: AppSystemVt,
	pub cvar_query: CvarQueryVtBase,
}
