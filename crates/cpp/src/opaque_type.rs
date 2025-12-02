#[macro_export]
macro_rules! opaque_type {
	{
		$(#[$attr:meta])*
		$vis:vis struct $name:ident;
	} => {
		#[repr(C)]
		$(#[$attr])*
		$vis struct $name {
			_data: (),
			_marker: ::core::marker::PhantomData<(*mut u8, ::core::marker::PhantomPinned)>,
		}
	};
}
