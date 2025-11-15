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
/// given the input `<visibility> static mut <name>: <plugin type> = <initializer>;`,
/// putting the plugin instance into a `static mut` item.
#[macro_export]
macro_rules! export_static_plugin_as {
	{
		$(#[$attr:meta])*
		$vis:vis static mut $name:ident: $ty:ty = $init:expr;
	} => {
		$(#[$attr])*
		$vis static mut $name: $crate::PluginObject<$ty> = $crate::PluginObject::new($init);

		const _: () = {
			struct ExportedPlugin;
			impl $crate::interface::RawInterfaceFactory for ExportedPlugin {
				#[allow(static_mut_refs)]
				unsafe fn create_interface_raw(
					&self, name: &::core::ffi::CStr,
					return_code: ::core::option::Option<&mut $crate::interface::cppdef::ReturnCode>,
				) -> ::core::option::Option<$crate::interface::cppdef::RawInterface> {
					let result = if name == <$crate::PluginObject<$ty> as $crate::interface::Interface>::IDENTIFIER {
						unsafe { Some($crate::interface::ToRawInterface::to_raw_interface(&mut $name)) }
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

			$crate::interface::export_interface_factory!(ExportedPlugin = ExportedPlugin);
		};
	};
}

/// Exports a [`StaticPlugin`](crate::StaticPlugin)
/// given the input `<plugin type> = <initializer>`.
#[macro_export]
macro_rules! export_static_plugin {
	($ty:ty = $init:expr) => {
		const _: () = {
			$crate::export_static_plugin_as! {
				static mut EXPORTED_STATIC_PLUGIN: $ty = $init;
			}
		};
	};
}

/// Exports a [`LoadablePlugin`](crate::LoadablePlugin),
/// given the input `<visibility> static mut <name>: <plugin type>;`,
/// putting the plugin instance into a `static mut` item.
#[macro_export]
macro_rules! export_loadable_plugin_as {
	{
		$(#[$attr:meta])*
		$vis:vis static mut $name:ident: $ty:ty;
	} => {
		$crate::export_static_plugin_as! {
			$vis static mut $name: $crate::PluginLoader<$ty> = $crate::PluginLoader::new();
		}
	};
}

/// Exports a [`LoadablePlugin`](crate::LoadablePlugin) given the plugin type.
#[macro_export]
macro_rules! export_loadable_plugin {
	($ty:ty) => {
		$crate::export_static_plugin!($crate::PluginLoader<$ty> = $crate::PluginLoader::new());
	};
}
