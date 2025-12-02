#[macro_export]
macro_rules! export_plugin_as {
	{
		$(#[$attr:meta])*
		$vis:vis $name:ident: $ty:ty;
	} => {
		$crate::rse_plugin::export_static_plugin_as! {
			$(#[$attr])*
			$vis static mut $name: $crate::plugin::Adapter<$ty> = $crate::plugin::Adapter::new();
		}
	};
}

#[macro_export]
macro_rules! export_plugin {
	($ty:ty) => {
		$crate::rse_plugin::export_static_plugin!($crate::plugin::Adapter<$ty> = $crate::plugin::Adapter::new());
	};
}
