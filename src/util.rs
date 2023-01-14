/*
 * Copyright 2023 Strixpyrr
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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

pub trait LastIndex {
	/// Returns the last index of an indexed value.
	fn last_index(&self) -> Option<usize>;
}

impl LastIndex for str {
	fn last_index(&self) -> Option<usize> {
		self.len().checked_sub(1)
	}
}
