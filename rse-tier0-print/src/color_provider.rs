use crate::Color;

pub trait ColorProvider {
	type Color<'a>: AsRef<Color> where Self: 'a;
	fn get_color(&self) -> Self::Color<'_>;
}
impl<C: ColorProvider> ColorProvider for &C {
	type Color<'a> = C::Color<'a> where Self: 'a;
	fn get_color(&self) -> Self::Color<'_> {
		C::get_color(*self)
	}
}

impl ColorProvider for Color {
	type Color<'a> = RefAsRef<'a> where Self: 'a;
	fn get_color(&self) -> Self::Color<'_> {
		RefAsRef(self)
	}
}

impl ColorProvider for (u8, u8, u8) {
	type Color<'a> = ValueAsRef where Self: 'a;
	fn get_color(&self) -> Self::Color<'_> {
		ValueAsRef(Color {
			r: self.0,
			g: self.1,
			b: self.2,
			a: 255,
		})
	}
}

impl ColorProvider for (u8, u8, u8, u8) {
	type Color<'a> = ValueAsRef where Self: 'a;
	fn get_color(&self) -> Self::Color<'_> {
		ValueAsRef(Color {
			r: self.0,
			g: self.1,
			b: self.2,
			a: self.3,
		})
	}
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstColor<const R: u8, const G: u8, const B: u8, const A: u8 = 255>;
impl<const R: u8, const G: u8, const B: u8, const A: u8> ColorProvider for ConstColor<R, G, B, A> {
	type Color<'a> = RefAsRef<'a> where Self: 'a;
	fn get_color(&self) -> Self::Color<'_> {
		RefAsRef(&Color {
			r: R,
			g: G,
			b: B,
			a: A,
		})
	}
}
macro_rules! const_colors {
	{
		$(
			$(#[$attr:meta])*
			$name:ident = ($r:expr, $g:expr, $b:expr, $a:expr);
		)*
	} => {
		$(
			impl ConstColor<$r, $g, $b, $a> {
				$(#[$attr])*
				pub const $name: Self = Self;
			}
		)*
	};
}
const_colors! {
	RED = (255, 0, 0, 255);
	GREEN = (0, 255, 0, 255);
	BLUE = (0, 0, 255, 255);
}

#[repr(transparent)]
pub struct ValueAsRef(pub Color);
impl AsRef<Color> for ValueAsRef {
	fn as_ref(&self) -> &Color {
		&self.0
	}
}

#[repr(transparent)]
pub struct RefAsRef<'a>(pub &'a Color);
impl AsRef<Color> for RefAsRef<'_> {
	fn as_ref(&self) -> &Color {
		self.0
	}
}
