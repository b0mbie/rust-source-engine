use ::core::{
	ffi::{
		CStr, c_char, c_int,
	},
	mem::size_of,
	slice::from_raw_parts_mut as slice_from_raw_parts_mut,
};
use ::rse_cpp::{
	RefConst, VtObjectMut, new_vtable_self, this_to_self,
};
use ::rse_game::{
	cppdef::entities::Edict,
	Command, ServerEdict,
};
use ::rse_game_interfaces::InterfaceFactories;
use ::rse_interface::{
	CreateInterfaceFn, RawInterface,
	Interface, ToRawInterface,
};

use crate::{
	cppdef::{
		ClientIndex, PluginResult, QueryCvarCookie, QueryCvarValueStatus, ServerPluginCallbacksVt,
		INTERFACEVERSION_ISERVERPLUGINCALLBACKS,
	},
	ClientConnect, Plugin, RejectReason,
};

pub trait StaticPlugin: Plugin {
	const NOT_LOADED: Self;
	fn load(&mut self, factories: InterfaceFactories<'_>) -> bool;
	fn unload(&mut self);
}

#[repr(C)]
pub struct PluginObject<T> {
	vtable: *mut ServerPluginCallbacksVt,
	inner: T,
}

unsafe impl<T> Interface for PluginObject<T> {
	const IDENTIFIER: &CStr = INTERFACEVERSION_ISERVERPLUGINCALLBACKS;
}
impl<T: StaticPlugin> ToRawInterface for PluginObject<T> {
	unsafe fn to_raw_interface(&mut self) -> RawInterface {
		unsafe { RawInterface::new_unchecked(self as *mut Self as *mut _) }
	}
}

impl<T> Default for PluginObject<T>
where
	T: StaticPlugin + Default,
{
	fn default() -> Self {
		Self::new(T::default())
	}
}

impl<T> PluginObject<T>
where
	T: StaticPlugin,
{
	pub const fn new(inner: T) -> Self {
		Self {
			vtable: Self::VTABLE as *const _ as *mut _,
			inner,
		}
	}

	pub const fn as_inner(&self) -> &T {
		&self.inner
	}

	pub const fn as_inner_mut(&mut self) -> &mut T {
		&mut self.inner
	}

	const VTABLE: &ServerPluginCallbacksVt = &new_vtable_self!(ServerPluginCallbacksVt {
		load,
		unload,
		pause,
		unpause,
		get_plugin_description,
		level_init,
		server_activate,
		game_frame,
		level_shutdown,
		client_active,
		client_disconnect,
		client_put_in_server,
		set_command_client,
		client_settings_changed,
		client_connect,
		client_command,
		network_id_validated,
		on_query_cvar_value_finished,
		on_edict_allocated,
		on_edict_freed
	});

	::rse_cpp::vtable_methods! {
		this: VtObjectMut<ServerPluginCallbacksVt>;
		fn load(interface_factory: CreateInterfaceFn, game_server_factory: CreateInterfaceFn) -> bool {
			let factories = InterfaceFactories::new(interface_factory, game_server_factory);
			this_to_self!(mut this).inner.load(factories)
		}
		fn unload() {
			this_to_self!(mut this).inner.unload();
		}
		fn pause() {
			this_to_self!(mut this).inner.pause()
		}
		fn unpause() {
			this_to_self!(mut this).inner.unpause()
		}
		fn get_plugin_description() -> *const c_char {
			this_to_self!(mut this).inner.description().as_ptr()
		}
		fn level_init(map_name: *const c_char) {
			let map_name = unsafe { CStr::from_ptr(map_name) };
			this_to_self!(mut this).inner.level_init(map_name)
		}
		fn server_activate(edict_list: *mut Edict, edict_count: c_int, client_max: ClientIndex) {
			// SAFETY: `Edict` is a transparent wrapper around `Edict`.
			let edicts = unsafe {
				slice_from_raw_parts_mut(edict_list as *mut ServerEdict, edict_count as _)
			};
			this_to_self!(mut this).inner.server_activate(edicts, client_max)
		}
		fn game_frame(simulating: bool) {
			this_to_self!(mut this).inner.game_frame(simulating)
		}
		fn level_shutdown() {
			this_to_self!(mut this).inner.level_shutdown()
		}
		fn client_active(entity: *mut Edict) {
			let entity = unsafe { ServerEdict::from_c_edict_mut(&mut *entity) };
			this_to_self!(mut this).inner.client_active(entity)
		}
		fn client_disconnect(entity: *mut Edict) {
			let entity = unsafe { ServerEdict::from_c_edict_mut(&mut *entity) };
			this_to_self!(mut this).inner.client_disconnect(entity)
		}
		fn client_put_in_server(entity: *mut Edict, player_name: *const c_char) {
			let entity = unsafe { ServerEdict::from_c_edict_mut(&mut *entity) };
			let player_name = unsafe { CStr::from_ptr(player_name) };
			this_to_self!(mut this).inner.client_put_in_server(entity, player_name)
	}
		fn set_command_client(index: c_int) {
			this_to_self!(mut this).inner.set_command_client(index)
		}
		fn client_settings_changed(edict: *mut Edict) {
			let edict = unsafe { ServerEdict::from_c_edict_mut(&mut *edict) };
			this_to_self!(mut this).inner.client_settings_changed(edict)
		}
		fn client_connect(
			out_allow_connect: *mut bool,
			entity: *mut Edict,
			name: *const c_char, address: *const c_char,
			out_reject: *mut c_char, out_reject_len: c_int,
		) -> PluginResult {
			let out_allow_connect = unsafe { &mut *out_allow_connect };
			let entity = unsafe { ServerEdict::from_c_edict_mut(&mut *entity) };
			let name = unsafe { CStr::from_ptr(name) };
			let address = unsafe { CStr::from_ptr(address) };
			let buffer = unsafe {
				slice_from_raw_parts_mut(
					out_reject as *mut u8,
					// For correctness, we may need to fix up the length,
					// since it's based on `c_char` and not `u8`.
					(out_reject_len as usize) * size_of::<c_char>() / size_of::<u8>(),
				)
			};
			let reject_reason = unsafe { RejectReason::new_unchecked(buffer) };
			let result = this_to_self!(mut this).inner.client_connect(entity, name, address, reject_reason);
			match result {
				ClientConnect::Continue => PluginResult::Continue,
				ClientConnect::Allow => {
					*out_allow_connect = true;
					PluginResult::Override
				}
				ClientConnect::AllowStop => {
					*out_allow_connect = true;
					PluginResult::Stop
				}
				ClientConnect::Reject => {
					*out_allow_connect = false;
					PluginResult::Override
				}
				ClientConnect::RejectStop => {
					*out_allow_connect = false;
					PluginResult::Stop
				}
			}
		}
		fn client_command(entity: *mut Edict, args: RefConst<Command>) -> PluginResult {
			let entity = unsafe { ServerEdict::from_c_edict_mut(&mut *entity) };
			let args = unsafe { args.as_ref() };
			this_to_self!(mut this).inner.client_command(entity, args)
		}
		fn network_id_validated(user_name: *const c_char, network_id: *const c_char) -> PluginResult {
			let user_name = unsafe { CStr::from_ptr(user_name) };
			let network_id = unsafe { CStr::from_ptr(network_id) };
			this_to_self!(mut this).inner.network_id_validated(user_name, network_id)
		}
		fn on_query_cvar_value_finished(
			cookie: QueryCvarCookie,
			player_entity: *mut Edict,
			status: QueryCvarValueStatus,
			cvar_name: *const c_char, cvar_value: *const c_char,
		) {
			let player_entity = unsafe { ServerEdict::from_c_edict_mut(&mut *player_entity) };
			let cvar_name = unsafe { CStr::from_ptr(cvar_name) };
			let cvar_value = unsafe { CStr::from_ptr(cvar_value) };
			this_to_self!(mut this).inner.on_query_cvar_value_finished(
				cookie, player_entity, status,
				cvar_name, cvar_value,
			)
		}
		fn on_edict_allocated(edict: *mut Edict) {
			let edict = unsafe { ServerEdict::from_c_edict_mut(&mut *edict) };
			this_to_self!(mut this).inner.on_edict_allocated(edict)
		}
		fn on_edict_freed(edict: *const Edict) {
			let edict = unsafe { ServerEdict::from_c_edict(&*edict) };
			this_to_self!(mut this).inner.on_edict_freed(edict)
		}
	}
}
