//! A common Abstract Syntax Tree for markup languages all supported by TransMark,
//! based on Markdown syntax. Draws heavily on on the [markdown] crate's [mdast](markdown::mdast)
//! implementation, but unlike [markdown], string slices are used instead of owned
//! strings. Also, Markdown extensions such as MDX and Frontmatter are not supported.
pub mod unist;

use markdown::mdast::AlignKind;
use property::Property;

use unist::{Parent, Position, Literal, Node};
pub use markdown::mdast::ReferenceKind;

// Macros to generate Node trait boilerplate. A bit overkill, but I don't feel like
// writing all of these out by hand.

macro_rules! impl_node {
	() => { };
	(life $name:ident $($t:tt)*) => {
		impl Node for $name<'_> {
			fn position(&self) -> Option<Position> {
				self.position.clone()
			}
	
			fn set_position(&mut self, position: Option<Position>) {
				self.position = position
			}
		}

		impl_node! { $($t)* }
	};
	($name:ident $($t:tt)*) => {
		impl Node for $name {
			fn position(&self) -> Option<Position> {
				self.position.clone()
			}
	
			fn set_position(&mut self, position: Option<Position>) {
				self.position = position
			}
		}

		impl_node! { $($t)* }
	};
}

macro_rules! impl_literal {
	($($name:ident)+) => {
		$(
			impl<'t> Literal<'t> for $name<'t> {
				fn value(&self) -> &'t str { self.value }

				fn set_value(&mut self, value: &'t str) {
					self.value = value;
				}
			}
		)+
	};
}

macro_rules! impl_parent {
	($($name:ident<$child:ident>)+) => {
		$(
			impl<'t> Parent<$child<'t>> for $name<'t> {
				fn children<'c>(&'c self) -> &'c [$child<'t>] {
					self.children.as_slice()
				}

				fn append_child(&mut self, node: $child<'t>) {
					self.children.push(node);
				}
			}
		)+
	};
}

macro_rules! impl_resource {
	($($name:ident)+) => {
		$(
			impl<'t> Resource<'t> for $name<'t> {
				fn url(&self) -> &'t str { self.url }
				fn title(&self) -> Option<&'t str> {
					self.title
				}
			
				fn set_url(&mut self, url: &'t str) {
					self.url = url;
				}
				fn set_title(&mut self, title: Option<&'t str>) {
					self.title = title;
				}
			}
		)+
	};
}

macro_rules! impl_association {
	($($name:ident)+) => {
		$(
			impl<'t> Association<'t> for $name<'t> {
				fn identifier(&self) -> &'t str         { self.identifier }
				fn label     (&self) -> Option<&'t str> { self.label      }
			
				fn set_identifier(&mut self, identifier: &'t str) {
					self.identifier = identifier;
				}
				fn set_label(&mut self, label: Option<&'t str>) {
					self.label = label;
				}
			}
		)+
	};
}

macro_rules! impl_reference {
	($($name:ident)+) => {
		$(
			impl<'t> Reference<'t> for $name<'t> {
				fn reference_kind(&self) -> ReferenceKind { self.kind }

				fn set_reference_kind(&mut self, kind: ReferenceKind) {
					self.kind = kind;
				}
			}
		)+
	};
}

macro_rules! impl_alternative {
	($($name:ident)+) => {
		$(
			impl<'t> Alternative<'t> for $name<'t> {
				fn alt(&self) -> Option<&'t str> { self.alt }
			
				fn set_alt(&mut self, alt: Option<&'t str>) {
					self.alt = alt;
				}
			}
		)+
	};
}

macro_rules! impl_literal_cstr {
	($($name:ident)+) => {
		$(
			impl<'t> $name<'t> {
				pub fn new(value: &'t str, position: Option<Position>) -> Self {
					Self { value, position }
				}
			}
		)+
	};
}

macro_rules! impl_parent_cstr {
	($($name:ident<$child:ident>)+) => {
		$(
			impl<'t> $name<'t> {
				pub fn new(children: Vec<$child<'t>>, position: Option<Position>) -> Self {
					Self { children, position }
				}
			}
		)+
	};
}

