use std::collections::HashMap;
use std::fmt::write;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::result::Result as StdResult;
use std::{fmt, mem};
use std::ops::Range;

use lazy_static::lazy_static;
use markdown::mdast::{Root, Html, Code, Link, Image, BlockQuote, List, Strong};
use regex_macro::regex;

use crate::util::Zip;
use crate::{TmDoc, IntoMarkdownText};
use crate::ast::{NodeBuilder, BlockNode, TextNode, BuildFn, AsNode};
use crate::markdown_text::escape_markdown;

use super::tokenizer::{Fragment, FragmentStream, split_fragments, TextFragment, Tag};

type Result<T> = StdResult<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ErrorKind {
	UnknownTag,
	UnopenedTag,
	UnclosedTag,
	UnexpectedTag(String, Range<usize>),
	MissingParam(String),
	MissingInner,
}

impl fmt::Display for ErrorKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::UnknownTag  => write!(f, "unknown tag" ),
			Self::UnopenedTag => write!(f, "unopened tag"),
			Self::UnclosedTag => write!(f, "unclosed tag"),
			Self::UnexpectedTag(
				name,
				Range {
					start,
					end
				}
			) => write!(f, "unexpected tag inside {name} tag from {start} to {end}"),
			Self::MissingParam(name) => write!(f, "missing required parameter {name}"),
			Self::MissingInner       => write!(f, "missing required inner text"),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Error {
	value: String,
	range: Range<usize>,
	kind: ErrorKind,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self {
			value,
			range: Range { start, end },
			kind
		} = self;

		write!(f, "BBCode parse error (\"{value}\" from {start} to {end}): {kind}.")
	}
}

#[derive(Clone, Default)]
enum Inner<'t> {
	#[default]
	None,
	Text(&'t str),
	Tree(Vec<Node<'t>>)
}

impl<'t> Inner<'t> {
	fn text(self) -> Option<&'t str> {
		if let Self::Text(value) = self {
			Some(value)
		} else {
			None
		}
	}

	fn tree(self) -> Option<Vec<Node<'t>>> {
		if let Self::Tree(value) = self {
			Some(value)
		} else {
			None
		}
	}

	fn take(&mut self) -> Self {
		mem::take(self)
	}

	fn take_text(&mut self) -> Option<&'t str> {
		self.take().text()
	}

	fn take_tree(&mut self) -> Option<Vec<Node<'t>>> {
		self.take().tree()
	}

	fn replace_text(&mut self, value: &'t str) -> Self {
		mem::replace(self, Self::Text(value))
	}

	fn replace_tree(&mut self, value: Vec<Node<'t>>) -> Self {
		mem::replace(self, Self::Tree(value))
	}

	fn build<N : BlockNode>(self, mut node: NodeBuilder<N>) -> Result<NodeBuilder<N>> {
		Ok(match self {
			Inner::None => node,
			Inner::Text(value) => node.text(escape_markdown(value)),
			Inner::Tree(value) => {
				for inner_node in value {
					node = inner_node.build(node)?
				}

				node
			}
		})
	}
}

#[derive(Clone)]
struct NodeTag<'t> {
	name: &'t str,
	name_range: Range<usize>,
	params: HashMap<&'t str, &'t str>,
	param_range: Range<usize>,
}

impl<'t> NodeTag<'t> {
	fn from_fragment(tag: Tag<'t>) -> Self {
		let Tag {
			name : TextFragment(name,  name_range),
			param: TextFragment(param, param_range)
		} = tag;

		Self::new(name, name_range, param, param_range)
	}

	fn new(
		name: &'t str,
		name_range: Range<usize>,
		param: &'t str,
		param_range: Range<usize>
	) -> Self {
		Self {
			name,
			name_range,
			params: Self::split(param),
			param_range
		}
	}

