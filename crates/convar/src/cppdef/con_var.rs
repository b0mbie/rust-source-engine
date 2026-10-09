use ::core::{
	ffi::{
		c_char, c_int, c_float,
	},
	marker::PhantomPinned,
};
use ::rse_cpp::{
	class,
	RefMut,
	TypeInfo,
};

use super::{
	CvarDllIdentifier,
	Registrable,
};

pub type FnChangeCallback = unsafe extern "C" fn(
	var: *mut IConVar, old_string: *const c_char, old_value: c_float,
);

class! {
	pub ConVar: pub virtual Registrable, pub virtual IConVar {
		#[cfg(not(windows))]
		pub virtual fn is_flag_set(flag: c_int) -> bool;
		pub virtual fn add_flags(flags: c_int);
		#[cfg(not(windows))]
		pub virtual fn get_name() -> *const c_char;
		pub virtual fn get_help_text() -> *const c_char;
		pub virtual fn is_registered() -> bool;
		pub virtual fn get_dll_identifier() -> CvarDllIdentifier;
		pub virtual fn create_base(name: *const c_char, help_string: *const c_char, flags: c_int);
		pub virtual fn init();

		#[cfg(not(windows))]
		pub virtual fn set_value_string(value: *const c_char);
		#[cfg(not(windows))]
		pub virtual fn set_value_float(value: c_float);
		#[cfg(not(windows))]
		pub virtual fn set_value_int(value: c_int);

		pub virtual fn internal_set_value(value: *const c_char);
		pub virtual fn internal_set_float_value(value: c_float);
		pub virtual fn internal_set_int_value(value: c_int);
		pub virtual fn clamp_value(value: RefMut<c_float>) -> bool;

		pub virtual fn change_string_value(new_value: *const c_char, old_value: c_float);

		pub virtual fn create_vtbl(
			name: *const c_char, default_value: *const c_char,
			flags: c_int,
			help_string: *const c_char,
			has_min: bool, min_value: c_float,
			has_max: bool, max_value: c_float,
			change_callback: FnChangeCallback,
		);

		pub virtual fn internal_set_float_value_2(value: c_float, force: bool);

		/// Parent of the `ConVar`,
		/// or a pointer to this one.
		pub parent: *mut ConVar;
		/// Pin for the `parent` field,
		/// because `parent` can point to this [`ConVar`].
		pub parent_pin: PhantomPinned;

		/// Default value of the ConVar. Must be static.
		pub default_value: *const c_char;
		/// Current string value of the ConVar.
		/// 
		/// Actual meaning depends on the behavior defined in the VTable
		/// (the string may or may not be dynamically-allocated),
		/// but it must always point to a valid C string.
		/// 
		/// `ConVar` can handle this value being null by returning `""`,
		/// but `ConVarRef` will return this pointer verbatim,
		/// so all supporting Rust code will assume this to be a non-null value.
		pub value_string: *mut c_char;
		/// Length of the C string pointed to by `value_string`, in bytes.
		/// 
		/// Actual meaning depends on the behavior defined in the VTable.
		pub string_length: c_int;
		pub value_float: c_float;
		pub value_int: c_int;
		pub has_min: bool;
		pub min_value: c_float;
		pub has_max: bool;
		pub max_value: c_float;
		pub change_callback: Option<FnChangeCallback>;

		pub has_comp_min: bool;
		pub comp_min_value: c_float;
		pub has_comp_max: bool;
		pub comp_max_value: c_float;
		pub using_competitive_restrictions: bool;
	}
}

#[repr(C)]
pub struct ConVarVtFull {
	pub offset_to_derived: isize,
	pub type_info: *const TypeInfo,
	// This is what will be pointed to by `ConVar`.
	pub base: ConVarVt,
}

class! {
	pub IConVar {
		pub virtual fn set_value_string(value: *const c_char);
		pub virtual fn set_value_float(value: c_float);
		pub virtual fn set_value_int(value: c_int);

		pub virtual fn get_name() -> *const c_char;
		pub virtual fn is_flag_set(flag: c_int) -> bool;
	}
}