macro_rules! impl_node_cstr {
	($($name:ident)+) => {
		$(
			impl $name {
				pub fn new(position: Option<Position>) -> Self {
					Self { position }
				}
			}
		)+
	};
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Content<'t> {
	Flow(FlowContent<'t>),
	Phasing(PhrasingContent<'t>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlowContent<'t> {
	Code(Code<'t>),
	Content(TextContent<'t>),
	FootnoteDef(FootnoteDef<'t>),
	Heading(Heading<'t>),
	Html(Html<'t>),
	List(List<'t>),
	Math(Math<'t>),
	Quote(Quote<'t>),
	Table(Table<'t>),
	ThematicBreak(ThematicBreak),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PhrasingContent<'t> {
	FootnoteRef(FootnoteDef<'t>),
	Link(Link<'t>),
	LinkRef(LinkReference<'t>),
	Static(StaticPhrasingContent<'t>)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StaticPhrasingContent<'t> {
	Break(Break),
	Delete(Delete<'t>),
	Emphasis(Emphasis<'t>),
	Html(Html<'t>),
	Image(Image<'t>),
	ImageRef(ImageReference<'t>),
	InlineCode(InlineCode<'t>),
	InlineMath(InlineMath<'t>),
	Strong(Strong<'t>),
	Text(Text<'t>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TextContent<'t> {
	Definition(Definition<'t>),
	Paragraph(Paragraph<'t>),
}

pub trait Resource<'t> {
	fn url  (&self) -> &'t str;
	fn title(&self) -> Option<&'t str>;

	fn set_url  (&mut self, url  : &'t str);
	fn set_title(&mut self, title: Option<&'t str>);
}

pub trait Association<'t> {
	fn identifier(&self) -> &'t str;
	fn label     (&self) -> Option<&'t str>;

	fn set_identifier(&mut self, identifier: &'t str);
	fn set_label     (&mut self, label     : Option<&'t str>);
}

pub trait Reference<'t> : Association<'t> {
	fn reference_kind(&self) -> ReferenceKind;

	fn set_reference_kind(&mut self, kind: ReferenceKind);
}

pub trait Alternative<'t> {
	fn alt(&self) -> Option<&'t str>;

	fn set_alt(&mut self, alt: Option<&'t str>);
}

/// A line break node.
/// ```markdown
/// a\
/// b
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Break {
	/// The position within the document.
	pub position: Option<Position>
}

/// A code fence node.
/// ~~~markdown
/// ```rust
/// fn something() { }
/// ```
/// ~~~
#[derive(Clone, Debug, Eq, PartialEq, Property)]
#[property(get(public), set(public), mut(disable))]
pub struct Code<'t> {
	/// The code string.
	#[property(skip)]
	pub value: &'t str,
	/// The language, if any.
	#[property(get(type = "clone"), set(type = "none"))]
	pub lang : Option<&'t str>,
	/// The metadata, if any.
	#[property(get(type = "clone"), set(type = "none"))]
	pub meta : Option<&'t str>,
	/// The position within the document.
	#[property(skip)]
	pub position: Option<Position>
}

/// A definition node.
/// ```markdown
/// [label]: url "title"
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Definition<'t> {
	/// The identifier. This should be a lowercased, word-character-only version of
	/// the label.
	pub identifier: &'t str,
	/// The label, if any.
	pub label: Option<&'t str>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A deletion node.
/// ```markdown
/// ~~strike~~
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delete<'t> {
	/// [PhrasingContent] children.
	pub children: Vec<PhrasingContent<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// An emphasis node.
/// ```markdown
/// *important*
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Emphasis<'t> {
	/// [PhrasingContent] children.
	pub children: Vec<PhrasingContent<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A footnote definition node.
/// ```markdown
/// [^id]: Some content "label"
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FootnoteDef<'t> {
	/// The identifier.
	pub identifier: &'t str,
	/// The label, if any.
	pub label: Option<&'t str>,
	/// [FlowContent] children.
	pub children: Vec<FlowContent<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A footnote reference node.
/// ```markdown
/// [^id]
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FootnoteRef<'t> {
	/// The identifier.
	pub identifier: &'t str,
	/// The label, if any.
	pub label: Option<&'t str>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A section heading node.
/// ```markdown
/// # The quick brown fox
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Property)]
#[property(get(public), set(public), mut(disable))]
pub struct Heading<'t> {
	/// The heading depth from 1 to 6.
	#[property(get(type = "clone"), set(type = "none"))]
	pub depth: u8,
	/// [PhrasingContent] children.
	#[property(skip)]
	pub children: Vec<PhrasingContent<'t>>,
	/// The position within the document.
	#[property(skip)]
	pub position: Option<Position>
}

/// An inline HTML tag.
/// ```markdown
/// <a></a>
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Html<'t> {
	/// The literal value.
	pub value: &'t str,
	/// The position within the document.
	pub position: Option<Position>
}

/// An inline code node.
/// ```markdown
/// `a`
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineCode<'t> {
	/// The code string.
	pub value: &'t str,
	/// The position within the document.
	pub position: Option<Position>
}

/// An inline LaTeX math node.
/// ```markdown
/// &x + y&
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineMath<'t> {
	/// The LaTeX math string.
	pub value: &'t str,
	/// The position within the document.
	pub position: Option<Position>
}

/// An image node.
/// ```markdown
/// ![alt](url "title")
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image<'t> {
	/// The alternate text to display if the image can't be rendered.
	pub alt: Option<&'t str>,
	/// The image url.
	pub url: &'t str,
	/// The image title, if any, to be displayed as extra information, such as a
	/// tooltip.
	pub title: Option<&'t str>,
	/// The position within the document.
	pub position: Option<Position>
}

/// An image reference node.
/// ```markdown
/// ![alt][Label]
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageReference<'t> {
	/// The alternate text to display if the image can't be rendered.
	pub alt: Option<&'t str>,
	/// The reference identifier.
	pub identifier: &'t str,
	/// The reference label.
	pub label: Option<&'t str>,
	/// The reference kind.
	pub kind: ReferenceKind,
	/// The position within the document.
	pub position: Option<Position>
}

/// A link node.
/// ```markdown
/// [some text](url "title")
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Link<'t> {
	/// The link URL.
	pub url: &'t str,
	/// The link title, if any, to be displayed as extra information, such as a
	/// tooltip.
	pub title: Option<&'t str>,
	/// [StaticPhrasingContent] children to display instead of the URL.
	pub children: Vec<StaticPhrasingContent<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A link reference node.
/// ```markdown
/// [some text][Label]
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkReference<'t> {
	/// The reference identifier.
	pub identifier: &'t str,
	/// The reference label.
	pub label: Option<&'t str>,
	/// The reference kind.
	pub kind: ReferenceKind,
	/// [StaticPhrasingContent] children.
	pub children: Vec<StaticPhrasingContent<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A list node.
/// ```markdown
/// - Item 1
/// - Item 2
/// - Item 3
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Property)]
#[property(get(public), set(public), mut(disable))]
pub struct List<'t> {
	/// Whether the list is ordered or unordered.
	#[property(get(type = "clone"), set(type = "none"))]
	pub ordered: bool,
	/// The list start number, or `None` if unordered.
	#[property(get(type = "clone"), set(type = "none"))]
	pub start: Option<u32>,
	/// `true` if some list items have a blank line between them, `false` or `None`
	/// if not.
	#[property(get(type = "clone"), set(type = "none"))]
	pub spread: Option<bool>,
	/// [ListItem] children.
	#[property(skip)]
	pub children: Vec<ListItem<'t>>,
	/// The position within the document.
	#[property(skip)]
	pub position: Option<Position>
}

/// A list item node.
/// ```markdown
/// - Item
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Property)]
#[property(get(public), set(public), mut(disable))]
pub struct ListItem<'t> {
	/// Whether the list item has a blank space at the end.
	#[property(get(type = "clone"), set(type = "none"))]
	pub spread: bool,
	/// Whether the list item is checked, or `None` if indeterminate (not a checked
	/// list).
	#[property(get(type = "clone"), set(type = "none"))]
	pub checked: Option<bool>,
	/// [FlowContent] children.
	#[property(skip)]
	pub children: Vec<FlowContent<'t>>,
	/// The position within the document.
	#[property(skip)]
	pub position: Option<Position>
}

/// A LaTeX math block.
/// ```markdown
/// $$
/// a + b
/// $$
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Property)]
#[property(get(public), set(public), mut(disable))]
pub struct Math<'t> {
	/// The LaTeX math string.
	#[property(skip)]
	pub value: &'t str,
	/// The metadata, if any.
	#[property(get(type = "clone"), set(type = "none"))]
	pub meta: Option<&'t str>,
	/// The position within the document.
	#[property(skip)]
	pub position: Option<Position>
}

/// A paragraph node.
/// ```markdown
/// The quick brown fox
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Paragraph<'t> {
	/// [PhrasingContent] children.
	pub children: Vec<PhrasingContent<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A block quote node.
/// ```markdown
/// > The quick brown fox
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Property)]
#[property(get(public), set(public), mut(disable))]
pub struct Quote<'t> {
	/// The author the quote is attributed to. While this field is not supported in
	/// Markdown, it could be formatted as text at the end:
	/// ```markdown
	/// > The quick brown fox
	/// —Unknown
	/// ```
	#[property(get(type = "clone"), set(type = "none"))]
	pub author: Option<&'t str>,
	/// [FlowContent] children.
	#[property(skip)]
	pub children: Vec<FlowContent<'t>>,
	/// The position within the document.
	#[property(skip)]
	pub position: Option<Position>
}

/// The root node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Root<'t> {
	/// [Content] children.
	pub children: Vec<Content<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A strong text node.
/// ```markdown
/// **scream**
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Strong<'t> {
	/// [PhrasingContent] children.
	pub children: Vec<PhrasingContent<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A table node.
/// ```markdown
/// | 1 | 2 | 3 |
/// |:-:|:--|--:|
/// | 4 | 5 | 6 |
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Property)]
#[property(get(public), set(public), mut(disable))]
pub struct Table<'t> {
	/// The column alignments.
	#[property(get(type = "clone"), set(type = "none"))]
	pub align: Option<Vec<AlignKind>>,
	/// [TableRow] children.
	pub children: Vec<TableRow<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A table cell node.
/// ```markdown
/// | a |
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableCell<'t> {
	/// [PhrasingContent] children.
	pub children: Vec<PhrasingContent<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A table row node.
/// ```markdown
/// | a | b | c |
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableRow<'t> {
	/// [TableCell] children.
	pub children: Vec<TableCell<'t>>,
	/// The position within the document.
	pub position: Option<Position>
}

/// A text node.
/// ```markdown
/// The quick brown fox
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Text<'t> {
	/// The literal value.
	pub value: &'t str,
	/// The position within the document.
	pub position: Option<Position>
}

/// A thematic break node.
/// ```markdown
/// ***
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThematicBreak {
	/// The position within the document.
	pub position: Option<Position>
}

impl_node! {
	Break
	life Code
	life Delete
	life Emphasis
	life FootnoteDef
	life FootnoteRef
	life Heading
	life Html
	life InlineCode
	life InlineMath
	life Image
	life ImageReference
	life Link
	life LinkReference
	life List
	life ListItem
	life Math
	life Paragraph
	life Quote
	life Root
	life Table
	life TableCell
	life TableRow
	life Text
	ThematicBreak
}

impl_literal! {
	Code
	Html
	Math
	Text
}

impl_parent! {
	Delete<PhrasingContent>
	FootnoteDef<FlowContent>
	Heading<PhrasingContent>
	Link<StaticPhrasingContent>
	LinkReference<StaticPhrasingContent>
	List<ListItem>
	ListItem<FlowContent>
	Paragraph<PhrasingContent>
	Quote<FlowContent>
	Root<Content>
	Table<TableRow>
	TableCell<PhrasingContent>
	TableRow<TableCell>
}

impl_association! {
	Definition
	FootnoteDef
	FootnoteRef
	ImageReference
	LinkReference
}

impl_alternative! {
	Image
	ImageReference
}

impl_reference! {
	ImageReference
	LinkReference
}

impl_resource! {
	Image
	Link
}

impl_literal_cstr! {
	Html
	InlineCode
	InlineMath
	Text
}

impl_parent_cstr! {
	Delete<PhrasingContent>
	Emphasis<PhrasingContent>
	Paragraph<PhrasingContent>
	Root<Content>
	Strong<PhrasingContent>
	TableCell<PhrasingContent>
	TableRow<TableCell>
}

impl_node_cstr! {
	Break
	ThematicBreak
}

impl<'t> Code<'t> {
	pub fn new(
		value: &'t str,
		lang: Option<&'t str>,
		meta: Option<&'t str>,
		position: Option<Position>
	) -> Self {
		Self { value, lang, meta, position }
	}
}

impl<'t> Definition<'t> {
	pub fn new(
		identifier: &'t str,
		label: Option<&'t str>,
		position: Option<Position>
	) -> Self {
		Self { identifier, label, position }
	}
}

impl<'t> FootnoteDef<'t> {
	pub fn new(
		identifier: &'t str,
		label: Option<&'t str>,
		children: Vec<FlowContent<'t>>,
		position: Option<Position>
	) -> Self {
		Self { identifier, label, children, position }
	}
}

impl<'t> FootnoteRef<'t> {
	pub fn new(
		identifier: &'t str,
		label: Option<&'t str>,
		position: Option<Position>
	) -> Self {
		Self { identifier, label, position }
	}
}

impl<'t> Heading<'t> {
	pub fn new(
		depth: u8,
		children: Vec<PhrasingContent<'t>>,
		position: Option<Position>
	) -> Self {
		Self { depth, children, position }
	}
}

impl<'t> Image<'t> {
	pub fn new(
		alt: Option<&'t str>,
		url: &'t str,
		title: Option<&'t str>,
		position: Option<Position>
	) -> Self {
		Self { alt, url, title, position }
	}
}

impl<'t> ImageReference<'t> {
	pub fn new(
		alt: Option<&'t str>,
		identifier: &'t str,
		label: Option<&'t str>,
		kind: ReferenceKind,
		position: Option<Position>
	) -> Self {
		Self { alt, identifier, label, kind, position }
	}
}

impl<'t> Link<'t> {
	pub fn new(
		url: &'t str,
		title: Option<&'t str>,
		children: Vec<StaticPhrasingContent<'t>>,
		position: Option<Position>
	) -> Self {
		Self { url, title, children, position }
	}
}

impl<'t> LinkReference<'t> {
	pub fn new(
		identifier: &'t str,
		label: Option<&'t str>,
		kind: ReferenceKind,
		children: Vec<StaticPhrasingContent<'t>>,
		position: Option<Position>
	) -> Self {
		Self { identifier, label, kind, children, position }
	}
}

impl<'t> List<'t> {
	pub fn new(
		ordered: bool,
		start: Option<u32>,
		spread: Option<bool>,
		children: Vec<ListItem<'t>>,
		position: Option<Position>
	) -> Self {
		Self { ordered, start, spread, children, position }
	}
}

impl<'t> ListItem<'t> {
	pub fn new(
		spread: bool,
		checked: Option<bool>,
		children: Vec<FlowContent<'t>>,
		position: Option<Position>
	) -> Self {
		Self { spread, checked, children, position }
	}
}

impl<'t> Math<'t> {
	pub fn new(
		value: &'t str,
		meta: Option<&'t str>,
		position: Option<Position>
	) -> Self {
		Self { value, meta, position }
	}
}

impl<'t> Quote<'t> {
	pub fn new(
		author: Option<&'t str>,
		children: Vec<FlowContent<'t>>,
		position: Option<Position>
	) -> Self {
		Self { author, children, position }
	}
}

impl<'t> Table<'t> {
	pub fn new(
		align: Option<Vec<AlignKind>>,
		children: Vec<TableRow<'t>>,
		position: Option<Position>
	) -> Self {
		Self { align, children, position }
	}
}
