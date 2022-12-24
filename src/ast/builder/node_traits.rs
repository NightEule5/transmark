use markdown::mdast::*;

// BlockNode

/// A node type that contains other nodes of any type.
pub trait BlockNode : AsNode {
	fn append(&mut self, node: Node);
}

impl BlockNode for BlockQuote {
	fn append(&mut self, node: Node) {
		self.children.push(node)
	}
}

impl BlockNode for Delete {
	fn append(&mut self, node: Node) {
		self.children.push(node)
	}
}

impl BlockNode for Emphasis {
	fn append(&mut self, node: Node) {
		self.children.push(node)
	}
}

impl BlockNode for Heading {
	fn append(&mut self, node: Node) {
		self.children.push(node)
	}
}

impl BlockNode for Paragraph {
	fn append(&mut self, node: Node) {
		self.children.push(node)
	}
}

impl BlockNode for Root {
	fn append(&mut self, node: Node) {
		self.children.push(node)
	}
}

impl BlockNode for Strong {
	fn append(&mut self, node: Node) {
		self.children.push(node)
	}
}

// ParentNode

/// A node type that contains other nodes of a specific type.
pub trait ParentNode<C : AsNode> : AsNode {
	fn append(&mut self, node: C);
}

impl ParentNode<ListItem> for List {
	fn append(&mut self, node: ListItem) {
		self.children.push(node.as_node())
	}
}

impl ParentNode<TableRow> for Table {
	fn append(&mut self, node: TableRow) {
		self.children.push(node.as_node())
	}
}

impl ParentNode<TableCell> for TableRow {
	fn append(&mut self, node: TableCell) {
		self.children.push(node.as_node())
	}
}

// TextNode

/// A node type that contains a text value.
pub trait TextNode : AsNode {
	fn set_value(&mut self, text: String);
}

impl TextNode for Code {
	fn set_value(&mut self, text: String) {
		self.value = text;
	}
}

impl TextNode for Html {
	fn set_value(&mut self, text: String) {
		self.value = text;
	}
}

impl TextNode for InlineCode {
	fn set_value(&mut self, text: String) {
		self.value = text;
	}
}

impl TextNode for InlineMath {
	fn set_value(&mut self, text: String) {
		self.value = text;
	}
}

impl TextNode for Math {
	fn set_value(&mut self, text: String) {
		self.value = text;
	}
}

impl TextNode for Text {
	fn set_value(&mut self, text: String) {
		self.value = text;
	}
}

// DefRefNode

pub trait DefRefNode : AsNode {
	fn set_id(&mut self, id: String);
	fn set_label(&mut self, label: Option<String>);
}

impl DefRefNode for Definition {
	fn set_id(&mut self, id: String) {
		self.identifier = id;
	}

	fn set_label(&mut self, label: Option<String>) {
		self.label = label;
	}
}

impl DefRefNode for LinkReference {
	fn set_id(&mut self, id: String) {
		self.identifier = id;
	}

	fn set_label(&mut self, label: Option<String>) {
		self.label = label;
	}
}

impl DefRefNode for ImageReference {
	fn set_id(&mut self, id: String) {
		self.identifier = id;
	}

	fn set_label(&mut self, label: Option<String>) {
		self.label = label;
	}
}

impl DefRefNode for FootnoteDefinition {
	fn set_id(&mut self, id: String) {
		self.identifier = id;
	}

	fn set_label(&mut self, label: Option<String>) {
		self.label = label;
	}
}

impl DefRefNode for FootnoteReference {
	fn set_id(&mut self, id: String) {
		self.identifier = id;
	}

	fn set_label(&mut self, label: Option<String>) {
		self.label = label;
	}
}

// RefKindNode

pub trait RefKindNode : AsNode {
	fn set_ref_kind(&mut self, kind: ReferenceKind);
}

impl RefKindNode for LinkReference {
	fn set_ref_kind(&mut self, kind: ReferenceKind) {
		self.reference_kind = kind;
	}
}

impl RefKindNode for ImageReference {
	fn set_ref_kind(&mut self, kind: ReferenceKind) {
		self.reference_kind = kind;
	}
}

