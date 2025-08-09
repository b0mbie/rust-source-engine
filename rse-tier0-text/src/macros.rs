#[macro_export]
macro_rules! msg {
	($logger:expr, $text:expr) => {{
		$crate::LoggerExt::msg(&$logger, $text);
	}};
}
