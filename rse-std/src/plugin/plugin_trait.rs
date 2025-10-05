use ::core::{
	error::Error,
	fmt::Write,
};
use ::rse_game_interfaces::InterfaceFactories;
use ::rse_tier0::{
	fmt_adapters::Warning,
	linked::spew::con,
};

pub trait Plugin: Sized {
	type LoadError: Error;
	fn load(factories: InterfaceFactories<'_>) -> Result<Self, Self::LoadError>;
}

pub fn load_plugin<P>(factories: InterfaceFactories<'_>) -> Option<P>
where
	P: Plugin,
{
	match P::load(factories) {
		Ok(plugin) => Some(plugin),
		Err(error) => {
			let mut output = Warning(&con());
			let _ = write!(output, "{error}");

			let mut source_mut = error.source();
			while let Some(source) = source_mut {
				let _ = write!(output, ": {source}");
				source_mut = source.source();
			}
			let _ = writeln!(output);

			None
		}
	}
}
