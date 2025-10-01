#[macro_export]
macro_rules! con_command {
	{
		dll_identifier = $dll_identifier:expr;
		help = $help:expr;

		$(#[$attr:meta])*
		$vis:vis $name:ident($invocation:pat) $dispatch:block
	} => {
		$crate::concat_idents::concat_idents!(
			name_impl = $name, _impl
			{
				#[allow(non_upper_case_globals, private_interfaces)]
				$(#[$attr])*
				$vis static mut $name: $crate::command::low::ConCommandObject<'_, name_impl> =
					$crate::command::con_command(name_impl);

				#[allow(dead_code, non_camel_case_types)]
				#[doc(hidden)]
				struct name_impl;
				unsafe impl $crate::command::Command for name_impl {
					const NAME: &::core::ffi::CStr = unsafe {
						::core::ffi::CStr::from_bytes_with_nul_unchecked(
							::core::concat!(::core::stringify!($name), '\0').as_bytes()
						)
					};
					const HELP: Option<&::core::ffi::CStr> = Some({
						let help: &::core::ffi::CStr = $help;
						help
					});
					fn dispatch(&mut self, $invocation: &$crate::Invocation) $dispatch

					fn dll_identifier(&mut self) -> $crate::cppdef::CvarDllIdentifier {
						$dll_identifier
					}
				}
			}
		);
	};
}
