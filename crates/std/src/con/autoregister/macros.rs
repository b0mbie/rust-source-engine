#[doc(hidden)]
pub use ::ctor::declarative::ctor;

/// Adds registrable, `'static` ConVars and ConCommands
/// that implement [`AsStaticRegistrable`](super::AsStaticRegistrable)
/// to the global list of registrables
/// which would be registered in a call to [`register_all`](super::register_all).
#[macro_export]
macro_rules! autoregister {
	{} => {};
	{
		$registrable1:expr $(, $registrable:expr)* $(,)?
	} => {
		$crate::con::ctor! {
			#[ctor(anonymous)]
			fn autoregister() {
				$crate::con::add_to_register($registrable1);
				$(
					$crate::con::add_to_register($registrable);
				)*
			}
		}
	};
}

/// Adds `static` items to
/// a global list of ConVars and ConCommands
/// that implement [`AsStaticRegistrable`](super::AsStaticRegistrable)
/// to the global list of registrables
/// which would be registered in a call to [`register_all`](super::register_all).
#[macro_export]
macro_rules! autoregistered {
	// We need the `static` items to be parsed this way
	// so that `rust-analyzer` can provide live code completion,
	// and it is seemingly not currently possible to implement this with one `macro_rules!`.
	{
		$($items:tt)*
	} => {
		$($items)*
		$crate::autoregistered_impl! {
			@parse() $($items)*
		}
	};
}

/// Implementation detail of [`autoregistered!`](crate::autoregistered!).
#[doc(hidden)]
#[macro_export]
macro_rules! autoregistered_impl {
	{@parse($($name:ident)*)} => {
		$crate::autoregister! {
			$(&$name,)*
		}
	};

	{
		@parse($($name:ident)*)
		$(#[$attr:meta])*
		$vis:vis static $new_name:ident: $ty:ty = $init:expr;
		$($rest:tt)*
	} => {
		$crate::autoregistered_impl! {
			@parse($($name)* $new_name)
			$($rest)*
		}
	};

	{
		@parse($($name:ident)*)
		$($whatever:tt)*
	} => {
		::core::compile_error! {
			"only immutable `static` items are allowed in this context"
		}
	};
}
