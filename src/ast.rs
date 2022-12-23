pub mod bbcode;

use markdown::mdast::{Root, Node};
use tl::VDom;
use tl::errors::ParseError as TlError;

use crate::ast::bbcode::BBDoc;
use crate::{IntoMarkdownAst, IntoBBCodeAst, IntoHtmlDom, Error, MarkdownFlavor, IntoMarkdownText, IntoBBCodeText, IntoHtmlText, IntoHtmlDomOwned};

use self::bbcode::Error as BbError;

/// A common AST for all supported markup languages. This is a wrapper around the
/// Markdown crate's AST; parsing is simply converting to Markdown.
pub struct TmDoc(pub Root);

impl TmDoc {
	pub fn parse_markdown(markdown: impl IntoMarkdownAst) -> TmDoc {
		todo!()
	}

	pub fn parse_bbcode(bbcode: impl IntoBBCodeAst) -> TmDoc {
		todo!()
	}

	pub fn parse_html<'d>(html: impl IntoHtmlDom<'d>) -> TmDoc {
		todo!()
	}

	pub fn parse_html_owned(html: impl IntoHtmlDomOwned) -> TmDoc {
		todo!()
	}

	fn to_md(self) -> Node {
		let Self(root) = self;

		Node::Root(root)
	}

	fn to_bb(self) -> BBDoc {
		todo!()
	}

	fn to_html<'d>(self) -> VDom<'d> {
		todo!()
	}

	fn to_md_text(self) -> String {
		self.to_md().into_markdown_text()
	}

	fn to_bb_text(self) -> String {
		self.to_bb().into_bbcode_text()
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
	fn into_markdown_ast(self, _: MarkdownFlavor) -> Result<Node, Error<!>> {
		Ok(self.to_md())
	}
}

impl IntoMarkdownText for TmDoc {
	fn into_markdown_text(self) -> String {
		self.to_md_text()
	}
}

impl IntoBBCodeAst for TmDoc {
	fn into_bbcode_ast(self) -> Result<BBDoc, Error<BbError>> { Ok(self.to_bb()) }
}

impl IntoBBCodeText for TmDoc {
	fn into_bbcode_text(self) -> String {
		self.to_bb_text()
	}
}

impl<'d> IntoHtmlDom<'d> for TmDoc {
	fn into_html_dom(self) -> Result<VDom<'d>, Error<TlError>> { Ok(self.to_html()) }
}

impl IntoHtmlText for TmDoc {
	fn into_html_text(self) -> String {
		self.to_html_text()
	}
}
