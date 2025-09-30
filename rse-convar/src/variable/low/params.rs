use ::core::ffi::{
	CStr, c_float,
};

#[derive(Default, Debug, Clone, Copy)]
pub struct ConVarParams<'a> {
	pub name: &'a CStr,
	pub default: &'a CStr,
	pub help: Option<&'a CStr>,
	pub min: Option<c_float>,
	pub max: Option<c_float>,
	pub comp_min: Option<c_float>,
	pub comp_max: Option<c_float>,
}

impl<'a> ConVarParams<'a> {
	pub const fn simple(name: &'a CStr, default: &'a CStr) -> Self {
		Self {
			name, default,
			help: None,
			min: None, max: None,
			comp_min: None, comp_max: None,
		}
	}
}
