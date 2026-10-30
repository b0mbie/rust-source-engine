use ::core::{
	convert::Infallible,
	ptr::NonNull,
};
use ::rse_cpp::{
	VTablePtr, new_vtable_self,
	This, this_to_self,
};

use crate::{
	cppdef::{
		IGameEventListener2, IGameEventListener2Vt,
		IGameEvent,
	},
	GameEvent,
};

pub trait FromEvent<'a, L: ?Sized>: Sized {
	type Error;
	fn from_event(listener: &mut L, event: &'a mut GameEvent) -> Result<Self, Self::Error>;
}

impl<'a, L: ?Sized> FromEvent<'a, L> for &'a mut GameEvent {
	type Error = Infallible;
	fn from_event(_: &mut L, event: &'a mut GameEvent) -> Result<Self, Self::Error> {
		Ok(event)
	}
}

pub trait EventListener<'a> {
	type Event: FromEvent<'a, Self>;
	fn on_event(&mut self, event: Self::Event);
}
impl<'a, T: EventListener<'a>> EventAcceptor<'a> for T {
	fn accept_event(&mut self, event: &'a mut GameEvent) {
		if let Ok(event) = T::Event::from_event(self, event) {
			self.on_event(event)
		}
	}
}

pub trait EventAcceptor<'a> {
	fn accept_event(&mut self, event: &'a mut GameEvent);
}

#[repr(C)]
pub struct EventListenerObject<T> {
	raw: IGameEventListener2,
	inner: T,
}

impl<T> Default for EventListenerObject<T>
where
	T: for<'a> EventAcceptor<'a> + Default,
{
	fn default() -> Self {
		Self::new(T::default())
	}
}

impl<T> EventListenerObject<T>
where
	T: for<'a> EventAcceptor<'a>,
{
	pub const fn new(inner: T) -> Self {
		Self {
			raw: IGameEventListener2 {
				vtable: VTablePtr::from_ref(Self::VTABLE),
			},
			inner,
		}
	}

	pub const fn as_listener(&mut self) -> Listener {
		Listener::from_mut(&mut self.raw)
	}

	pub const fn as_inner(&self) -> &T {
		&self.inner
	}

	pub const fn as_mut_inner(&mut self) -> &mut T {
		&mut self.inner
	}

	const VTABLE: &IGameEventListener2Vt = &new_vtable_self!(IGameEventListener2Vt {
		destructors: rse_cpp::Destructors::new(Self::destructor),
		fire_game_event
	});

	::rse_cpp::vtable_methods! {
		this: This<IGameEventListener2>;
		fn destructor() {
			unsafe { this.cast::<Self>().drop_in_place() }
		}
		fn fire_game_event(event: *mut IGameEvent) {
			let event = unsafe { GameEvent::from_mut_ptr(event) };
			unsafe { this_to_self!(mut this) }.inner.accept_event(event)
		}
	}
}

pub type Listener = NonNull<IGameEventListener2>;
