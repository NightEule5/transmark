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

use std::ops::Range;

use regex::{Match, Regex, Captures, CaptureMatches};
use regex_macro::regex;

fn tag_regex() -> &'static Regex {
	regex!(r#"(?ix)
	\[(
		(?P<param>
			(?P<tag>\w+)
			(=[\S&&[^\]]]+)? # Single parameter, i.e. =800x600
			(
				\s+     # Separating whitespace
				[a-z]+  # Parameter key
				=
				"[^"]+" # Parameter value
			)*
		)|
		/(?P<endTag>\w+)
	)]
	"#)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Fragment<'t> {
	Text(TextFragment<'t>),
	StartTag(Tag<'t>),
	EndTag(TextFragment<'t>),
}

impl<'t> Fragment<'t> {
	fn new_text(text: &'t str, range: Range<usize>) -> Self {
		Self::Text(TextFragment(text, range))
	}

	fn new_start_tag(
		name: Match<'t>,
		param: Match<'t>
	) -> Self {
		Self::new_start_tag_raw(
			name.as_str(),
			name.range(),
			param.as_str(),
			param.range()
		)
	}

	fn new_start_tag_raw(
		name: &'t str,
		name_range: Range<usize>,
		param: &'t str,
		param_range: Range<usize>
	) -> Self {
		Self::StartTag(
			Tag {
				name : TextFragment(name,   name_range),
				param: TextFragment(param, param_range)
			}
		)
	}

	fn new_end_tag(tag_match: Match<'t>) -> Self {
		Self::new_end_tag_raw(
			tag_match.as_str(),
			tag_match.range()
		)
	}

	fn new_end_tag_raw(
		tag: &'t str,
		range: Range<usize>
	) -> Self {
		Self::EndTag(TextFragment(tag, range))
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Tag<'t> {
	pub name : TextFragment<'t>,
	pub param: TextFragment<'t>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TextFragment<'t>(pub &'t str, pub Range<usize>);

pub(super) fn split_fragments(input: &str) -> FragmentStream<'_> {
	FragmentStream::new(input)
}

pub(super) struct FragmentStream<'t> {
	input: &'t str,
	pos: usize,
	last_tag: Option<Captures<'t>>,
	tags: CaptureMatches<'static, 't>,
}

impl<'t> FragmentStream<'t> {
	fn new(input: &'t str) -> Self {
		Self {
			input,
			pos: 0,
			last_tag: None,
			tags: tag_regex().captures_iter(input),
		}
	}

	fn next_text(&mut self) -> Option<Fragment<'t>> {
		let Self {
			input,
			pos,
			last_tag,
			tags,
		} = self;

		if let Some(_) = last_tag {
			return None
		}

		let len = input.len();

		if *pos >= len {
			return None
		}

		*last_tag = tags.next();

		Some(
			if let Some(last_tag) = last_tag {
				let tag_match = last_tag.get(0).expect("no match");
				let range = *pos..tag_match.start();
				*pos = tag_match.end();

				if range.is_empty() {
					return None
				}

				Fragment::new_text(&input[range.clone()], range)
			} else {
				let range = *pos..len;
				*pos += len;

				Fragment::new_text(&input[range.clone()], range)
			}
		)
	}

	fn next_tag(&mut self) -> Option<Fragment<'t>> {
		self.last_tag
			.take()
			.map(|captures| {
				if let Some(param) = captures.name("param") {
					let tag = captures
						.name("tag")
						.expect("no tag group");

					Fragment::new_start_tag(tag, param)
				} else {
					let end_tag = captures
						.name("endTag")
						.expect("no endTag group");

					Fragment::new_end_tag(end_tag)
				}
			})
	}

	fn next_fragment(&mut self) -> Option<Fragment<'t>> {
		self.next_text()
			.or_else(|| self.next_tag())
	}
}

impl<'t> Iterator for FragmentStream<'t> {
	type Item = Fragment<'t>;

	fn next(&mut self) -> Option<Self::Item> {
		self.next_fragment()
	}
}

#[cfg(test)]
mod tests {
    use super::*;

	use ExpectedFragment::*;

	const SIMPLE_BLOCK     : &str = r"[tag]text[/tag]";
	const BLOCK_WITH_VALUE : &str = r"[tag=value]text[/tag]";
	const BLOCK_WITH_PARAMS: &str = r#"[tag abc="val1" def="val2"]text[/tag]"#;
	const BLOCK_WITH_BOTH  : &str = r#"[tag=value abc="val1" def="val2"]text[/tag]"#;
	const TEXT_PREFIX_BLOCK: &str = r"You can make [b]bold text![/b]";
	const TEXT_SUFFIX_BLOCK: &str = r"[u]Underline[/u] text with the 'u' tag!";
	const TEXT_INFIX_BLOCK : &str = "Paragraph 1\n[h1]Heading[/h1]\nParagraph 2";
	const NESTED_BLOCK     : &str = r"[s]Stricken and [i]italicized[/i] text[/s]";

	// Test set generation

