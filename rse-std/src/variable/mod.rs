use ::rse_convar::variable::Variable;

mod object;
pub use object::*;
mod wrapper;
pub use wrapper::*;

pub const fn con_var<T>(inner: T) -> ConVar<T>
where
	T: Variable,
{
	ConVar::new(inner)
}
