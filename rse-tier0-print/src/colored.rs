use crate::{
	PrintTo, DirectPrinter,
	ColorProvider,
	ComposeThen,
};

pub trait IntoColored<C: ColorProvider>: Sized {
	fn colored(self, color_provider: C) -> Colored<Self, C> {
		Colored {
			text: self,
			color_provider,
		}
	}
}
impl<T, C: ColorProvider> IntoColored<C> for T {}

#[derive(Clone, Copy)]
pub struct Colored<T, C> {
	pub text: T,
	pub color_provider: C,
}

impl<P: ?Sized, T, C> PrintTo<P> for Colored<T, C>
where
	P: DirectPrinter<T>,
	C: ColorProvider,
{
	fn print_to(self, printer: &P) {
		printer.direct_color_print(self.color_provider.get_color().as_ref(), self.text)
	}
}

impl<T, C> ComposeThen for Colored<T, C> {}
