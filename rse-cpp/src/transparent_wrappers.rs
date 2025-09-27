#[macro_export]
macro_rules! transparent_wrapper_impls {
	($name:ident for $target:ty as $target_name:literal) => {
		/// Returns a mutable reference to a value of type
		#[doc = concat!("[`", stringify!($name), "`]")]
		/// given a reference to the inner type.
		pub const fn from_mut(inner: &mut $target) -> &mut Self {
			unsafe { &mut *(inner as *mut $target as *mut Self) }
		}

		/// Returns an immutable reference to a value of type
		#[doc = concat!("[`", stringify!($name), "`]")]
		/// given a reference to the inner type.
		pub const fn from_ref(inner: &$target) -> &Self {
			unsafe { &*(inner as *const $target as *const Self) }
		}

		/// Returns a mutable reference to a value of type
		#[doc = concat!("[`", stringify!($name), "`]")]
		/// given a raw pointer.
		/// 
		/// See also [`from_mut`](Self::from_mut) for the safe version.
		/// 
		/// # Safety
		/// `ptr` must point to a valid, mutable
		#[doc = concat!("[`", $target_name, "`](", stringify!($target), ").")]
		pub const unsafe fn from_mut_ptr<'a>(ptr: *mut $target) -> &'a mut Self {
			unsafe { &mut *(ptr as *mut Self) }
		}

		/// Returns an immutable reference to a value of type
		#[doc = concat!("[`", stringify!($name), "`]")]
		/// given a raw pointer.
		/// 
		/// See also [`from_ref`](Self::from_ref) for the safe version.
		/// 
		/// # Safety
		/// `ptr` must point to a valid, immutable
		#[doc = concat!("[`", $target_name, "`](", stringify!($target), ").")]
		pub const unsafe fn from_ptr<'a>(ptr: *const $target) -> &'a Self {
			unsafe { &*(ptr as *const Self) }
		}

		/// Returns the raw mutable pointer given a mutable reference to a value of type
		#[doc = concat!("[`", stringify!($name), "`].")]
		pub const fn as_mut_ptr(&mut self) -> *mut $target {
			self as *mut Self as *mut $target
		}

		/// Returns the raw immutable pointer given an immutable reference to a value of type
		#[doc = concat!("[`", stringify!($name), "`].")]
		pub const fn as_ptr(&self) -> *const $target {
			self as *const Self as *const $target
		}

		$crate::transparent_wrapper_inner_impls!($name for $target as $target_name);
	};
}

#[macro_export]
macro_rules! transparent_wrapper_inner_impls {
	($name:ident for $target:ty as $target_name:literal) => {
		/// Returns an immutable reference to the inner C++ structure
		/// given an immutable reference to a value of type
		#[doc = concat!("[`", stringify!($name), "`].")]
		pub const fn as_inner(&self) -> &$target {
			&self.0
		}

		/// Returns a mutable reference to the inner C++ structure
		/// given a mutable reference to a value of type
		#[doc = concat!("[`", stringify!($name), "`].")]
		///
		/// See also [`as_inner`](Self::as_inner) for a safe version that doesn't permit mutation.
		/// 
		/// # Safety
		/// The structure contains public fields for highly-specific applications,
		/// which can be freely mutated and cause *Undefined Behavior*.
		pub const unsafe fn as_mut_inner(&mut self) -> &mut $target {
			&mut self.0
		}
	};
}

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
			$crate::transparent_wrapper_impls!($name for $target as $target_name);
		}

		impl<'a> From<&'a $target> for &'a $name {
			fn from(value: &'a $target) -> &'a $name {
				$name::from_ref(value)
			}
		}

		impl<'a> From<&'a mut $target> for &'a mut $name {
			fn from(value: &'a mut $target) -> &'a mut $name {
				$name::from_mut(value)
			}
		}
	};
}
