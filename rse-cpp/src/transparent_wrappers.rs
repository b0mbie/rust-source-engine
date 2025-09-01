#[macro_export]
macro_rules! transparent_wrapper {
	{
		$(#[$attr:meta])*
		$vis:vis struct $name:ident for $target:ty as $target_name:literal;
	} => {
		#[repr(transparent)]
		$(#[$attr])*
		$vis struct $name($target);

		impl $name {
			/// Returns a mutable reference to a value of type
			#[doc = concat!("[`", stringify!($name), "`]")]
			/// given a raw pointer.
			/// 
			/// # Safety
			/// `ptr` must point to a valid, mutable
			#[doc = concat!("[`", $target_name, "`](", stringify!($target), ").")]
			pub const unsafe fn from_ptr_mut<'a>(ptr: *mut $target) -> &'a mut Self {
				unsafe { &mut *(ptr as *mut Self) }
			}

			/// Returns an immutable reference to a value of type
			#[doc = concat!("[`", stringify!($name), "`]")]
			/// given a raw pointer.
			/// 
			/// # Safety
			/// `ptr` must point to a valid, immutable
			#[doc = concat!("[`", $target_name, "`](", stringify!($target), ").")]
			pub const unsafe fn from_ptr<'a>(ptr: *const $target) -> &'a Self {
				unsafe { &*(ptr as *const Self) }
			}

			/// Returns the raw mutable pointer given a mutable reference to a value of type
			#[doc = concat!("[`", stringify!($name), "`].")]
			pub const fn as_ptr_mut(&mut self) -> *mut $target {
				self as *mut Self as *mut $target
			}

			/// Returns the raw immutable pointer given an immutable reference to a value of type
			#[doc = concat!("[`", stringify!($name), "`].")]
			pub const fn as_ptr(&self) -> *const $target {
				self as *const Self as *const $target
			}
		}
	};
}
