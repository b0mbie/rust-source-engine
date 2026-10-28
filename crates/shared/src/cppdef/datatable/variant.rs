use ::core::{
	ffi::{
		CStr, c_char, c_float, c_int, c_void,
	},
	fmt::{
		self, Write as _,
	},
};

use super::SendPropType;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DVariant {
	pub inner: VariantInner,
	pub prop_type: SendPropType,
}

impl fmt::Display for DVariant {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unsafe { match self.prop_type {
			SendPropType::Int => self.inner.as_int.fmt(f),
			SendPropType::Float => self.inner.as_float.fmt(f),
			SendPropType::Vector => {
				f.write_str("(")?;
				let [x, y, z] = self.inner.as_vector;
				x.fmt(f)?;
				f.write_str(", ")?;
				y.fmt(f)?;
				f.write_str(", ")?;
				z.fmt(f)?;
				f.write_str(")")
			}
			SendPropType::VectorXy => {
				f.write_str("(")?;
				let [x, y, ..] = self.inner.as_vector;
				x.fmt(f)?;
				f.write_str(", ")?;
				y.fmt(f)?;
				f.write_str(")")
			}
			SendPropType::String => {
				let string_ptr = self.inner.as_string;
				if !string_ptr.is_null() {
					let c_str = CStr::from_ptr(string_ptr);
					for chunk in c_str.to_bytes().utf8_chunks() {
						f.write_str(chunk.valid())?;
						if !chunk.invalid().is_empty() {
							f.write_char(char::REPLACEMENT_CHARACTER)?;
						}
					}
					Ok(())
				} else {
					f.write_str("NULL")
				}
			}
			SendPropType::Array => f.write_str("Array"),
			SendPropType::DataTable => f.write_str("DataTable"),
			#[cfg(feature = "datatable-supports-int64")]
			SendPropType::Int64 => self.inner.as_int64.fmt(f),
		} }
	}
}

impl fmt::Debug for DVariant {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unsafe { match self.prop_type {
			SendPropType::Int => self.inner.as_int.fmt(f),
			SendPropType::Float => self.inner.as_float.fmt(f),
			SendPropType::Vector => self.inner.as_vector.fmt(f),
			SendPropType::VectorXy => self.inner.as_vector[..2].fmt(f),
			SendPropType::String => {
				let string_ptr = self.inner.as_string;
				if !string_ptr.is_null() {
					CStr::from_ptr(string_ptr).fmt(f)
				} else {
					f.write_str("NULL")
				}
			}
			SendPropType::Array => self.inner.as_data.fmt(f),
			SendPropType::DataTable => self.inner.as_data.fmt(f),
			#[cfg(feature = "datatable-supports-int64")]
			SendPropType::Int64 => self.inner.as_int64.fmt(f),
		} }
	}
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union VariantInner {
	pub as_float: c_float,
	pub as_int: c_int,
	pub as_string: *const c_char,
	pub as_data: *mut c_void,
	pub as_vector: [c_float; 3],
	#[cfg(feature = "datatable-supports-int64")]
	pub as_int64: i64,
}
