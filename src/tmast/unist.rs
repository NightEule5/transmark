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

use std::cmp::min;

pub use markdown::unist::*;

use crate::util::LastIndex;

/// A node.
pub trait Node {
	/// Returns the node [Position] within the document.
	fn position(&self) -> Option<Position>;

	/// Sets the node [Position].
	fn set_position(&mut self, position: Option<Position>);
}

/// A node containing other nodes.
pub trait Parent<N> : Node {
	/// Returns a slice of the node's children.
	fn children<'c>(&'c self) -> &'c [N];

	/// Appends a child the the node.
	fn append_child(&mut self, node: N);
}

/// A leaf node that contains a literal string.
pub trait Literal<'v> : Node {
	/// Returns the value.
	fn value(&self) -> &'v str;

	/// Sets the value.
	fn set_value(&mut self, value: &'v str);
}

pub trait PositionSlice {
	/// Returns a slice of the specified string at this position.
	fn slice<'t>(&self, value: &'t str) -> &'t str;
}

pub trait PointOffset {
	/// Creates a [Point] with the specified offset, calulating its line and column
	/// from the value. If the offset falls outside the value, the line number will
	/// be the value's line count, and the column will be the remaining characters.
	fn new_offset(offset: usize, value: &str) -> Point;
}

impl PositionSlice for Position {
	fn slice<'t>(&self, value: &'t str) -> &'t str {
		let start = self.start.offset;
		let end   = self.end  .offset;
		&value[start..min(end, value.len())]
	}
}

impl PointOffset for Point {
	fn new_offset(offset: usize, value: &str) -> Point {
		fn take_lines(str: &str, count: usize) -> &str {
			let mut n = 0;

			for line in str.lines().take(count) {
				n += line.len();

				let get_next = |v: &str|
					str.get(n..n + v.len())
						.filter(|next| *next == v);

				n += get_next("\n") // LF
					.or_else(|| get_next("\r\n")) // CRLF
					.map_or(0, str::len);
			}

			&str[..=n]
		}

		let len  = min(value.len(), offset + 1);
		let str  = &value[..len];
		let line = str.lines().count();
		let mut column = offset + 1;
		column -= take_lines(str, line - 1).last_index().unwrap_or_default();
		
		Point { line, column, offset }
	}
}

#[cfg(test)]
mod tests {
    use super::{Point, PointOffset};

	#[test]
	fn single_line_point() {
		let value = "The quick brown fox";
		let offset = 8;
		let exp = Point {
			line: 1,
			column: 9,
			offset
		};

		assert_eq!(Point::new_offset(offset, value), exp);
	}

	#[test]
	fn lf_point() {
		let value = "The quick brown fox\njumps over the lazy dog";
		let offset = 26;
		let exp = Point {
			line: 2,
			column: 7,
			offset
		};

		assert_eq!(Point::new_offset(offset, value), exp);
	}

	#[test]
	fn crlf_point() {
		let value = "The quick brown fox\r\njumps over the lazy dog";
		let offset = 27;
		let exp = Point {
			line: 2,
			column: 7,
			offset
		};

		assert_eq!(Point::new_offset(offset, value), exp);
	}

	#[test]
	fn mixed_point() {
		let value = "The quick\r\nbrown fox\njumps over\r\nthe lazy dog";
		let offset = 37;
		let exp = Point {
			line: 4,
			column: 5,
			offset
		};

		assert_eq!(Point::new_offset(offset, value), exp);
	}

	#[test]
	fn oob_point() {
		let value = "The quick brown fox\njumps over the lazy dog";
		let offset = 43;
		let exp = Point {
			line: 2,
			column: 24,
			offset
		};

		assert_eq!(Point::new_offset(offset, value), exp);
	}
}
