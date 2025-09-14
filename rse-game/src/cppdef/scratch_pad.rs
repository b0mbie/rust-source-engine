use ::core::ffi::{
	c_char, c_float, c_int, c_ulong, c_uchar,
};
use ::rse_cpp::RefConst;
use ::rse_math::{
	QAngle, Vector, Vector2D,
};
use ::rse_utl::cppdef::UtlVector;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CspColor {
	pub color: Vector,
	pub alpha: c_float,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CspVert {
	pub pos: Vector,
	pub color: CspColor,
}

#[derive(Debug)]
#[repr(C)]
pub struct CspVertList {
	pub verts: UtlVector<CspVert>,
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
::rse_cpp::vtable! {
	pub ScratchPad3DVt {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
		pub fn release();
		pub fn set_mapping(
			input_min: RefConst<Vector>,
			input_max: RefConst<Vector>,
			output_min: RefConst<Vector>,
			output_max: RefConst<Vector>,
		);
		pub fn get_auto_flush() -> bool;
		pub fn set_auto_flush(auto_flush: bool);
		pub fn draw_point(v: RefConst<CspVert>, point_size: c_float);
		pub fn draw_line(v1: RefConst<CspVert>, v2: RefConst<CspVert>);
		pub fn draw_polygon(verts: RefConst<CspVertList>);
		pub fn draw_rect_yz(x_pos: c_float, min: RefConst<Vector2D>, max: RefConst<Vector2D>, color: RefConst<CspColor>);
		pub fn draw_rect_xz(y_pos: c_float, min: RefConst<Vector2D>, max: RefConst<Vector2D>, color: RefConst<CspColor>);
		pub fn draw_rect_xy(z_pos: c_float, min: RefConst<Vector2D>, max: RefConst<Vector2D>, color: RefConst<CspColor>);
		pub fn draw_wireframe_box(min: RefConst<Vector>, max: RefConst<Vector>, color: RefConst<Vector>);
		pub fn draw_text(str: *const c_char, params: RefConst<TextParams>);
		pub fn set_render_state(state: RenderState, val: c_ulong);
		pub fn clear();
		pub fn flush();
		pub fn draw_image_bw(
			data: *const c_uchar,
			width: c_int, height: c_int, pitch_in_bytes: c_int,
			outline_pixels: bool, outline_image: bool,
			corners_bl_tl_tr_br_or_default: *const Vector,
		);
		pub fn draw_image_rgba(
			data: *const SpRgba,
			width: c_int, height: c_int, pitch_in_bytes: c_int,
			outline_pixels: bool, outline_image: bool,
			corners_bl_tl_tr_br_or_default: *const Vector,
		);
	}
}
