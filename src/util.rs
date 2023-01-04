pub trait Consume<T> {
	fn consume(self, consume: impl FnOnce(T) -> ());
}

impl<T> Consume<T> for Option<T> {
	fn consume(self, consume: impl FnOnce(T) -> ()) {
		self.map(consume);
	}
}

pub trait Zip<T, E> {
	fn zip(self, other: Result<T, E>) -> Result<(T, T), E>;
}

impl<T, E> Zip<T, E> for Result<T, E> {
	fn zip(self, other: Result<T, E>) -> Result<(T, T), E> {
		self.and_then(|x| Ok((x, other?)))
	}
}
