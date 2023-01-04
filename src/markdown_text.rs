use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
	static ref RE: Regex = Regex::new(
		r"#{1,6}|[=-]{2,}|[*_]+|(?<=\d)\.|(?<=^|\s)[-+>]|~{2}|`{1,3}|!?\[|</?.*>"
	).unwrap();
}

pub fn escape_markdown(text: &str) -> String {
	RE.replace_all(text, r"\$0").to_string()
}

#[cfg(test)]
mod tests {
    use super::escape_markdown;

	#[test]
	fn escape() {
		let md = r#"
<https://www.markdownguide.org/basic-syntax/>

<html></html>

# Heading level 1
## Heading level 2
### Heading level 3
#### Heading level 4
##### Heading level 5
###### Heading level 6

Heading level 1
===============
Heading level 2
---------------

I just love **bold text**.
I just love __bold text__.
Love**is**bold

Italicized text is the *cat's meow*.
Italicized text is the _cat's meow_.
A*cat*meow

> Dorothy followed her through many of the beautiful rooms in her castle.

> Dorothy followed her through many of the beautiful rooms in her castle.
>
>> The Witch bade her clean the pots and kettles and sweep the floor and keep the fire fed with wood.

1. First item
2. Second item
3. Third item
4. Fourth item

1. First item
2. Second item
3. Third item
    1. Indented item
    2. Indented item
4. Fourth item

- First item
- Second item
- Third item
    - Indented item
    - Indented item
- Fourth item

***
---
_________________

At the command prompt, type `nano`.
``Use `code` in your Markdown file.``
```json
{
    "firstName": "John",
    "lastName": "Smith",
    "age": 25
}
```

My favorite search engine is [Duck Duck Go](https://duckduckgo.com "The best search engine for privacy").
		"#;

		let esc = r#"
\<https://www.markdownguide.org/basic-syntax/>

\<html></html>

\# Heading level 1
\## Heading level 2
\### Heading level 3
\#### Heading level 4
\##### Heading level 5
\###### Heading level 6

Heading level 1
\===============
Heading level 2
\---------------

I just love \**bold text\**.
I just love \__bold text\__.
Love\**is\**bold

Italicized text is the \*cat's meow\*.
Italicized text is the \_cat's meow\_.
A\*cat\*meow

\> Dorothy followed her through many of the beautiful rooms in her castle.

\> Dorothy followed her through many of the beautiful rooms in her castle.
\>
\>> The Witch bade her clean the pots and kettles and sweep the floor and keep the fire fed with wood.

1\. First item
2\. Second item
3\. Third item
4\. Fourth item

1\. First item
2\. Second item
3\. Third item
    1\. Indented item
    2\. Indented item
4\. Fourth item

\- First item
\- Second item
\- Third item
    \- Indented item
    \- Indented item
\- Fourth item

\***
\---
\_________________

At the command prompt, type \`nano\`.
\``Use \`code\` in your Markdown file.\``
\```json
{
    "firstName": "John",
    "lastName": "Smith",
    "age": 25
}
\```

My favorite search engine is \[Duck Duck Go](https://duckduckgo.com "The best search engine for privacy").
		"#;

		assert_eq!(escape_markdown(md), esc.to_string());
	}
}
