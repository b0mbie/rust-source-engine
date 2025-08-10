use crate::{
	ConstColor,
	PrintTo, DirectPrinter,
	ColorProvider,
	ComposeThen,
};

pub trait IntoColored: Sized {
	fn colored<C: ColorProvider>(self, color_provider: C) -> Colored<Self, C> {
		Colored {
			text: self,
			color_provider,
		}
	}

	fn rgb<const R: u8, const G: u8, const B: u8>(self) -> Colored<Self, ConstColor<R, G, B>> {
		Colored {
			text: self,
			color_provider: ConstColor,
		}
	}

	fn rgba<const R: u8, const G: u8, const B: u8, const A: u8>(self) -> Colored<Self, ConstColor<R, G, B, A>> {
		Colored {
			text: self,
			color_provider: ConstColor,
		}
	}
}
impl<T> IntoColored for T {}

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
