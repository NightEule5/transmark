mod node_traits;
pub(crate) use node_traits::*;

use std::assert_matches::assert_matches;

use markdown::mdast::*;

use crate::TmDoc;

pub trait BuildFn<N : AsNode, E> = FnOnce(NodeBuilder<N>) -> Result<NodeBuilder<N>, E>;

pub struct NodeBuilder<N : AsNode> { node: N }

impl NodeBuilder<Root> {
	pub fn new() -> Self {
		Self { node: new_root() }
	}

	pub fn build(self) -> TmDoc {
		TmDoc(self.node.as_node())
	}
}

impl NodeBuilder<Code> {
	pub fn set_lang(mut self, lang: Option<String>) -> Self {
		self.node.lang = lang;
		self
	}
}

impl NodeBuilder<Heading> {
	pub fn set_depth(mut self, depth: u8) -> Self {
		assert_matches!(depth, 1..6);

		self.node.depth = depth;
		self
	}
}

impl NodeBuilder<List> {
	pub fn set_ordered(mut self, ordered: bool) -> Self {
		self.node.ordered = ordered;
		self
	}

	pub fn set_start(mut self, start: Option<u32>) -> Self {
		self.node.start = start;
		self
	}

	pub fn set_spread(mut self, spread: bool) -> Self {
		self.node.spread = spread;
		self
	}

	fn append(mut self, node: impl AsNode) -> Self {
		self.node.children.push(node.as_node());
		self
	}

	pub fn item<E>(self, build: impl BuildFn<ListItem, E>) -> Result<Self, E> {
		Ok(self.append(build(new_list_item())?.node()))
	}
}

impl NodeBuilder<ListItem> {
	pub fn set_spread(mut self, spread: bool) -> Self {
		self.node.spread = spread;
		self
	}

	pub fn set_checked(mut self, checked: Option<bool>) -> Self {
		self.node.checked = checked;
		self
	}
}

impl NodeBuilder<Table> {
	pub fn align_column(mut self, alignment: AlignKind) -> Self {
		self.node.align.push(alignment);
		self
	}

	pub fn row<E>(self, build: impl BuildFn<TableRow, E>) -> Result<Self, E> {
		Ok(self.append(build(new_table_row())?.node()))
	}
}

impl NodeBuilder<TableRow> {
	pub fn row<E>(self, build: impl BuildFn<TableCell, E>) -> Result<Self, E> {
		Ok(self.append(build(new_table_cell())?.node()))
	}
}
impl<N : AsNode> NodeBuilder<N> {
	pub fn node(self) -> N { self.node }
}

impl<N : BlockNode> NodeBuilder<N> {
	pub fn append(mut self, node: impl AsNode) -> Self {
		self.node.append(node.as_node());
		self
	}

	pub fn block_quote<E>(self, build: impl BuildFn<BlockQuote, E>) -> Result<Self, E> {
		Ok(self.append(build(new_block_quote())?.node()))
	}

	pub fn line_break(self) -> Self {
		self.append(new_break())
	}

	pub fn code<E>(self, build: impl BuildFn<Code, E>) -> Result<Self, E> {
		Ok(self.append(build(new_code())?.node()))
	}

	pub fn definition<E>(self, build: impl BuildFn<Definition, E>) -> Result<Self, E> {
		Ok(self.append(build(new_def())?.node()))
	}

	pub fn delete<E>(self, build: impl BuildFn<Delete, E>) -> Result<Self, E> {
		Ok(self.append(build(new_delete())?.node()))
	}

	pub fn emphasis<E>(self, build: impl BuildFn<Emphasis, E>) -> Result<Self, E> {
		Ok(self.append(build(new_emph())?.node()))
	}

	pub fn footnote_definition<E>(self, build: impl BuildFn<FootnoteDefinition, E>) -> Result<Self, E> {
		Ok(self.append(build(new_foot_def())?.node()))
	}

	pub fn footnote_reference<E>(self, build: impl BuildFn<FootnoteReference, E>) -> Result<Self, E> {
		Ok(self.append(build(new_foot_ref())?.node()))
	}

	pub fn heading<E>(self, build: impl BuildFn<Heading, E>) -> Result<Self, E> {
		Ok(self.append(build(new_heading())?.node()))
	}

