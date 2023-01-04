#![feature(
	assert_matches,
	exclusive_range_pattern,
	never_type,
	option_zip,
	trait_alias,
)]

pub mod ast;
pub(crate) mod util;
pub mod markdown_text;
pub use ast::TmDoc;
use markdown::{to_mdast, ParseOptions};
use tl::VDomGuard;

use std::io::{Read, Error as IoError, BufReader};

use ast::bbcode::{Error as BbError, self};
use markdown::mdast::Node;
use tl::{VDom, errors::ParseError as TlError};

pub enum Error<P> {
	Parse(P),
	Read (IoError),
}

pub enum MarkdownFlavor {
	CommonMark,
	GFM,
	Custom(ParseOptions),
}

impl MarkdownFlavor {
	pub fn options(self) -> ParseOptions {
		match self {
			Self::CommonMark  => ParseOptions::default(),
			Self::GFM         => ParseOptions::gfm(),
			Self::Custom(opt) => opt,
		}
	}
}

// AST traits

pub trait IntoCommonAst<E> {
	fn into_common_ast(self) -> Result<TmDoc, E>;
}

/// Facilitates conversion or parsing into a Markdown [Node].
pub trait IntoMarkdownAst {
	/// Converts self into a Markdown [Node].
	fn into_markdown_ast(self, flavor: MarkdownFlavor) -> Result<Node, Error<!>>;
}

/// Facilitates conversion or parsing into a [TmDoc] representation of BBCode.
pub trait IntoBBCodeAst {
	/// Converts self into a [TmDoc].
	fn into_bbcode_ast<'t>(self) -> Result<TmDoc, Error<BbError<'t>>>;
}

/// Facilitates conversion or parsing into a [VDom] representation of HTML.
pub trait IntoHtmlDom<'d> {
	/// Converts self into a [VDom].
	fn into_html_dom(self) -> Result<VDom<'d>, Error<TlError>>;
}

/// Facilitates conversion or parsing into a [VDomGuard] representation of HTML.
pub trait IntoHtmlDomOwned {
	/// Converts self into an owned [VDomGuard].
	unsafe fn into_html_dom_owned(self) -> Result<VDomGuard, Error<TlError>>;
}

// Text traits

pub trait IntoMarkdownText {
	fn into_markdown_text(self) -> String;
}

pub trait IntoBBCodeText {
	fn into_bbcode_text(self) -> String;
}

pub trait IntoHtmlText {
	fn into_html_text(self) -> String;
}

// Default AST conversions

impl IntoMarkdownAst for Node {
	fn into_markdown_ast(self, _: MarkdownFlavor) -> Result<Node, Error<!>> { Ok(self) }
}

impl IntoMarkdownAst for &str {
	fn into_markdown_ast(self, flavor: MarkdownFlavor) -> Result<Node, Error<!>> {
		Ok(to_mdast(self, &flavor.options()).unwrap())
	}
}

impl IntoMarkdownAst for String {
	fn into_markdown_ast(self, flavor: MarkdownFlavor) -> Result<Node, Error<!>> {
		self.as_str().into_markdown_ast(flavor)
	}
}

impl<R : Read> IntoMarkdownAst for BufReader<R> {
	fn into_markdown_ast(mut self, flavor: MarkdownFlavor) -> Result<Node, Error<!>> {
		let mut text = String::new();

		self.read_to_string(&mut text).map_err(Error::Read)?;

		text.into_markdown_ast(flavor)
	}
}

/*impl IntoBBCodeAst for &str {
	fn into_bbcode_ast<'t>(self) -> Result<TmDoc, Error<BbError<'t>>> {
		bbcode::parse(self).map_err(Error::Parse)
	}
}*/

/*impl IntoBBCodeAst for String {
	fn into_bbcode_ast<'t>(self) -> Result<TmDoc, Error<BbError<'t>>> {
		bbcode::parse(&self).map_err(Error::Parse)
	}
}*/

/*impl<R : Read> IntoBBCodeAst for BufReader<R> {
	fn into_bbcode_ast<'t>(mut self) -> Result<TmDoc, Error<BbError<'t>>> {
		let mut text = String::new();

		self.read_to_string(&mut text).map_err(Error::Read)?;

		bbcode::parse(&text).map_err(Error::Parse)
	}
}*/

impl<'d> IntoHtmlDom<'d> for VDom<'d> {
	fn into_html_dom(self) -> Result<VDom<'d>, Error<TlError>> { Ok(self) }
}

impl IntoHtmlDomOwned for VDomGuard {
	unsafe fn into_html_dom_owned(self) -> Result<VDomGuard, Error<TlError>> { Ok(self) }
}

impl<'d> IntoHtmlDom<'d> for &'d str {
	fn into_html_dom(self) -> Result<VDom<'d>, Error<TlError>> {
		tl::parse(self, tl::ParserOptions::default())
			.map_err(Error::Parse)
	}
}

impl IntoHtmlDomOwned for String {
	unsafe fn into_html_dom_owned(self) -> Result<VDomGuard, Error<TlError>> {
		tl::parse_owned(self, tl::ParserOptions::default())
			.map_err(Error::Parse)
	}
}

impl<R : Read> IntoHtmlDomOwned for BufReader<R> {
	unsafe fn into_html_dom_owned(mut self) -> Result<VDomGuard, Error<TlError>> {
		let mut text = String::new();

		self.read_to_string(&mut text).map_err(Error::Read)?;

		text.into_html_dom_owned()
	}
}

// Default text conversions

impl IntoMarkdownText for Node {
	fn into_markdown_text(self) -> String { self.to_string() }
}

impl<'d> IntoHtmlText for VDom<'d> {
	fn into_html_text(self) -> String { self.outer_html() }
}

impl IntoHtmlText for VDomGuard {
	fn into_html_text(self) -> String {
		self.get_ref().outer_html()
	}
}
