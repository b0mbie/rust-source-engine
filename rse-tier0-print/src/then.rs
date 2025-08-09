use crate::PrintTo;

pub trait ComposeThen: Sized {
	fn then<T>(self, then: T) -> Then<Self, T> {
		Then {
			first: self,
			second: then,
		}
	}
}

#[derive(Clone, Copy)]
pub struct Then<A, B> {
	pub first: A,
	pub second: B,
}

impl<P: ?Sized, A, B> PrintTo<P> for Then<A, B>
where
	A: PrintTo<P>,
	B: PrintTo<P>,
{
	fn print_to(self, printer: &P) {
		self.first.print_to(printer);
		self.second.print_to(printer)
	}
}

impl<A, B> ComposeThen for Then<A, B> {}
