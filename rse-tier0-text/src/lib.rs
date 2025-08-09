pub use ::rse_tier0 as tier0;

pub use tier0::Color;
use tier0::{
	Logger, ColorLogger,
};

mod macros;

mod color_provider;
pub use color_provider::*;
mod loggable;
pub use loggable::*;

pub mod prelude {
	pub use crate::{
		Color,
		ColorLoggerExt,
		IntoThen, IntoColored,
		msg,
	};
}

pub trait IntoColored<C: ColorProvider, L: ?Sized>: Sized + Loggable<L> {
	fn colored(self, color_provider: C) -> Colored<Self, C> {
		Colored {
			text: self,
			color_provider,
		}
	}
}
impl<T: Loggable<L>, C: ColorProvider, L: ?Sized> IntoColored<C, L> for T {}

#[derive(Clone, Copy)]
pub struct Colored<T, C> {
	pub text: T,
	pub color_provider: C,
}
impl<T, C> IntoThen for Colored<T, C> {}
impl<L: ?Sized, T, C> ColorLoggable<L> for Colored<T, C>
where
	L: ColorLogger<T>,
	C: ColorProvider,
{
	fn color_msg_to(self, logger: &L) {
		logger.color_msg(self.color_provider.get_color().as_ref(), self.text)
	}
}

pub trait IntoThen: Sized {
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
impl<A, B> IntoThen for Then<A, B> {}
impl<L: ?Sized, A, B> ColorLoggable<L> for Then<A, B>
where
	A: ColorLoggable<L>,
	B: ColorLoggable<L>,
{
	fn color_msg_to(self, logger: &L) {
		self.first.color_msg_to(logger);
		self.second.color_msg_to(logger)
	}
}
