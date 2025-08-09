use ::core::{
	convert::Infallible,
	ffi::c_int,
};

pub trait PrecisionSome {}
impl PrecisionSome for c_int {}
impl PrecisionSome for Infallible {}
pub trait PrecisionNone {}
impl PrecisionNone for () {}
impl PrecisionNone for Infallible {}

pub trait IntoPrecision {
	type Some: PrecisionSome;
	type None: PrecisionNone;
	fn into_precision(self) -> Result<Self::Some, Self::None>;
}
impl IntoPrecision for () {
	type Some = Infallible;
	type None = ();
	fn into_precision(self) -> Result<Self::Some, Self::None> {
		Err(())
	}
}
impl IntoPrecision for c_int {
	type Some = c_int;
	type None = Infallible;
	fn into_precision(self) -> Result<Self::Some, Self::None> {
		Ok(self)
	}
}
impl IntoPrecision for Option<c_int> {
	type Some = c_int;
	type None = ();
	fn into_precision(self) -> Result<<Self as IntoPrecision>::Some, <Self as IntoPrecision>::None> {
		match self {
			Some(precision) => Ok(precision),
			None => Err(()),
		}
	}
}
