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
use transmark::{TmDoc, IntoHtmlDom};
use std::io::BufReader;
use std::fs::File;
use tl::{VDom, ParserOptions, parse};

let html_text: &str = r#"
<html>
	<head></head>
	<body>
		<h1>Title!</h1>

		<p>Some <i>meaningful</i> text.</p>
	</body>
</html>
"#;

let file: File = File::open("doc.html");
let vdom: VDom<'_> = parse(html_text, ParserOptions::default());

let text_doc: TmDoc = TmDoc::parse_html(html_text);
let file_doc: TmDoc = TmDoc::parse_html(html_file);
let vdom_doc: TmDoc = TmDoc::parse_html(html_vdom);
```

Then, convert to another language's AST:

```rust
use transmark::{TmDoc, IntoMarkdownAst, IntoBBCodeAst, IntoHtmlDom};
use transmark::ast::bbcode::BBDoc;
use markdown::mdast::Node;
use tl::VDom;

let doc: TmDoc = ...;

let md: Node = doc.convert_to_md();
let bb: BBDoc = doc.convert_to_bb();
let html: VDom<'_> = doc.convert_to_html();
```

Alternatively, write converted markup as text:

```rust
use transmark::{TmDoc, IntoMarkdownText, IntoBBCodeText, IntoHtmlText};

let doc: TmDoc = ...;

let md: String = doc.convert_to_md_text();
let bb: String = doc.convert_to_bb_text();
let html: String = doc.convert_to_html_text();
```

### Note on conversion lossiness

Note that because some markup constructs will have no equivalent in other languages, conversion can be lossy. Any styling in HTML, font colors or sizes in BBCode, etc. may be absent in the generated markup.

## License

This project is licensed under [Apache-2.0](LICENSE).