// LinkNode

pub trait LinkNode : AsNode {
	fn set_title(&mut self, title: Option<String>);
	fn set_url(&mut self, url: String);
}

impl LinkNode for Definition {
	fn set_title(&mut self, title: Option<String>) {
		self.title = title;
	}

	fn set_url(&mut self, url: String) {
		self.url = url;
	}
}

impl LinkNode for Link {
	fn set_title(&mut self, title: Option<String>) {
		self.title = title;
	}

	fn set_url(&mut self, url: String) {
		self.url = url;
	}
}

impl LinkNode for Image {
	fn set_title(&mut self, title: Option<String>) {
		self.title = title;
	}

	fn set_url(&mut self, url: String) {
		self.url = url;
	}
}

// AltNode

pub trait AltNode : AsNode {
	fn set_alt(&mut self, alt: String);
}

impl AltNode for Image {
	fn set_alt(&mut self, alt: String) {
		self.alt = alt;
	}
}

impl AltNode for ImageReference {
	fn set_alt(&mut self, alt: String) {
		self.alt = alt;
	}
}

// AsNode

pub trait AsNode {
	fn as_node(self) -> Node;
}

impl AsNode for Node {
	fn as_node(self) -> Node { self }
}

impl AsNode for BlockQuote {
	fn as_node(self) -> Node { Node::BlockQuote(self) }
}

impl AsNode for Break {
	fn as_node(self) -> Node { Node::Break(self) }
}

impl AsNode for Code {
	fn as_node(self) -> Node { Node::Code(self) }
}

impl AsNode for Definition {
	fn as_node(self) -> Node { Node::Definition(self) }
}

impl AsNode for Delete {
	fn as_node(self) -> Node { Node::Delete(self) }
}

impl AsNode for Emphasis {
	fn as_node(self) -> Node { Node::Emphasis(self) }
}

impl AsNode for FootnoteDefinition {
	fn as_node(self) -> Node { Node::FootnoteDefinition(self) }
}

impl AsNode for FootnoteReference {
	fn as_node(self) -> Node { Node::FootnoteReference(self) }
}

impl AsNode for Heading {
	fn as_node(self) -> Node { Node::Heading(self) }
}

impl AsNode for Html {
	fn as_node(self) -> Node { Node::Html(self) }
}

impl AsNode for Image {
	fn as_node(self) -> Node { Node::Image(self) }
}

impl AsNode for ImageReference {
	fn as_node(self) -> Node { Node::ImageReference(self) }
}

impl AsNode for InlineCode {
	fn as_node(self) -> Node { Node::InlineCode(self) }
}

impl AsNode for InlineMath {
	fn as_node(self) -> Node { Node::InlineMath(self) }
}

impl AsNode for Link {
	fn as_node(self) -> Node { Node::Link(self) }
}

impl AsNode for LinkReference {
	fn as_node(self) -> Node { Node::LinkReference(self) }
}

impl AsNode for List {
	fn as_node(self) -> Node { Node::List(self) }
}

impl AsNode for ListItem {
	fn as_node(self) -> Node { Node::ListItem(self) }
}

impl AsNode for Math {
	fn as_node(self) -> Node { Node::Math(self) }
}

impl AsNode for Paragraph {
	fn as_node(self) -> Node { Node::Paragraph(self) }
}

impl AsNode for Root {
	fn as_node(self) -> Node { Node::Root(self) }
}

impl AsNode for Strong {
	fn as_node(self) -> Node { Node::Strong(self) }
}

impl AsNode for Table {
	fn as_node(self) -> Node { Node::Table(self) }
}

impl AsNode for TableCell {
	fn as_node(self) -> Node { Node::TableCell(self) }
}

impl AsNode for TableRow {
	fn as_node(self) -> Node { Node::TableRow(self) }
}

impl AsNode for Text {
	fn as_node(self) -> Node { Node::Text(self) }
}

impl AsNode for ThematicBreak {
	fn as_node(self) -> Node { Node::ThematicBreak(self) }
}
