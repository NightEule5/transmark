# TransMark

TransMark is a markup language conversion library written in Rust. It can convert between Markdown, HTML, BBCode, and plain text out-of-the-box, and more conversions can be added via a common AST.

## Usage

Add `transmark` to your dependencies by running `cargo add transmark --git https://github.com/NightEule5/transmark --tag v0.1.0`, or by adding it to your `Cargo.toml`:

```toml
[dependencies]
transmark = { git = "https://github.com/NightEule5/transmark", tag = "v0.1.0" }
```

First, parse your document or pass in an already parsed AST from a supported library:

```rust
use transmark::{Markup, ConvertMarkup};
use std::io::BufReader;
use std::fs::File;
use tl::{VDom, ParserOptions, parse};

fn main() {
	let text: &str = r#"
<html>
	<head></head>
	<body>
		<h1>Title!</h1>

		<p>Some <i>meaningful</i> text.</p>
	</body>
</html>
"#;

	let file: File = File::open("doc.html").unwrap();
	let reader: BufReader<_> = BufReader::new(file);

	let vdom: VDom<'_> = parse(html_text, ParserOptions::default());

	let text_doc: Markup = Markup::from_html(text);
	let file_doc: Markup = Markup::from_html(reader);
	let vdom_doc: Markup = Markup::convert_from(vdom);
}
```

Then, convert to another language's AST:

```rust
use transmark::{Markup, ConvertMarkup};
use markdown::mdast::Node;
use tl::VDom;

fn main() -> transmark::Result<()> {
	let doc: Markup = ...;

	let md: Node = doc.convert_into()?;
	let html: VDom<'_> = doc.convert_into()?;
	
	Ok(())
}
```

Alternatively, write converted markup as text:

```rust
use transmark::{Markup, WriteMarkup};

fn main() -> transmark::Result<()> {
	let doc: Markup = ...;

	let md: String = doc.write_md_text()?;
	let bb: String = doc.write_bb_text()?;
	let html: String = doc.write_html_text()?;
	
	Ok(())
}
```

### Custom languages

Custom languages can be added by implementing the `ConvertMarkup`, `WriteMarkup`, and `ReadMarkup` traits on the AST:

```rust
use transmark::{Markup, ConvertMarkup, WriteMarkup, ReadMarkup};
use std::io::{BufReader, BufWriter, Read, Write};

// Your AST
struct Doc {
	// ...
}

// Some conversion error information
struct Error {
	// ...
}

// Convert to and from the common AST
impl ConvertMarkup<Markup<'_>, Error> for Doc {
	fn convert_into(self) -> Result<Markup<'_>, Error> {
		// ...
	}

	fn convert_from(markup: Markup<'_>) -> Result<Self, Error> {
		// ...
	}
}

// Write markup text
impl WriteMarkup<Error> for Doc {
	fn write_markup_text(self) -> Result<String, Error> {
		// ...
	}

	fn write_markup<W: Write>(self, buf: &mut BufWriter<W>) -> Result<(), Error> {
		// ...
	}
}

// Read markup text
impl WriteMarkup<Error> for Doc {
	fn read_markup_text(value: &str) -> Result<Self, Error> {
		// ...
	}

	fn read_markup<R: Read>(buf: &mut BufReader<R>) -> Result<Self, Error> {
		// ...
	}
}
```

## License

This project is licensed under [Apache-2.0](LICENSE).
