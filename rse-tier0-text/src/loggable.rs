pub trait Loggable<L: ?Sized> {
    fn msg_to(self, logger: &L);
    fn warning_to(self, logger: &L);
    fn log_to(self, logger: &L);
}

impl<T, L: ?Sized> Loggable<L> for T
where
    L: crate::Logger<T>,
{
    fn log_to(self, logger: &L) {
        logger.log(self)
    }
    fn msg_to(self, logger: &L) {
        logger.msg(self)
    }
    fn warning_to(self, logger: &L) {
        logger.warning(self)
    }
}

pub trait ColorLoggerExt<T> {
	fn color_text(&self, t: T);
}
impl<T: ColorLoggable<L>, L: ?Sized> ColorLoggerExt<T> for L {
	fn color_text(&self, t: T) {
		t.color_msg_to(self)
	}
}

pub trait ColorLoggable<L: ?Sized> {
	fn color_msg_to(self, logger: &L);
}