	pub fn html<E>(self, build: impl BuildFn<Html, E>) -> Result<Self, E> {
		Ok(self.append(build(new_html())?.node()))
	}

	pub fn image<E>(self, build: impl BuildFn<Image, E>) -> Result<Self, E> {
		Ok(self.append(build(new_image())?.node()))
	}

	pub fn image_reference<E>(self, build: impl BuildFn<ImageReference, E>) -> Result<Self, E> {
		Ok(self.append(build(new_image_ref())?.node()))
	}

	pub fn inline_code<E>(self, build: impl BuildFn<InlineCode, E>) -> Result<Self, E> {
		Ok(self.append(build(new_inline_code())?.node()))
	}

	pub fn inline_math<E>(self, build: impl BuildFn<InlineMath, E>) -> Result<Self, E> {
		Ok(self.append(build(new_inline_math())?.node()))
	}

	pub fn link<E>(self, build: impl BuildFn<Link, E>) -> Result<Self, E> {
		Ok(self.append(build(new_link())?.node()))
	}

	pub fn link_reference<E>(self, build: impl BuildFn<LinkReference, E>) -> Result<Self, E> {
		Ok(self.append(build(new_link_ref())?.node()))
	}

	pub fn list<E>(self, build: impl BuildFn<List, E>) -> Result<Self, E> {
		Ok(self.append(build(new_list())?.node()))
	}

	pub fn math<E>(self, build: impl BuildFn<Math, E>) -> Result<Self, E> {
		Ok(self.append(build(new_math())?.node()))
	}

	pub fn paragraph<E>(self, build: impl BuildFn<Paragraph, E>) -> Result<Self, E> {
		Ok(self.append(build(new_para())?.node()))
	}

	pub fn strong<E>(self, build: impl BuildFn<Strong, E>) -> Result<Self, E> {
		Ok(self.append(build(new_strong())?.node()))
	}

	pub fn table<E>(self, build: impl BuildFn<Table, E>) -> Result<Self, E> {
		Ok(self.append(build(new_table())?.node()))
	}

	pub fn text(self, value: String) -> Self {
		self.append(new_text(value))
	}

	pub fn thematic_break(self) -> Self {
		self.append(new_them_break())
	}
}

// Workaround for:
// the type parameter `C` is not constrained by the impl trait, self type, or
// predicates unconstrained type parameter
pub trait ParentNodeBuilder<N : ParentNode<C>, C : AsNode> {
	fn append(self, node: C) -> Self;
}

impl<N : ParentNode<C>, C : AsNode> ParentNodeBuilder<N, C> for NodeBuilder<N> {
	fn append(mut self, node: C) -> Self {
		self.node.append(node);
		self
	}
}

impl<N : TextNode> NodeBuilder<N> {
	pub fn set_value(mut self, text: String) -> Self {
		self.node.set_value(text);
		self
	}
}

impl<N : DefRefNode> NodeBuilder<N> {
	pub fn set_id(mut self, id: String) -> Self {
		self.node.set_id(id);
		self
	}

	pub fn set_label(mut self, label: Option<String>) -> Self {
		self.node.set_label(label);
		self
	}
}

impl<N : RefKindNode> NodeBuilder<N> {
	pub fn set_ref_kind(mut self, kind: ReferenceKind) -> Self {
		self.node.set_ref_kind(kind);
		self
	}
}

impl<N : LinkNode> NodeBuilder<N> {
	pub fn set_title(mut self, title: Option<String>) -> Self {
		self.node.set_title(title);
		self
	}

	pub fn set_url(mut self, url: String) -> Self {
		self.node.set_url(url);
		self
	}
}

impl<N : AltNode> NodeBuilder<N> {
	pub fn set_alt(mut self, alt: String) -> Self {
		self.node.set_alt(alt);
		self
	}
}

fn new_root() -> Root {
	Root { children: vec![], position: None }
}

fn new_block_quote() -> NodeBuilder<BlockQuote> {
	NodeBuilder {
		node: BlockQuote { children: vec![], position: None }
	}
}

fn new_break() -> Break { Break { position: None } }

