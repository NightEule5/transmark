mod node_traits;
pub(crate) use node_traits::*;

use std::assert_matches::assert_matches;
use std::collections::HashMap;

use markdown::mdast::*;

use crate::TmDoc;

pub trait BuildFn<N : AsNode, E> = FnOnce(NodeBuilder<N>) -> Result<NodeBuilder<N>, E>;

/// Populates a [NodeBuilder].
pub trait Populate<N : AsNode, E> {
	/// Returns a populated [NodeBuilder] wrapped in a [Result].
	fn populate(self) -> Result<NodeBuilder<N>, E>;
}

impl<N : AsNode, E, P> Populate<N, E> for P where P : FnOnce() -> Result<NodeBuilder<N>, E> {
	fn populate(self) -> Result<NodeBuilder<N>, E> { self() }
}

pub struct NodeBuilder<N : AsNode> { node: N }

impl NodeBuilder<Root> {
	fn build_fake_root<E>(build: impl BuildFn<Root, E>) -> Result<TmDoc, E> {
		Ok(build(Self::default())?.build())
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

impl NodeBuilder<Html> {
	pub fn build_value<E>(
		mut self,
		tag: &str,
		params: HashMap<&str, &str>,
		build_inner: impl BuildFn<Root, E>
	) -> Result<Self, E> {
		let fake_root = NodeBuilder::build_fake_root(build_inner)?;
		let inner     = fake_root.to_md_text();
		let inner_len = inner.len();

		let mut outer_len = tag.len() * 2 + 5; // <tag></tag>
				outer_len += params
					.iter()
					.map(|(k, v)|
						k.len() + v.len() + 4 // .k1="v1".k2="v2" ...
					).sum::<usize>();
		let mut value = String::with_capacity(outer_len + inner_len);

		value.push('<');
		value.push_str(tag);
		
		value = params.into_iter().fold(value, |mut value, (k, v)| {
			value.push(' ');
			value.push_str(k);
			value.push_str("=\"");
			value.push_str(v);
			value.push('"');
			value
		});

		value.push('>');
		value.push_str(&inner);
		value.push_str("</");
		value.push_str(tag);
		value.push_str(">\n");

		Ok(self.set_value(value))
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
	pub fn append_value(mut self, text: String) -> Self {
		self.node.append_value(text);
		self
	}

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
	pub fn set_title_html(mut self, title: NodeBuilder<Html>) -> Self {
		self.set_title(Some(title.node.value))
	}

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

impl Default for NodeBuilder<BlockQuote> {
	fn default() -> Self { new_block_quote() }
}

impl Default for NodeBuilder<Break> {
	fn default() -> Self { Self { node: new_break() } }
}

impl Default for NodeBuilder<Code> {
	fn default() -> Self { new_code() }
}

impl Default for NodeBuilder<Definition> {
	fn default() -> Self { new_def() }
}

impl Default for NodeBuilder<Delete> {
	fn default() -> Self { new_delete() }
}

impl Default for NodeBuilder<Emphasis> {
	fn default() -> Self { new_emph() }
}

impl Default for NodeBuilder<FootnoteDefinition> {
	fn default() -> Self { new_foot_def() }
}

impl Default for NodeBuilder<Heading> {
	fn default() -> Self { new_heading() }
}

impl Default for NodeBuilder<Html> {
	fn default() -> Self { new_html() }
}

impl Default for NodeBuilder<Image> {
	fn default() -> Self { new_image() }
}

impl Default for NodeBuilder<ImageReference> {
	fn default() -> Self { new_image_ref() }
}

impl Default for NodeBuilder<InlineCode> {
	fn default() -> Self { new_inline_code() }
}

impl Default for NodeBuilder<InlineMath> {
	fn default() -> Self { new_inline_math() }
}

impl Default for NodeBuilder<Link> {
	fn default() -> Self { new_link() }
}

impl Default for NodeBuilder<LinkReference> {
	fn default() -> Self { new_link_ref() }
}

impl Default for NodeBuilder<List> {
	fn default() -> Self { new_list() }
}

impl Default for NodeBuilder<ListItem> {
	fn default() -> Self { new_list_item() }
}

impl Default for NodeBuilder<Math> {
	fn default() -> Self { new_math() }
}

impl Default for NodeBuilder<Paragraph> {
	fn default() -> Self { new_para() }
}

impl Default for NodeBuilder<Strong> {
	fn default() -> Self { new_strong() }
}

impl Default for NodeBuilder<Root> {
	fn default() -> Self {
		Self { node: new_root() }
	}
}

impl Default for NodeBuilder<Table> {
	fn default() -> Self { new_table() }
}

impl Default for NodeBuilder<TableCell> {
	fn default() -> Self { new_table_cell() }
}

impl Default for NodeBuilder<TableRow> {
	fn default() -> Self { new_table_row() }
}

impl Default for NodeBuilder<Text> {
	fn default() -> Self { Self { node: new_text(String::new()) } }
}

impl Default for NodeBuilder<ThematicBreak> {
	fn default() -> Self { Self{ node: new_them_break() } }
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
