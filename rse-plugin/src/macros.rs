/// Generates a [`CStr`](::core::ffi::CStr) that can be used as a description for a server plugin
/// by concatenating the crate's name and version.
/// 
/// For example, given the `plugin` crate of version `0.1.0`, this macro will generate the C string `plugin 0.1.0`.
#[macro_export]
macro_rules! plugin_description {
	() => {
		unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(
			::core::concat!(::core::env!("CARGO_PKG_NAME"), ' ', ::core::env!("CARGO_PKG_VERSION"), "\0").as_bytes()
		) }
	};
}

/// Exports a [`StaticPlugin`](crate::StaticPlugin),
/// given the input `<visibility> <name> = <plugin type>`,
/// putting the plugin instance into a `static mut` item.
#[macro_export]
macro_rules! export_static_plugin_as {
	($vis:vis $name:ident = $ty:ty) => {
		$vis static mut $name: $crate::PluginObject<$ty> =
			$crate::PluginObject::new(<$ty as $crate::StaticPlugin>::NOT_LOADED);

		const _: () = {
			struct ExportedPlugin;
			impl $crate::interface::RawInterfaceFactory for ExportedPlugin {
				#[allow(static_mut_refs)]
				unsafe fn create_interface_raw(
					&self, name: &::core::ffi::CStr,
					return_code: ::core::option::Option<&mut $crate::interface::cppdef::ReturnCode>,
				) -> ::core::option::Option<$crate::interface::cppdef::RawInterface> {
					let result = if name == <$crate::PluginObject<$ty> as $crate::interface::Interface>::IDENTIFIER {
						unsafe { Some($crate::interface::ToRawInterface::to_raw_interface(&mut PLUGIN)) }
					} else {
						None
					};
					if let ::core::option::Option::Some(return_code) = return_code {
						*return_code = if result.is_some() {
							$crate::interface::cppdef::ReturnCode::OK
						} else {
							$crate::interface::cppdef::ReturnCode::FAILED
						};
					}
					result
				}
			}
			impl $crate::interface::DllInterfaceFactory for ExportedPlugin {
				const INSTANCE: &Self = &ExportedPlugin;
			}

			$crate::interface::dll_interface_factory!(ExportedPlugin);
		};
	};
}

/// Exports a [`StaticPlugin`](crate::StaticPlugin) given the plugin type.
#[macro_export]
macro_rules! export_static_plugin {
	($ty:ty) => {
		const _: () = {
			$crate::export_static_plugin_as!(PLUGIN = $ty);
		};
	};
}

/// Exports a [`LoadablePlugin`](crate::LoadablePlugin),
/// given the input `<visibility> <name> = <plugin type>`,
/// putting the plugin instance into a `static mut` item.
#[macro_export]
macro_rules! export_loadable_plugin_as {
	($vis:vis $name:ident = $ty:ty) => {
		$crate::export_static_plugin_as!($vis $name = $crate::PluginLoader<$ty>);
	};
}

/// Exports a [`LoadablePlugin`](crate::LoadablePlugin) given the plugin type.
#[macro_export]
macro_rules! export_loadable_plugin {
	($ty:ty) => {
		$crate::export_static_plugin!($crate::PluginLoader<$ty>);
	};
}
