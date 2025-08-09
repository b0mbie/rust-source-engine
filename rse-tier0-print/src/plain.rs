use crate::{
	DirectPrinter, PrintTo,
	ComposeThen,
};

pub trait IntoPlain: Into<Plain<Self>> {
	fn plain(self) -> Plain<Self> {
		self.into()
	}
}
impl<T: Into<Plain<Self>>> IntoPlain for T {}

#[derive(Clone, Copy)]
pub struct Plain<T>(pub T);
impl<T> From<T> for Plain<T> {
	fn from(value: T) -> Self {
		Self(value)
	}
}

impl<T, P: ?Sized> PrintTo<P> for Plain<T>
where
	P: DirectPrinter<T>,
{
	fn print_to(self, printer: &P) {
		printer.direct_print(self.0)
	}
}

impl<T> ComposeThen for Plain<T> {}
