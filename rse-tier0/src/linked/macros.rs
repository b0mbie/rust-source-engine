#[macro_export]
macro_rules! dev_msg {
	() => {
		$crate::msgln!($crate::linked::spew::dev())
	};

	($($arg:tt)+) => {
		$crate::msgln!($crate::linked::spew::dev(), $($arg)+)
	};
}

#[macro_export]
macro_rules! dev_warn {
	() => {
		$crate::warnln!($crate::linked::spew::dev())
	};

	($($arg:tt)+) => {
		$crate::warnln!($crate::linked::spew::dev(), $($arg)+)
	};
}

#[macro_export]
macro_rules! con_msg {
	() => {
		$crate::msgln!($crate::linked::spew::con())
	};

	($($arg:tt)+) => {
		$crate::msgln!($crate::linked::spew::con(), $($arg)+)
	};
}

#[macro_export]
macro_rules! con_warn {
	() => {
		$crate::warnln!($crate::linked::spew::con())
	};

	($($arg:tt)+) => {
		$crate::warnln!($crate::linked::spew::con(), $($arg)+)
	};
}

#[macro_export]
macro_rules! con_color_msg {
	($($arg:tt)*) => {{
		let con = $crate::linked::spew::con();
		$crate::color_msg!(con => $($arg)*);
		$crate::msgln!(con);
	}};
}
