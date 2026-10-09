use ::core::ffi::{
	c_char, c_float, c_int,
};
use ::rse_cpp::{
	class, RefMut,
};

class! {
	pub ICommandLine {
		pub virtual fn create_cmd_line(command_line: *const c_char);
		pub virtual fn create_cmd_line_from_vector(argc: c_int, argv: *mut *mut c_char);
		pub virtual fn get_cmd_line() -> *const c_char;

		pub virtual fn check_parm(key: *const c_char, out_value: *mut *const c_char) -> *const c_char;
		pub virtual fn remove_parm(parm: *const c_char);
		pub virtual fn append_parm(parm: *const c_char, values: *const c_char);

		pub virtual fn parm_value_str(key: *const c_char, default: *const c_char) -> *const c_char;
		pub virtual fn parm_value_int(key: *const c_char, default: c_int) -> c_int;
		pub virtual fn parm_value_float(key: *const c_char, default: c_float) -> c_float;

		pub virtual fn parm_count() -> c_int;
		pub virtual fn find_parm(key: *const c_char) -> c_int;
		pub virtual fn get_parm(index: c_int) -> *const c_char;

		pub virtual fn set_parm(index: c_int, new_parm: *const c_char);

		pub virtual fn parm_value_by_index(index: c_int, default: *const c_char);

		pub virtual fn has_parm(key: *const c_char) -> bool;

		pub virtual fn get_parms() -> *mut *const c_char;
	}
}

unsafe extern "C" {
	pub fn CommandLine_Tier0() -> RefMut<ICommandLine>;
}