	fn split(param: &'t str) -> HashMap<&'t str, &'t str> {
		param.split(' ')
			.filter(|p| !p.is_empty())
			.filter_map(|pair|
				pair.split_once('=')
					.and_then(|(k, v)|
						Some((
							k,
							v.strip_prefix('"')?
							 .strip_suffix('"')?
						))
					)
			)
			.collect()
	}

	fn name(&self) -> (&'t str, Range<usize>) { (self.name, self.name_range.clone()) }
}

#[derive(Clone)]
struct Node<'t> {
	tag: Option<NodeTag<'t>>,
	inner: Inner<'t>
}

impl<'t> Node<'t> {
	fn tag(tag: Tag) -> Self {
		Self {
			tag: Some(NodeTag::from_fragment(tag)),
			inner: Inner::None
		}
	}

	fn text(value: &'t str) -> Self {
		Self { tag: None, inner: Inner::Text(value) }
	}

	fn root() -> Self {
		Self { tag: None, inner: Inner::Tree(Vec::with_capacity(8)) }
	}

	fn put_text(&mut self, value: &'t str) {
		match self.inner {
			Inner::None => {
				self.inner.replace_text(value);
			}
			Inner::Text(existing) => {
				let tree = Vec::new();
				tree.push(Self::text(existing));
				tree.push(Self::text(value));

				self.inner.replace_tree(tree);
			}
			Inner::Tree(tree) => {
				tree.push(Self::text(value));
			}
		}
	}