fn new_code() -> NodeBuilder<Code> {
	NodeBuilder {
		node: Code {
			value: String::new(),
			position: None,
			lang: None,
			meta: None
		}
	}
}

fn new_def() -> NodeBuilder<Definition> {
	NodeBuilder {
		node: Definition {
			position: None,
			url: String::new(),
			title: None,
			identifier: String::new(),
			label: None
		}
	}
}

fn new_delete() -> NodeBuilder<Delete> {
	NodeBuilder {
		node: Delete { children: vec![], position: None }
	}
}

fn new_emph() -> NodeBuilder<Emphasis> {
	NodeBuilder {
		node: Emphasis { children: vec![], position: None }
	}
}

fn new_foot_def() -> NodeBuilder<FootnoteDefinition> {
	NodeBuilder {
		node: FootnoteDefinition {
			children: vec![],
			position: None,
			identifier: String::new(),
			label: None
		}
	}
}

fn new_foot_ref() -> NodeBuilder<FootnoteReference> {
	NodeBuilder {
		node: FootnoteReference {
			position: None,
			identifier: String::new(),
			label: None
		}
	}
}

fn new_heading() -> NodeBuilder<Heading> {
	NodeBuilder {
		node: Heading {
			children: vec![],
			position: None,
			depth: 1
		}
	}
}

fn new_html() -> NodeBuilder<Html> {
	NodeBuilder {
		node: Html { value: String::new(), position: None }
	}
}

fn new_image() -> NodeBuilder<Image> {
	NodeBuilder {
		node: Image {
			position: None,
			alt: String::new(),
			url: String::new(),
			title: None
		}
	}
}

fn new_image_ref() -> NodeBuilder<ImageReference> {
	NodeBuilder {
		node: ImageReference {
			position: None,
			alt: String::new(),
			reference_kind: ReferenceKind::Shortcut,
			identifier: String::new(),
			label: None
		}
	}
}

fn new_inline_code() -> NodeBuilder<InlineCode> {
	NodeBuilder {
		node: InlineCode { value: String::new(), position: None }
	}
}

fn new_inline_math() -> NodeBuilder<InlineMath> {
	NodeBuilder {
		node: InlineMath { value: String::new(), position: None }
	}
}

fn new_link() -> NodeBuilder<Link> {
	NodeBuilder {
		node: Link {
			children: vec![],
			position: None,
			url: String::new(),
			title: None
		}
	}
}

fn new_link_ref() -> NodeBuilder<LinkReference> {
	NodeBuilder {
		node: LinkReference {
			children: vec![],
			position: None,
			reference_kind: ReferenceKind::Shortcut,
			identifier: String::new(),
			label: None
		}
	}
}

fn new_list() -> NodeBuilder<List> {
	NodeBuilder {
		node: List {
			children: vec![],
			position: None,
			ordered: false,
			start: None,
			spread: false
		}
	}
}

fn new_list_item() -> NodeBuilder<ListItem> {
	NodeBuilder {
		node: ListItem {
			children: vec![],
			position: None,
			spread: false,
			checked: None
		}
	}
}

fn new_math() -> NodeBuilder<Math> {
	NodeBuilder {
		node: Math {
			value: String::new(),
			position: None,
			meta: None
		}
	}
}

fn new_para() -> NodeBuilder<Paragraph> {
	NodeBuilder {
		node: Paragraph { children: vec![], position: None }
	}
}

fn new_strong() -> NodeBuilder<Strong> {
	NodeBuilder {
		node: Strong { children: vec![], position: None }
	}
}

fn new_table() -> NodeBuilder<Table> {
	NodeBuilder {
		node: Table {
			children: vec![],
			position: None,
			align: vec![]
		}
	}
}

fn new_table_cell() -> NodeBuilder<TableCell> {
	NodeBuilder {
		node: TableCell { children: vec![], position: None }
	}
}

fn new_table_row() -> NodeBuilder<TableRow> {
	NodeBuilder {
		node: TableRow { children: vec![], position: None }
	}
}

fn new_text(value: String) -> Text {
	Text { value, position: None }
}

fn new_them_break() -> ThematicBreak {
	ThematicBreak { position: None }
}
