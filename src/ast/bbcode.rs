//! A basic BBCode parser implementation, parsing directly to the common AST. 

mod tag_builders;
mod parser;
mod tokenizer;

use std::collections::HashMap;
use std::num::ParseIntError;
use std::ops::Range;
use std::vec;

use fancy_regex::{Regex, Error as RegexError, Match};
use lazy_static::lazy_static;
use markdown::mdast::Root;
use regex_macro::regex;

use crate::TmDoc;

use super::{NodeBuilder, BlockNode};

lazy_static! {
	static ref BLOCK_REGEX: Regex =
		Regex::new(r#"(?six)
		\[
			(?<params>
				(?<tag>[a-z]+)
				(=\S+?)? # Single parameter, i.e. =800x600
				(
					\s+     # Separating whitespace
					[a-z]+  # Parameter key
					=
					"[^"]+" # Parameter value
				)*
			)
		]
		(?<inner>.*)
		\[/\k<tag>] # End tag
		"#).unwrap();
}

use ErrorKind::*;

#[derive(Debug)]
pub enum ErrorKind<'t> {
	UnclosedTag(&'t str),
	UnopenedTag(&'t str),
	UnknownTag(&'t str),
	UnexpectedTag(&'t str),
	TagParamParse {
		tag: &'t str,
		key: &'t str,
		val: &'t str,
		err: ParseIntError
	},
	TagParamInvalid {
		tag: &'t str,
		key: &'t str,
		val: &'t str,
		err: String
	},
	TagParamMissing {
		tag: &'t str,
		key: &'t str
	},
	MatchFailed(RegexError),
}

#[derive(Debug)]
pub struct Error<'t> {
	pub range: Range<usize>,
	pub kind: ErrorKind<'t>,
}

impl<'t> Error<'t> {
	fn new(pos: usize, len: usize, kind: ErrorKind<'t>) -> Self {
		Self::new_range(pos..pos + len, kind)
	}

	fn new_range(range: Range<usize>, kind: ErrorKind<'t>) -> Self {
		Self { range, kind }
	}

	fn unclosed_tag(range: Range<usize>, tag: &'t str) -> Self {
		Self::new_range(range, UnclosedTag(tag))
	}

	fn unopened_tag(range: Range<usize>, tag: &'t str) -> Self {
		Self::new_range(range, UnopenedTag(tag))
	}

	fn unknown_tag(range: Range<usize>, tag: &'t str) -> Self {
		Self::new_range(range, UnknownTag(tag))
	}

	fn unexpected_tag(range: Range<usize>, tag: &'t str) -> Self {
		Self::new_range(range, UnexpectedTag(tag))
	}

	fn param_missing(
		range: Range<usize>,
		tag: &'t str,
		key: &'t str
	) -> Self {
		Self::new_range(
			range,
			TagParamMissing { tag, key }
		)
	}

	fn param_parse(
		range: Range<usize>,
		tag: &'t str,
		key: &'t str,
		val: &'t str,
		err: ParseIntError
	) -> Self {
		Self::new_range(
			range,
			TagParamParse { tag, key, val, err }
		)
	}

	fn param_invalid(
		range: Range<usize>,
		tag: &'t str,
		key: &'t str,
		val: &'t str,
		err: &'t str
	) -> Self {
		Self::new_range(
			range,
			TagParamInvalid { tag, key, val, err: err.to_string() }
		)
	}
}

pub fn parse(value: &str) -> Result<TmDoc, Error> {
	// Wat?
	/*Ok(split(value, 0..value.len())?
		.into_iter()
		.fold(
			Ok(NodeBuilder::<Root>::new()),
			|nb, block| parse_block(nb?, block)
		)?.build())*/
	todo!()
}

/*pub(super) enum Block<'t> {
	Text(&'t str),
	Tag(Match<'t>, Option<Match<'t>>, (Range<usize>, Vec<Block<'t>>))
}

impl<'t> Block<'t> {
	fn text(self) -> Result<&'t str, Error<'t>> {
		match self {
			Block::Text(value) => Ok(value),
			Block::Tag(tag, _, _) => Err(
				Error::unexpected_tag(tag.range(), tag.as_str())
			)
		}
	}
}

fn split(value: &str, range: Range<usize>) -> Result<Vec<Block<'_>>, Error> {
	let to_err = |error, pos|
		Error::new(
			range.start + pos,
			value.len(),
			MatchFailed(error)
		);

	let mut blocks = Vec::with_capacity(16);
	let mut pos = 0;

	for block_res in BLOCK_REGEX.captures_iter(value) {
		let block_caps = block_res.map_err(|e| to_err(e, pos))?;
		let block_start = block_caps.get(0).unwrap().start();

		if block_start > pos {
			let text = &value[pos..block_start];

			if let Some(unclosed_tag) = regex!(r"(?i)\[\w+]").find(text) {
				let start = range.start + unclosed_tag.start();
				let end   = range.end   + unclosed_tag.end();

				let mut tag = unclosed_tag.as_str();
						tag = &tag[1..tag.len() - 1];

				Err(Error::unclosed_tag(start..end, tag))?
			}

			if let Some(unopened_tag) = regex!(r"(?i)\[/\w+]").find(text) {
				let start = range.start + unopened_tag.start();
				let end   = range.end   + unopened_tag.end();

				let mut tag = unopened_tag.as_str();
						tag = &tag[2..tag.len() - 1];

				Err(Error::unopened_tag(start..end, tag))?
			}

			blocks.push(Block::Text(text));
		}

		let tag   = block_caps.name("tag"  ).unwrap();
		let param = block_caps.name("param");
		let inner = block_caps.name("inner").unwrap();

		let inner_value = inner.as_str();
		let inner_range = inner.range();

		fn is_verbatim(tag: &str, param: Option<Match>) -> bool {
			tag.eq_ignore_ascii_case("code"   ) ||
			tag.eq_ignore_ascii_case("img"    ) ||
			tag.eq_ignore_ascii_case("pre"    ) ||
			tag.eq_ignore_ascii_case("youtube") ||
			tag.eq_ignore_ascii_case("url") && param.is_none()
		}

		blocks.push(
			Block::Tag(
				tag,
				param,
				if inner_range.is_empty() {
					(inner_range, vec![])
				} else if is_verbatim(tag.as_str(), param) {
					(inner_range, vec![ Block::Text(inner_value) ])
				} else {
					(inner_range.clone(), split(inner_value, inner_range)?)
				}
			)
		);

		pos += block_caps.len();
	}

	Ok(blocks)
}

pub(self) fn parse_block<N : BlockNode>(
	builder: NodeBuilder<N>,
	block: Block<'_>
) -> Result<NodeBuilder<N>, Error> {
	Ok(match block {
    	Block::Text(value) => builder.text(value.to_string()),
		Block::Tag(tag, param, (inner_range, inner_blocks)) => {
			let parameters = param
				.map(TagParameters::split)
				.unwrap_or_else(||
					TagParameters::empty(tag.end())
				);
			
			Tag::parse(tag.as_str(), tag.range(), parameters)?
				.build(builder, inner_range, inner_blocks)?
		}
	})
}

struct TagParameters<'t> {
	full_range: Range<usize>,
	params: HashMap<&'t str, &'t str>,
	ranges: HashMap<&'t str, Range<usize>>
}

impl<'t> TagParameters<'t> {
	fn empty(off: usize) -> Self {
		Self {
			full_range: off..off,
			params: HashMap::new(),
			ranges: HashMap::new()
		}
	}

	fn split(parameters: Match) -> Self {
		let pos = parameters.start();

		let pairs = parameters.as_str().split(' ');

		let params: HashMap<_, _> = pairs
			.filter(|s| !s.is_empty())
			.map(|pair| pair.split_once('=').unwrap())
			.collect();
		let ranges: HashMap<_, _> = pairs
			.map(|p| {
				let range = pos..pos + p.len();

				pos += p.len() + 1;

				(p, range)
			}).filter(|(_, r)| !r.is_empty())
				.collect();

		Self {
			full_range: parameters.range(),
			params,
			ranges
		}
	}

	fn get_single(&self) -> Option<&str> {
		self.get("")
	}

	fn get(&self, key: &str) -> Option<&str> {
		self.params
			.get(key)
			.cloned()
	}

	fn get_single_strict(
		&self,
		err: impl FnOnce() -> Error<'t>
	) -> Result<&str, Error> {
		self.get_strict("", err)
	}

	fn get_strict(
		&self,
		key: &str,
		err: impl FnOnce() -> Error<'t>
	) -> Result<&str, Error> {
		self.get(key).ok_or_else(err)
	}

	fn get_single_range(&self) -> Range<usize> {
		self.get_range("")
	}

	fn get_range(&self, key: &str) -> Range<usize> {
		self.ranges
			.get(key)
			.cloned()
			.unwrap_or(self.full_range)
	}
}

enum Tag<'t> {
	Bold,
	Center,
	Code(Option<&'t str>),
	Color(&'t str),
	Image {
		value : Option<(u32, u32)>,
		width : Option<u32>,
		height: Option<u32>,
	},
	Italic,
	Left,
	List(bool),
	ListItem,
	Pre,
	Quote(Option<&'t str>),
	Right,
	Size(Option<u32>),
	Spoiler(Option<&'t str>),
	Strikethrough,
	Style {
		color: Option<&'t str>,
		size : Option<u32>,
	},
	Table,
	TableRow,
	TableCell(bool),
	Underline,
	Url(Option<&'t str>),
	Youtube,
}

impl<'t> Tag<'t> {
	fn parse(
		tag: &'t str,
		range: Range<usize>,
		parameters: TagParameters<'t>
	) -> Result<Self, Error> {
		let missing = |key|
			Error::param_missing(
				parameters.full_range,
				tag,
				key
			);
		let parse_val = |key, val: &str|
			val.parse::<u32>().map_err(|err|
				Error::param_parse(
					parameters.get_range(key),
					tag,
					key,
					val,
					err
				)
			);
		let parse = |key|
			parameters
				.get(key)
				.map(|val| parse_val(key, val))
				.swap();
				

		Ok(match tag.to_ascii_lowercase().as_str() {
			"b"       => Tag::Bold,
			"center"  => Tag::Center,
			"code"    => Tag::Code(parameters.get_single()),
			"color"   => Tag::Color(
				parameters.get_single_strict(|| missing("color"))?
			),
			"img"     => Tag::Image {
				value: {
					// This got ridiculous...
					parameters.get_single()
						.map(|dim|
							dim.split_once('x')
								.ok_or_else(||
									Error::param_invalid(
										parameters.get_single_range(),
										tag,
										"img",
										dim,
										"no dimension delimiter 'x' found"
									)
								)
						).swap()?
						.map(|(w, h)|
							Ok((
								parse_val("img", w)?,
								parse_val("img", h)?
							))
						)
						.swap()?
				},
				width : parse("width" )?,
				height: parse("height")?
			},
			"i"       => Tag::Italic,
			"left"    => Tag::Left,
			"list" |
			"ul"      => Tag::List(false),
			"ol"      => Tag::List(true),
			"li"      => Tag::ListItem,
			"pre"     => Tag::Pre,
			"quote"   => Tag::Quote(parameters.get_single()),
			"right"   => Tag::Right,
			"size"    => Tag::Size(parse("")?),
			"spoiler" => Tag::Spoiler(parameters.get_single()),
			"s"       => Tag::Strikethrough,
			"style"   => Tag::Style {
				color: parameters.get("color"),
				size : parse("size")?
			},
			"table"   => Tag::Table,
			"tr"      => Tag::TableRow,
			"th"      => Tag::TableCell(true),
			"td"      => Tag::TableCell(false),
			"u"       => Tag::Underline,
			"url"     => Tag::Url(parameters.get_single()),
			"youtube" => Tag::Youtube,
			_         => return Err(Error::unknown_tag(range, tag))
		})
	}

	fn build<N : BlockNode>(
		self,
		builder: NodeBuilder<N>,
		inner_range: Range<usize>,
		inner_blocks: Vec<Block<'t>>
	) -> Result<NodeBuilder<N>, Error> {
		fn build_block<'t, B : BlockNode>(
			nb: NodeBuilder<B>,
			inner_blocks: Vec<Block<'t>>
		) -> Result<NodeBuilder<B>, Error<'t>> {
			for block in inner_blocks {
				nb = parse_block(nb, block)?;
			}

			Ok(nb)
		}

		match self {
			Tag::Bold => builder.strong(|nb| build_generic(nb, inner_blocks)),
			Tag::Center => todo!(),
			Tag::Code(lang) => builder.code(|nb| build_code(nb, lang, inner_blocks)),
			Tag::Color(color) => todo!(),
			Tag::Image { value, width, height } => todo!(),
			Tag::Italic => builder.emphasis(|nb| build_block(nb, inner_blocks)),
			Tag::Left => build_block(builder, inner_blocks),
			Tag::List(ordered) => todo!(),
			Tag::ListItem => todo!(),
			Tag::Pre => todo!(),
			Tag::Quote(name) => todo!(),
			Tag::Right => todo!(),
			Tag::Size(size) => todo!(),
			Tag::Spoiler(name) => todo!(),
			Tag::Strikethrough => todo!(),
			Tag::Style { color, size } => todo!(),
			Tag::Table => todo!(),
			Tag::TableRow => todo!(),
			Tag::TableCell(alignment) => todo!(),
			Tag::Underline => todo!(),
			Tag::Url(url) => todo!(),
			Tag::Youtube => todo!(),
		}
	}
}

trait Swap<T, E> {
	fn swap(self) -> Result<Option<T>, E>;
}

// Wat?
impl<T, E> Swap<T, E> for Option<Result<T, E>> {
	fn swap(self) -> Result<Option<T>, E> {
		Ok(
			if let Some(result) = self {
				Some(result?)
			} else {
				None
			}
		)
	}
}*/
