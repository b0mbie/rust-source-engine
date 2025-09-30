use ::core::ffi::{
	c_char, c_int, c_float,
};
use ::rse_cpp::vtable;

use super::ConCommandBaseVt;

// TODO: Add a `typeinfo` field for `ConVar`.
// This is because of a `dynamic_cast` to `ConVar_ServerBounded` in `ConVar_PrintDescription`.
// It can just be null,
// in which case the cast will just always fail due to a simple pointer comparison failure in Itanium.
#[repr(C)]
pub struct ConVarVt {
	pub con_command_base: ConCommandBaseVt,
	pub iface: ConVarIfaceVt,
	pub convar: ConVarVtBase,
}

vtable! {
	pub ConVarIfaceVt {
		pub fn set_value_string(value: *const c_char);
		pub fn set_value_float(value: *const c_float);
		pub fn set_value_int(value: *const c_int);
		pub fn get_name() -> *const c_char;
		pub fn is_flag_set(flag: c_int) -> bool;
	}
}

// TODO: `ConVar`.
vtable! {
	pub ConVarVtBase for VtObjectPtr<ConVarVt> {}
}
