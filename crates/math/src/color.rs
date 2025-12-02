pub type ColorComponent = ::core::ffi::c_uchar;

/// Source Engine Red-Green-Blue-Alpha color type.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Color {
	pub rgba: [ColorComponent; 4],
}

macro_rules! component_fns {
	{
		name = $name:literal;
		index = $index:literal;
		$(
			get = $get:ident, mut = $mut:ident;
		)+
	} => {
		$(
			#[doc = concat!("Returns the ", $name, " component of the color.")]
			/// 
			/// # Examples
			/// ```
			#[doc = concat!("# use ", env!("CARGO_CRATE_NAME"), "::Color;")]
			/// let mut color = Color::rgba(31, 63, 127, 255);
			#[doc = concat!("*color.", stringify!($mut), "() = 15;")]
			#[doc = concat!("assert_eq!(color.", stringify!($get), "(), 15);")]
			/// ```
			pub const fn $get(&self) -> ColorComponent {
				self.rgba[$index]
			}

			#[doc = concat!("Returns a mutable reference to the ", $name, " component of the color.")]
			/// 
			/// # Examples
			/// ```
			#[doc = concat!("# use ", env!("CARGO_CRATE_NAME"), "::Color;")]
			/// let mut color = Color::rgba(31, 63, 127, 255);
			#[doc = concat!("*color.", stringify!($mut), "() = 15;")]
			#[doc = concat!("assert_eq!(color.", stringify!($get), "(), 15);")]
			/// ```
			pub const fn $mut(&mut self) -> &mut ColorComponent {
				&mut self.rgba[$index]
			}
		)+
	};
}

impl Color {
	/// Returns an opaque color with the given Red, Green and Blue components.
	pub const fn rgb(red: ColorComponent, green: ColorComponent, blue: ColorComponent) -> Self {
		Self {
			rgba: [red, green, blue, ColorComponent::MAX],
		}
	}

	/// Returns a color with the given Red, Green, Blue and Alpha components.
	pub const fn rgba(red: ColorComponent, green: ColorComponent, blue: ColorComponent, alpha: ColorComponent) -> Self {
		Self {
			rgba: [red, green, blue, alpha],
		}
	}

	component_fns! {
		name = "Red";
		index = 0;
		get = red, mut = red_mut;
		get = r, mut = r_mut;
	}

	component_fns! {
		name = "Green";
		index = 1;
		get = green, mut = green_mut;
		get = g, mut = g_mut;
	}

	component_fns! {
		name = "Blue";
		index = 2;
		get = b, mut = b_mut;
		get = blue, mut = blue_mut;
	}

	component_fns! {
		name = "Alpha";
		index = 3;
		get = alpha, mut = alpha_mut;
		get = a, mut = a_mut;
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
