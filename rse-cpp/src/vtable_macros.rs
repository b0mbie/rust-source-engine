#[macro_export]
macro_rules! vtable_methods {
	{
		$this:ident : $this_ty:ty;
		$(
			$(#[$attr:meta])*
			fn $name:ident($($param:tt)*) $(-> $return:ty)? $body:block
		)*
	} => {
		$(
			$crate::virtual_fn! {
				$(#[$attr])*
				fn $name($this: $this_ty, $($param)*) $(-> $return)? $body
			}
		)*
	};
}

#[macro_export]
macro_rules! this_to_pin_self {
	($($arg:tt)*) => {
		::core::pin::Pin::new_unchecked($crate::this_to_self!($($arg)*))
	};
}
