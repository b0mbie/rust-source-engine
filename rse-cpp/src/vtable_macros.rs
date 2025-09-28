/// [`vtable!`](::cppdvt::vtable!) for Source 1.
/// 
/// The only meaningful difference at the time of writing is the assumption that
/// Source 1 does not use exceptions, and therefore call frames from it cannot be unwound.
#[macro_export]
macro_rules! vtable {
	{
		$(#[$vt_attr:meta])*
		$vt_vis:vis $vt_name:ident for $vt_this:ty {
			$(
				$(#[$fn_attr:meta])*
				$fn_vis:vis fn $fn_name:ident($($fn_param:tt)*) $(-> $fn_ret:ty)?;
			)*
		}
	} => {
		$(#[$vt_attr])*
		#[repr(C)]
		$vt_vis struct $vt_name {
			$(
				#[cfg(all(windows, target_arch = "x86"))]
				$(#[$fn_attr])*
				$fn_vis $fn_name:
					unsafe extern "thiscall" fn (
						this: $vt_this, $($fn_param)*
					) $(-> $fn_ret)?,
				#[cfg(not(all(windows, target_arch = "x86")))]
				$(#[$fn_attr])*
				$fn_vis $fn_name:
					unsafe extern "C" fn (
						this: $vt_this, $($fn_param)*
					) $(-> $fn_ret)?,
			)*
		}
	};

	(
		$(#[$vt_attr:meta])*
		$vt_vis:vis $vt_name:ident {
			$(
				$(#[$fn_attr:meta])*
				$fn_vis:vis fn $fn_name:ident($($fn_param:tt)*) $(-> $fn_ret:ty)?;
			)*
		}
	) => {
		$crate::vtable! {
			$(#[$vt_attr])*
			$vt_vis $vt_name for $crate::VtObjectPtr<$vt_name> {
				$(
					$(#[$fn_attr])*
					$fn_vis fn $fn_name($($fn_param)*) $(-> $fn_ret)?;
				)*
			}
		}
	};
}

#[macro_export]
macro_rules! vtable_methods {
	{
		$this:ident : $this_ty:ty;
		$(
			$(#[$attr:meta])*
			fn $name:ident($($param:tt)*) $(-> $return:ty)? {
				$($body:tt)*
			}
		)*
	} => {
		$(
			#[cfg(all(windows, target_arch = "x86"))]
			$(#[$attr])*
			unsafe extern "thiscall" fn $name($this: $this_ty, $($param)*) $(-> $return)? {
				$($body)*
			}

			#[cfg(not(all(windows, target_arch = "x86")))]
			$(#[$attr])*
			unsafe extern "C" fn $name($this: $this_ty, $($param)*) $(-> $return)? {
				$($body)*
			}
		)*
	};
}
