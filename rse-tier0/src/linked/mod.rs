pub mod cppdef;

mod macros;

mod errors;
mod spew;
pub use spew::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkedTier0;

macro_rules! call_printf {
	($t:expr => $func:expr $(, $($arg:tt)+)?) => {{
		use ::printf::{
			IntoFormattable, Formattable, IntoPrecision,
		};
		let fmt = IntoFormattable::into_formattable($t);
		let fmt_str = fmt.format_string();
		if let Ok(precision) = fmt.precision().into_precision() {
			$func($($($arg)+,)? fmt_str.as_ref().as_ptr(), precision, Formattable::into_c_type(fmt))
		} else {
			$func($($($arg)+,)? fmt_str.as_ref().as_ptr(), Formattable::into_c_type(fmt))
		}
	}};
}
pub(crate) use call_printf;
