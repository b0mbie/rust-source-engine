use ::core::ffi::{
	CStr, c_char, c_float, c_int,
};
use ::rse_cpp::{
	class, RefConst,
};
use ::rse_interface::cppdef::app_system::IAppSystem;
use ::rse_math::Color;

use crate::cppdef::{
	ConCommandBase, ConCommand, ConVar,
	CvarDllIdentifier,
	FnChangeCallback,
};

pub const CVAR_INTERFACE_VERSION: &CStr = c"VEngineCvar004";
class! {
	pub ICvar: pub virtual IAppSystem {
		pub virtual fn allocate_dll_identifier() -> CvarDllIdentifier;
		pub virtual fn register_con_command(command_base: *mut ConCommandBase);
		pub virtual fn unregister_con_command(command_base: *mut ConCommandBase);
		pub virtual fn unregister_con_commands(id: CvarDllIdentifier);
		pub virtual fn get_command_line_value(name: *const c_char) -> *const c_char;
		pub virtual fn find_command_base(name: *const c_char) -> *mut ConCommandBase;
		pub virtual fn find_command_base_const(name: *const c_char) -> *const ConCommandBase;
		pub virtual fn find_var(name: *const c_char) -> *mut ConVar;
		pub virtual fn find_var_const(name: *const c_char) -> *const ConVar;
		pub virtual fn find_command(name: *const c_char) -> *mut ConCommand;
		pub virtual fn find_command_const(name: *const c_char) -> *const ConCommand;
		pub virtual fn get_commands() -> *mut ConCommandBase;
		pub virtual fn get_commands_const() -> *const ConCommandBase;
		pub virtual fn install_global_change_callback(callback: FnChangeCallback);
		pub virtual fn remove_global_change_callback(callback: FnChangeCallback);
		pub virtual fn call_global_change_callbacks(var: *mut ConVar, old_string: *const c_char, old_value: c_float);
		pub virtual fn install_console_display_func(display_func: *mut IConsoleDisplayFunc);
		pub virtual fn remove_console_display_func(display_func: *mut IConsoleDisplayFunc);
		pub virtual fn console_color_printf(color: RefConst<Color>, format: *const c_char, ...);
		pub virtual fn console_printf(format: *const c_char, ...);
		pub virtual fn console_dprintf(format: *const c_char, ...);
		pub virtual fn revert_flagged_convars(flag: c_int);
		pub virtual fn install_cvar_query(query: *mut ICvarQuery);
		#[cfg(feature = "xbox360")]
		pub virtual fn publish_to_vx_console();
		pub virtual fn is_material_thread_set_allowed() -> bool;
		pub virtual fn queue_material_thread_set_value_string(convar: *mut ConVar, value: *const c_char);
		pub virtual fn queue_material_thread_set_value_int(convar: *mut ConVar, value: c_int);
		pub virtual fn queue_material_thread_set_value_float(convar: *mut ConVar, value: c_float);
		pub virtual fn has_queued_material_thread_convar_sets() -> bool;
		pub virtual fn process_queued_material_thread_convar_sets() -> c_int;
	}
}

class! {
	pub IConsoleDisplayFunc {
		pub virtual fn color_print(color: RefConst<Color>, message: *const c_char);
		pub virtual fn print(message: *const c_char);
		pub virtual fn dprint(message: *const c_char);
	}
}

pub const CVAR_QUERY_INTERFACE_VERSION: &CStr = c"VCvarQuery001";
class! {
	pub ICvarQuery: pub virtual IAppSystem {
		pub virtual fn are_convars_linkable(child: *const ConVar, parent: *const ConVar) -> bool;
	}
}
