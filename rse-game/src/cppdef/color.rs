pub type ColorComponent = ::core::ffi::c_uchar;

// TODO: Move somewhere else?
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Color {
	// These correctly correspond to `[ColorComponent; 4]`.
	pub r: ColorComponent,
	pub g: ColorComponent,
	pub b: ColorComponent,
	pub a: ColorComponent,
}

impl Color {
	pub const fn rgb(r: ColorComponent, g: ColorComponent, b: ColorComponent) -> Self {
		Self {
			r, g, b,
			a: ColorComponent::MAX,
		}
	}

	pub const fn rgba(r: ColorComponent, g: ColorComponent, b: ColorComponent, a: ColorComponent) -> Self {
		Self {
			r, g, b, a,
		}
	}
}

impl From<(ColorComponent, ColorComponent, ColorComponent)> for Color {
	fn from((r, g, b): (ColorComponent, ColorComponent, ColorComponent)) -> Self {
		Self::rgb(r, g, b)
	}
}

impl From<(ColorComponent, ColorComponent, ColorComponent, ColorComponent)> for Color {
	fn from((r, g, b, a): (ColorComponent, ColorComponent, ColorComponent, ColorComponent)) -> Self {
		Self::rgba(r, g, b, a)
	}
}
