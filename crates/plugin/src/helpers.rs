use ::core::ffi::CStr;
use ::rse_cpp::virtual_call;
use ::rse_shared::KeyValues;

use crate::{
	cppdef::{
		IServerPluginHelpers, DialogType,
		IServerPluginCallbacks,
		QueryCvarCookie_t, InvalidQueryCvarCookie,
	},
	ServerEdict,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct QueryCvarCookie(pub QueryCvarCookie_t);
impl QueryCvarCookie {
	pub const INVALID: Self = Self(InvalidQueryCvarCookie);

	pub const fn is_valid(&self) -> bool {
		self.0 != Self::INVALID.0
	}

	pub const fn is_invalid(&self) -> bool {
		self.0 == Self::INVALID.0
	}
}

impl ServerPluginHelpers {
	pub unsafe fn create_message(
		&mut self,
		entity: &mut ServerEdict,
		ty: DialogType, data: &mut KeyValues,
		plugin: *mut IServerPluginCallbacks,
	) {
		unsafe { virtual_call!(mut self.as_mut_inner() => create_message(
			entity.as_mut_ptr(), ty, data.as_mut_ptr(), plugin,
		)) }
	}

	pub fn client_command(&mut self, entity: &mut ServerEdict, cmd: &CStr) {
		unsafe { virtual_call!(mut self.as_mut_inner() => client_command(entity.as_mut_ptr(), cmd.as_ptr())) }
	}

	pub fn start_query_cvar_value(&mut self, entity: &mut ServerEdict, name: &CStr) -> QueryCvarCookie {
		let raw = unsafe { virtual_call!(mut self.as_mut_inner() => start_query_cvar_value(
			entity.as_mut_ptr(), name.as_ptr(),
		)) };
		QueryCvarCookie(raw)
	}
}

::rse_cpp::transparent_wrapper! {
	pub struct ServerPluginHelpers for IServerPluginHelpers as "IServerPluginHelpers";
}
