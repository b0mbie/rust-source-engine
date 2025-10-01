/// Generates a [`ConVarValue`](crate::variable::low::ConVarValue) expression from a literal value.
/// 
/// # Examples
/// ```
/// # use rse_convar::{variable::low::ConVarValue, cvar_value};
/// assert_eq!(
///     cvar_value!(c"how"),
///     ConVarValue {
///         c_str: c"how",
///         float: 0.0,
///         int: 0,
///     },
/// );
/// assert_eq!(
///     cvar_value!(s c"how"),
///     ConVarValue {
///         c_str: c"how",
///         float: 0.0,
///         int: 0,
///     },
/// );
/// assert_eq!(
///     cvar_value!(f 1.234),
///     ConVarValue {
///         c_str: c"1.234",
///         float: 1.234,
///         int: 1,
///     },
/// );
/// assert_eq!(
///     cvar_value!(i 512),
///     ConVarValue {
///         c_str: c"512",
///         float: 512.0,
///         int: 512,
///     },
/// );
/// ```
#[macro_export]
macro_rules! cvar_value {
	($c_str:expr $(,)?) => {
		$crate::variable::low::ConVarValue {
			c_str: $c_str,
			float: 0.0,
			int: 0,
		}
	};
	(s $value:expr $(,)?) => {
		$crate::variable::low::ConVarValue {
			c_str: $value,
			float: 0.0,
			int: 0,
		}
	};
	(f $value:literal $(,)?) => {{
		let float = $value;
		$crate::variable::low::ConVarValue {
			c_str: unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(
				::core::concat!(::core::stringify!($value), '\0').as_bytes()
			) },
			float,
			int: float as _,
		}
	}};
	(i $value:literal $(,)?) => {{
		let int = $value;
		$crate::variable::low::ConVarValue {
			c_str: unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(
				::core::concat!(::core::stringify!($value), '\0').as_bytes()
			) },
			int,
			float: int as _,
		}
	}};
}
