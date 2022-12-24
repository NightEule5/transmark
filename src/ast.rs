pub mod bbcode;
mod builder;
pub use builder::*;

use markdown::mdast::Node;
use tl::VDom;
use tl::errors::ParseError as TlError;

use crate::{IntoMarkdownAst, IntoBBCodeAst, IntoHtmlDom, Error as InternalError, MarkdownFlavor, IntoMarkdownText, IntoBBCodeText, IntoHtmlText, IntoHtmlDomOwned};

use self::bbcode::Error as BbError;

/// A common AST for all supported markup languages. This is a wrapper around the
/// Markdown crate's AST; parsing is simply converting to Markdown.
pub struct TmDoc(pub Node);

pub enum ParseErrorKind {
	/// Some error ocurred during AST conversion.
	AstConversion,
}

pub struct ParseError<P> {
	pub kind: ParseErrorKind,
	pub inner: Option<InternalError<P>>
}

impl<P> ParseError<P> {
	fn ast_conversion(err: InternalError<P>) -> Self {
		Self { kind: ParseErrorKind::AstConversion, inner: Some(err) }
	}
}

impl TmDoc {
	pub fn parse_markdown(markdown: impl IntoMarkdownAst, flavor: MarkdownFlavor) -> Result<TmDoc, ParseError<!>> {
		let md = markdown.into_markdown_ast(flavor)
			.map_err(ParseError::ast_conversion)?;
		
		Ok(TmDoc(md))
	}

	pub fn parse_bbcode(bbcode: impl IntoBBCodeAst) -> Result<TmDoc, ParseError<BbError>> {
		bbcode.into_bbcode_ast().map_err(ParseError::ast_conversion)
	}

	pub fn parse_html<'d>(html: impl IntoHtmlDom<'d>) -> Result<TmDoc, ParseError<TlError>> {
		todo!()
	}

	pub fn parse_html_owned(html: impl IntoHtmlDomOwned) -> Result<TmDoc, ParseError<TlError>> {
		todo!()
	}

	fn to_md(self) -> Node { self.0 }

	fn to_html<'d>(self) -> VDom<'d> {
		todo!()
	}

	fn to_md_text(self) -> String {
		self.to_md().into_markdown_text()
	}

	fn to_bb_text(self) -> String {
		self.into_bbcode_text()
	}

	fn to_html_text(self) -> String {
		self.to_html().into_html_text()
	}

	fn to_plain_text(self) -> String {
		todo!()
	}
}

// 1:1 Markdown conversion
impl IntoMarkdownAst for TmDoc {
	fn into_markdown_ast(self, _: MarkdownFlavor) -> Result<Node, InternalError<!>> {
		Ok(self.to_md())
	}
}

impl IntoMarkdownText for TmDoc {
	fn into_markdown_text(self) -> String {
		self.to_md_text()
	}
}

impl IntoBBCodeAst for TmDoc {
	fn into_bbcode_ast(self) -> Result<TmDoc, InternalError<BbError>> { Ok(self) }
}

impl IntoBBCodeText for TmDoc {
	fn into_bbcode_text(self) -> String {
		self.to_bb_text()
	}
}

impl<'d> IntoHtmlDom<'d> for TmDoc {
	fn into_html_dom(self) -> Result<VDom<'d>, InternalError<TlError>> { Ok(self.to_html()) }
}

impl IntoHtmlText for TmDoc {
	fn into_html_text(self) -> String {
		self.to_html_text()
	}
}
