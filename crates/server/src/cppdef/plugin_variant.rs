use ::core::ffi::{
	c_char, c_float, c_int,
};
use ::rse_cpp::class;
use ::rse_math::color32;
use ::rse_shared::cppdef::{
	entities::edict_t,
	fieldtype_t,
};

class! {
	pub pluginvariant {
		pub val: PluginVariantVal;
		pub e_val: *mut edict_t;
		pub isz_val: [c_char; 1024];
		pub field_type: fieldtype_t;
	}
}

#[repr(C)]
pub union PluginVariantVal {
	pub b_val: bool,
	pub i_val: c_int,
	pub fl_val: c_float,
	pub vec_val: [c_float; 3],
	pub rgba_val: color32,
}
