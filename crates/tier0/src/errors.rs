pub trait Tier0Errors<T> {
	fn error(&self, t: T) -> !;
}
