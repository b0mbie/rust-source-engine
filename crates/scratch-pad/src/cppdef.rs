use ::core::ffi::{
	c_char, c_float, c_int, c_ulong, c_uchar,
};
use ::rse_cpp::{
	class,
	RefConst,
};
use ::rse_math::{
	QAngle, Vector, Vector2D,
};
use ::rse_utl::cppdef::CUtlVector;

class! {
	#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
	pub CSPColor {
		pub color: Vector;
		pub alpha: c_float;
	}
}

class! {
	#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
	pub CSPVert {
		pub pos: Vector;
		pub color: CSPColor;
	}
}

class! {
	#[derive(Debug)]
	pub CSPVertList {
		pub verts: CUtlVector<CSPVert>;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum RenderState {
	FillMode = 0,
	ZRead,
	ZBias,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum FillMode {
	Wireframe = 0,
	Solid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SpRgba {
	pub r: c_uchar,
	pub g: c_uchar,
	pub b: c_uchar,
	pub a: c_uchar,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct TextParams {
	/// Text color.
	pub color: Vector,
	/// Text alpha value.
	pub alpha: c_float,
	/// `true` if the background is a solid color.
	pub solid_background: bool,
	/// `true` if the text has an outline.
	pub outline: bool,
	/// Position to render the text in.
	pub position: Vector,
	/// `true` if centered on `position`,
	/// `false` if `position` is the upper-left corner.
	pub centered: bool,
	/// Orientation of the text.
	pub angles: QAngle,
	/// `true` if rendering the text from both sides.
	pub two_sided: bool,
	/// Letter width in world space.
	pub letter_width: c_float,
}

// TODO: `IScratchPad3D`.
class! {
	pub IScratchPad3D {
		pub virtual fn destructor();
		#[cfg(not(windows))]
		pub virtual fn destructor_2();
		pub virtual fn release();
		pub virtual fn set_mapping(
			input_min: RefConst<Vector>,
			input_max: RefConst<Vector>,
			output_min: RefConst<Vector>,
			output_max: RefConst<Vector>,
		);
		pub virtual fn get_auto_flush() -> bool;
		pub virtual fn set_auto_flush(auto_flush: bool);
		pub virtual fn draw_point(v: RefConst<CSPVert>, point_size: c_float);
		pub virtual fn draw_line(v1: RefConst<CSPVert>, v2: RefConst<CSPVert>);
		pub virtual fn draw_polygon(verts: RefConst<CSPVertList>);
		pub virtual fn draw_rect_yz(x_pos: c_float, min: RefConst<Vector2D>, max: RefConst<Vector2D>, color: RefConst<CSPColor>);
		pub virtual fn draw_rect_xz(y_pos: c_float, min: RefConst<Vector2D>, max: RefConst<Vector2D>, color: RefConst<CSPColor>);
		pub virtual fn draw_rect_xy(z_pos: c_float, min: RefConst<Vector2D>, max: RefConst<Vector2D>, color: RefConst<CSPColor>);
		pub virtual fn draw_wireframe_box(min: RefConst<Vector>, max: RefConst<Vector>, color: RefConst<Vector>);
		pub virtual fn draw_text(str: *const c_char, params: RefConst<TextParams>);
		pub virtual fn set_render_state(state: RenderState, val: c_ulong);
		pub virtual fn clear();
		pub virtual fn flush();
		pub virtual fn draw_image_bw(
			data: *const c_uchar,
			width: c_int, height: c_int, pitch_in_bytes: c_int,
			outline_pixels: bool, outline_image: bool,
			corners_bl_tl_tr_br_or_default: *const Vector,
		);
		pub virtual fn draw_image_rgba(
			data: *const SpRgba,
			width: c_int, height: c_int, pitch_in_bytes: c_int,
			outline_pixels: bool, outline_image: bool,
			corners_bl_tl_tr_br_or_default: *const Vector,
		);
	}
}
