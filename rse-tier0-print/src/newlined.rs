use crate::{
	Plain,
	ComposeThen, PrintTo,
};

pub trait ComposeNewlined: Sized {
	fn newlined(self) -> Newlined<Self> {
		Newlined(self)
	}
}
impl<T> ComposeNewlined for T {}

#[derive(Clone, Copy)]
pub struct Newlined<T>(pub T);

impl<P: ?Sized, T> PrintTo<P> for Newlined<T>
where
	T: PrintTo<P>,
	Plain<&'static [u8]>: PrintTo<P>,
{
	fn print_to(self, printer: &P) {
		self.0.print_to(printer);
		// TODO: Is there a better way to express the newline?
		Plain(b"\n".as_ref()).print_to(printer)
	}
}

impl<T> ComposeThen for Newlined<T> {}