	static SIMPLE_BLOCK_EXP: ExpectedSequence = &[
		("root", StartTag("tag", "tag")),
		("inner", Text("text")),
		("root", EndTag("tag"))
	];

	static BLOCK_WITH_VALUE_EXP: ExpectedSequence = &[
		("root", StartTag("tag", "tag=value")),
		("inner", Text("text")),
		("root", EndTag("tag"))
	];

	static BLOCK_WITH_PARAMS_EXP: ExpectedSequence = &[
		("root", StartTag("tag", r#"tag abc="val1" def="val2""#)),
		("inner", Text("text")),
		("root", EndTag("tag"))
	];

	static BLOCK_WITH_BOTH_EXP: ExpectedSequence = &[
		("root", StartTag("tag", r#"tag=value abc="val1" def="val2""#)),
		("inner", Text("text")),
		("root", EndTag("tag"))
	];

	static TEXT_PREFIX_BLOCK_EXP: ExpectedSequence = &[
		("prefix", Text("You can make ")),
		("bold", StartTag("b", "b")),
		("inner", Text("bold text!")),
		("bold", EndTag("b"))
	];

	static TEXT_SUFFIX_BLOCK_EXP: ExpectedSequence = &[
		("underline", StartTag("u", "u")),
		("inner", Text("Underline")),
		("underline", EndTag("u")),
		("suffix", Text(" text with the 'u' tag!"))
	];

	static TEXT_INFIX_BLOCK_EXP: ExpectedSequence = &[
		("prefix", Text("Paragraph 1\n")),
		("heading", StartTag("h1", "h1")),
		("inner", Text("Heading")),
		("heading", EndTag("h1")),
		("suffix", Text("\nParagraph 2")),
	];

	static NESTED_BLOCK_EXP: ExpectedSequence = &[
		("strike", StartTag("s", "s")),
		("inner prefix", Text("Stricken and ")),
		("italic", StartTag("i", "i")),
		("inner", Text("italicized")),
		("italic", EndTag("i")),
		("inner suffix", Text(" text")),
		("strike", EndTag("s"))
	];

	type ExpectedSequence = &'static [(&'static str, ExpectedFragment)];

	enum ExpectedFragment {
		Text(&'static str),
		StartTag(&'static str, &'static str),
		EndTag(&'static str)
	}

	fn generate(seq: ExpectedSequence) -> Vec<(String, Fragment<'static>)> {
			let mut i = 0;

			seq.into_iter().map(move |(label, expected)| {
			match expected {
				Text(value) => {
					let start = i;

					i += value.len();

					(
						format!("{label} start tag"),
						Fragment::new_text(value, start..i)
					)
				}
				StartTag(name, param) => {
					i += 1; // Skip [

					let  name_len = name .len();
					let param_len = param.len();
					let  name_end = i +  name_len;
					let param_end = i + param_len;

					let pair = (
						format!("{label} text"),
						Fragment::new_start_tag_raw(
							name,
							i.. name_end,
							param,
							i..param_end
						)
					);

					i = param_end + 1; // Skip ]

					pair
				}
				EndTag(name) => {
					i += 2; // Skip [/

					let start = i;

					i += name.len();

					let pair = (
						format!("{label} end tag"),
						Fragment::new_end_tag_raw(name, start..i)
					);

					i += 1; // Skip ]

					pair
				}
			}
		}).collect()
	}

	fn test(
		input: &str,
		expected: ExpectedSequence
	) {
		let mut fragments = split_fragments(input);

		for (label, exp_fragment) in generate(expected) {
			let expect_msg = format!("no {label}");

			assert_eq!(
				fragments.next().expect(expect_msg.as_str()),
				exp_fragment,
				"invalid {label}"
			);
		}

		assert_eq!(fragments.next(), None, "extra fragment(s) collected");
	}

	#[test]
	fn simple_block() {
		test(SIMPLE_BLOCK, SIMPLE_BLOCK_EXP);
	}

	#[test]
	fn block_with_value() {
		test(BLOCK_WITH_VALUE, BLOCK_WITH_VALUE_EXP);
	}

	#[test]
	fn block_with_params() {
		test(BLOCK_WITH_PARAMS, BLOCK_WITH_PARAMS_EXP);
	}

	#[test]
	fn block_with_both() {
		test(BLOCK_WITH_BOTH, BLOCK_WITH_BOTH_EXP);
	}

	#[test]
	fn text_prefix_block() {
		test(TEXT_PREFIX_BLOCK, TEXT_PREFIX_BLOCK_EXP);
	}

	#[test]
	fn text_suffix_block() {
		test(TEXT_SUFFIX_BLOCK, TEXT_SUFFIX_BLOCK_EXP);
	}

	#[test]
	fn text_infix_block() {
		test(TEXT_INFIX_BLOCK, TEXT_INFIX_BLOCK_EXP);
	}

	#[test]
	fn nested_block() {
		test(NESTED_BLOCK, NESTED_BLOCK_EXP);
	}
}
