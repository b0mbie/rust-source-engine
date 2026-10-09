/// Internal implementation detail.
#[doc(hidden)]
pub use ::rse_cpp::transparent_wrapper as cpp_transparent_wrapper;

#[macro_export]
macro_rules! ptr_interface {
	{
		$(#[$attr:meta])*
		$vis:vis struct $Owned:ident($Wrapper:ident);
		Pointee = $Pointee:ty;
	} => {
		$crate::cpp_transparent_wrapper! {
			$(#[$attr])*
			$vis struct $Wrapper for $Pointee as ::core::stringify! { $Pointee };
		}
		#[repr(transparent)]
		$(#[$attr])*
		$vis struct $Owned {
			raw: ::core::ptr::NonNull<$Wrapper>,
		}
		impl ::core::ops::Deref for $Owned {
			type Target = $Wrapper;
			fn deref(&self) -> &Self::Target {
				unsafe { self.raw.as_ref() }
			}
		}
		impl ::core::ops::DerefMut for $Owned {
			fn deref_mut(&mut self) -> &mut Self::Target {
				unsafe { self.raw.as_mut() }
			}
		}
		unsafe impl $crate::FromRawInterface for $Owned
		where
			$Owned: $crate::Interface,
		{
			type Code = ();
			const INITIAL_CODE: Self::Code = ();

			unsafe fn from_raw_interface(raw_interface: $crate::RawInterface, _: Self::Code) -> Self {
				Self {
					raw: ::core::ptr::NonNull::cast(raw_interface),
				}
			}
			
			type Error = $crate::InterfaceError<Self>;
			fn convert_error(_: Self::Code) -> Self::Error {
				$crate::InterfaceError::new()
			}
		}
	};
}
