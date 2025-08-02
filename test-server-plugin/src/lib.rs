#![no_std]

use ::core::ffi::CStr;

use rse_server_plugin::{
	interface::{
		cppdef::{
			CreateInterfaceFn, RawInterface, ReturnCode,
		}, Interface, ToRawInterface
	},
	Plugin, PluginObject,
};

struct Test;
impl Plugin for Test {
	fn load(&mut self, app_system_factory: CreateInterfaceFn, game_server_factory: CreateInterfaceFn) -> bool {
		let _ = app_system_factory;
		let _ = game_server_factory;
		true
	}
	fn plugin_description(&mut self) -> &CStr {
		c"Test"
	}
}

static mut PLUGIN: PluginObject<Test> = PluginObject::new(Test);

struct SinglePlugin;
impl ::rse_server_plugin::interface::RawInterfaceFactory for SinglePlugin {
	#[allow(static_mut_refs)]
	unsafe fn create_interface_raw(
		&self, name: &CStr, return_code: Option<&mut ReturnCode>,
	) -> Option<RawInterface> {
		let result = if name == PluginObject::<Test>::IDENTIIFER {
			unsafe { Some(PLUGIN.to_raw_interface()) }
		} else {
			None
		};
		if let Some(return_code) = return_code {
			*return_code = if result.is_some() { ReturnCode::OK } else { ReturnCode::FAILED };
		}
		result
	}
}
impl ::rse_server_plugin::interface::DllInterfaceFactory for SinglePlugin {
	const INSTANCE: &Self = &SinglePlugin;
}

::rse_server_plugin::interface::dll_interface_factory!(SinglePlugin);
