#[macro_export]
macro_rules! dev_msg {
	() => {
		$crate::msgln!($crate::linked::dev())
	};

	($($arg:tt)+) => {
		$crate::msgln!($crate::linked::dev(), $($arg)+)
	};
}

#[macro_export]
macro_rules! dev_warn {
	() => {
		$crate::warnln!($crate::linked::dev())
	};

	($($arg:tt)+) => {
		$crate::warnln!($crate::linked::dev(), $($arg)+)
	};
}

#[macro_export]
macro_rules! con_msg {
	() => {
		$crate::msgln!($crate::linked::con())
	};

	($($arg:tt)+) => {
		$crate::msgln!($crate::linked::con(), $($arg)+)
	};
}

#[macro_export]
macro_rules! con_warn {
	() => {
		$crate::warnln!($crate::linked::con())
	};

	($($arg:tt)+) => {
		$crate::warnln!($crate::linked::con(), $($arg)+)
	};
}

#[macro_export]
macro_rules! con_color_msg {
	($($arg:tt)+) => {
		$crate::color_msgln!($crate::linked::con(), $($arg)+)
	};
}