	fn parse(&mut self, mut fragments: FragmentStream<'t>) -> Result<FragmentStream<'t>> {
		let end_tag = self.tag.as_ref().map(NodeTag::name);

		while let Some(fragment) = fragments.next() {
			match fragment {
				Fragment::Text(TextFragment(value, _)) => {
					self.put_text(value);
				}
				Fragment::StartTag(tag) => {
					fragments = Self::tag(tag).parse(fragments)?;
				}
				Fragment::EndTag(
					TextFragment(value, range)
				) => if let Some((tag, range)) = end_tag {
					if value == tag {
						break
					} else {
						return Err(
							Error {
								value: tag.to_string(),
								range,
								kind: ErrorKind::UnclosedTag
							}
						)
					}
				} else {
					return Err(
						Error {
							value: value.to_string(),
							range,
							kind: ErrorKind::UnopenedTag
						}
					)
				}
			}
		}

		Ok(fragments)
	}

	fn build<N : BlockNode>(&self, node: NodeBuilder<N>) -> Result<NodeBuilder<N>> {
		if let Some(
			NodeTag {
				name,
				name_range,
				params,
				param_range
			}
		) = self.tag {
			let mut style = |style: String| {
				params.clear();
				params.insert("style", &style);
			};

			let align = |alignment| style(format!("text-align: {alignment};"));
			let color = |color    | style(format!("color: {color};"         ));
			let size  = |size     | style(format!("font-size: {size};"      ));
	
			let color_size = |color, size| style(
				format!("color: {color}; font-size: {size};")
			);

			fn build_html<N : BlockNode>(
				node: NodeBuilder<N>,
				tag: &str,
				inner: Inner,
				params: HashMap<&str, &str>,
				build_header: impl BuildFn<Html, Error>
			) -> Result<NodeBuilder<N>> {
				node.html(|node|
					build_header(node)?
						.build_value(tag, params, inner)
				)
			}
	
			let build_span_html = || build_html(node, "span", self.inner, params, Ok);

			match name {
				"b"       => node.strong(|nb| self.build(nb)),
				"center" |
				"left"   |
				"right"   => {
					align(name);
					build_span_html()
				}
				"code"    => self.build_code(node, self.inner, params),
				"color"   => if let Some(_) = params.remove("color").map(color) {
					build_span_html()
				} else {
					self.inner.build(node)
				},
				"i"       => node.emphasis(self.inner),
				"img"     => self.build_img(node, self.inner, params),
				"ol"      => self.build_list(node, self.inner, true),
				"pre"     => node.inline_code(|nb| self.build_text(nb, self.inner)),
				"quote"   => {
					node = node.block_quote(self.inner)?;

					Ok(
						if let Some(attribution) = params.remove("quote").map(escape_markdown) {
							node.text(format!("—{attribution}"))
						} else {
							node
						}
					)
				}
				"s"       => node.delete(|nb| self.build(nb)),
				"size"    => if let Some(_) = params.remove("size").map(size) {
					build_span_html()
				} else {
					self.inner.build(node)
				},
				"spoiler" => {
					build_html(
						node,
						"details",
						self.inner,
						HashMap::new(),
						|node: NodeBuilder<Html>|
							Ok(
								if let Some(summary) = params
									.remove("spoiler")
									.map(escape_markdown) {
									node.append_value(
										format!("<summary>{summary}</summary>")
									)
								} else {
									node
								}
							)
					)
				}
				"style"   => {
					let color_val = params.remove("color");
					let  size_val = params.remove("size" );

					if let Some(color_val) = color_val {
						if let Some(size_val) = size_val {
							color_size(color_val, size_val);
						} else {
							color(color_val);
						}
					} else {
						if let Some(size_val) = size_val {
							size(size_val);
						} else { }
					}

					build_span_html()
				}
				"table"   => todo!(),
				"td"      => todo!(),
				"th"      => todo!(),
				"tr"      => todo!(),
				"u"       => build_html(node, "u", self.inner, HashMap::new(), Ok),
				"ul" |
				"list"    => self.build_list(node, self.inner, false),
				"url"     => self.build_url(node, self.inner, params, |s| s),
				"youtube" => self.build_url(node, self.inner, HashMap::new(), |id|
					format!("https://youtube.com/watch?v={id}")
				),
				_         => Err(
					Error {
						value: name.to_string(),
						range: name_range,
						kind: ErrorKind::UnknownTag
					}
				)
			}
		} else {
			self.inner.build(node)
		}
	}

	fn get_inner_text(&self, inner: Inner) -> Result<String> {
		Ok (match inner {
			Inner::None        => "".to_string(),
			Inner::Text(value) => value.to_string(),
			Inner::Tree(value) => {
				let NodeTag {
					name: tag,
					name_range: range,
					params: _,
					param_range: _
				} = self.tag.unwrap();

				let NodeTag {
					name: unexp_tag,
					name_range: unexp_range,
					params: _,
					param_range: _
				} = value.iter().find_map(|v| v.tag).unwrap();

				return Err(
					Error {
						value: tag.to_string(),
						range,
						kind: ErrorKind::UnexpectedTag(
							unexp_tag.to_string(),
							unexp_range
						)
					}
				)
			}
		})
	}

	fn build_text<N : TextNode>(
		&self,
		node: NodeBuilder<N>,
		inner: Inner
	) -> Result<NodeBuilder<N>> {
		let text = self.get_inner_text(inner)?;

		Ok(node.set_value(text))
	}

	fn build_into_html(
		&self,
		node: NodeBuilder<Html>,
		inner: Inner,
		prefix: &str,
		suffix: &str
	) -> Result<NodeBuilder<Html>> {
		let md = match inner {
			Inner::None        => "".to_string(),
			Inner::Text(value) => escape_markdown(value),
			Inner::Tree(value) => {
				let mut fake_root = NodeBuilder::new();

				for node in value {
					fake_root = node.build(fake_root)?;
				}

				fake_root.build().into_markdown_text()
			}
		};

		Ok(node.set_value(format!("{prefix}\n{md}\n{suffix}")))
	}

	fn build_code<N : BlockNode>(
		&self,
		node: NodeBuilder<N>,
		inner: Inner,
		mut params: HashMap<&str, &str>
	) -> Result<NodeBuilder<N>> {
		let code = self.get_inner_text(inner)?;
		let lang = params.remove("code").map(String::from);

		node.code(|node| Ok(node.set_value(code).set_lang(lang)))
	}

	fn build_url<N : BlockNode>(
		&self,
		node: NodeBuilder<N>,
		inner: Inner,
		mut params: HashMap<&str, &str>,
		map_url: impl FnOnce(String) -> String
	) -> Result<NodeBuilder<N>> {
		let inner = self.get_inner_text(inner)?;
		let mut title = Some(inner);
		let mut url = if let Some(url) = params.remove("url").map(String::from) {
			url
		} else {
			title.take().unwrap()
		};
		url = map_url(url);

		node.link(|node| Ok(node.set_url(url).set_title(title)))
	}

	fn build_img<N : BlockNode>(
		&self,
		node: NodeBuilder<N>,
		inner: Inner,
		mut params: HashMap<&str, &str>
	) -> Result<NodeBuilder<N>> {
		let url    = self.get_inner_text(inner)?;
		let alt    = params.get("alt"   ).map(ToString::to_string);
		let title  = params.get("title" ).map(ToString::to_string);
		let width  = params.get("width" ).cloned();
		let height = params.get("height").cloned();
		let dim    = params.remove("img")
			.and_then(|p| p.split_once('x'))
			.or_else(||
				width.zip(height)
					.or_else(||
						height.zip_with(
							width,
							|h, w| (w, h)
						)
					)
			);

		if let Some((w, h)) = dim {
			params.insert("width",  w);
			params.insert("height", h);
			params.insert("src", &url);

			node.link(|nb|
				Ok(
					nb.set_title_html(
						NodeBuilder::default()
							.build_value("img", params, Ok)?
					).set_url(url)
				)
			)
		} else {
			node.image(|nb|
				Ok(
					nb.set_url(url)
				  	  .set_alt(alt.unwrap_or(String::from("")))
				  	  .set_title(title)
				)
			)
		}
	}

	fn build_list<N : BlockNode>(
		&self,
		node: NodeBuilder<N>,
		inner: Inner,
		ordered: bool
	) -> Result<NodeBuilder<N>> {
		node.list(|mut node: NodeBuilder<List>| {
			node =
				node.set_ordered(ordered)
					.set_start(ordered.then_some(1))
					.set_spread(false);

			let li_shorthand = regex!(r"^\s*\[*].*$");
			let items = match inner {
				Inner::None        => Vec::new(),
				Inner::Text(value) => {
					li_shorthand.find_iter(value)
						.map(|s| (&s.as_str().trim()[3..]).trim_start())
						.map(Node::text)
						.collect()
				}
				Inner::Tree(value) => value
			};

			for item in items {
				match item.inner {
					Inner::None => { }
					Inner::Text(value) => todo!(),
					Inner::Tree(value) => todo!(),
				}
			}
			Ok(node)
		})
	}
}

#[cfg(test)]
mod tests {
	const SIMPLE_BLOCK      : &str = r"[tag]text[/tag]";
	const BLOCK_WITH_VALUE  : &str = r"[tag=value]text[/tag]";
	const BLOCK_WITH_PARAMS : &str = r#"[tag abc="val1" def="val2"]text[/tag]"#;
	const BLOCK_WITH_BOTH   : &str = r#"[tag=value abc="val1" def="val2"]text[/tag]"#;
	const TEXT_PREFIX_BLOCK : &str = r"You can make [b]bold text![/b]";
	const TEXT_SUFFIX_BLOCK : &str = r"[u]Underline[/u] text with the 'u' tag!";
	const TEXT_INFIX_BLOCK  : &str = "Paragraph 1\n[h1]Heading[/h1]\nParagraph 2";
	const NESTED_BLOCK      : &str = r"[s]Stricken and [i]italicized[/i] text[/s]";
	const OUT_OF_SCOPE_BLOCK: &str = r"[size=14]oops! Your [quote] is out of scope![/size][/quote]";
}
