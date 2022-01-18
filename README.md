# Snippet parser

![tests](https://img.shields.io/badge/tests-passing-green) <!--Tests are performed manually for this project if anyone is wondering-->
![Language](https://img.shields.io/badge/lang-Rust-B7410E)
[![Licenses](https://img.shields.io/crates/l/snippets-rs)](#license)
[![Crates.io](https://img.shields.io/crates/v/snippets-rs)](https://crates.io/crates/snippets-rs)
[![Docs.rs](https://img.shields.io/docsrs/snippets-rs)](https://docs.rs/snippets-rs/latest/snippets_rs/)

The snippets file format is a human-readable format for storing strings, which makes it perfect for developpers.

For more information on the format, see [the snippets spec](https://github.com/jomy10/snippets).

**Example of a snippet file**
```snippet
-- my snippet --
This snippet contains a string
-- end --
Text here is ignored, so it can be used to write comments
-- my second snippet --
This snippet contains multiple lines --
-- end --
```

## About this parser
This is a [snippet](https://github.com/jomy10/snippets) parser written in Rust. It can be used to both read and write
snippets. When reading snippets from a file, they are only read into memory as needed.

## Overview
In this section we will define `snippets.snip` as:
```snippet
-- snippet1 --
Are we human?
Or are we dancer?
-- end --

-- snippet2 --
This is my church.
This is where I heal my hurts.
-- end --

This is a comment
-- snippet3 --
Never gonna give you up
Never gonna let you down
Never gonna run around and desert you

Never gonna make you cry
Never gonna say goodbye
Never gonna tell a lie and hurt you
-- end --
```

### Reading from a file
```rust
let parser = SnippetParser::read("snippet.snip").unwrap();
```

Now that we have a parser, we have some possibilities to read snippets:

#### Iterator
```rust
assert_eq!(
    Snippet::new("snippet1".to_string(), "Are we human?\nOr are we dancer?".to_string),
    parser.next()
);
```

OR

```rust
let snippets = vec![
    Snippet::new("snippet1".to_string(), "Are we human?\nOr are we dancer?".to_string),
    Snippet::new("snippet2".to_string(), /*..*/),
    Snippet::new("snippet3".to_string(), /*...*/),
];
let snippets_from_iterator = parser.into_iter().map(|snip| snip).collect::<Vec<Snippet>>();

assert_eq!(snippets, snippets_from_iterator);
```

#### Get snippets
```rust
assert_eq!(snippets, parser.get_snippets().unwrap());
```

#### Get snippet with title
```rust
assert_eq!(
        Snippet::new("snippet1".to_string(), "Are we human?\nOr are we dancer?".to_string),
        parser.get_snippet("snippet1")
);
```

### Adding snippets to the parser
You can add a snippet to a parser using:
```rust
parser.add_snippet(Snippet::new("snippet4".to_string(), "No one knows".to_string));
```

You can also create an empty parser and add snippets to it, or initialize a parser directly with snippets.
```rust
let mut parser1 = SnippetParser::new();
parser1.add_snippet(Snippet::new("snippet4".to_string(), "No one knows".to_string));
let parser2 = SnippetParser::from_snippets(vec![Snippet::new("snippet4".to_string(), "No one knows".to_string)]);
assert_eq!(parser1.get_snippets(), parser2.get_snippets());
```

### Saving to a new file
```rust
// Get file contents
let file_contents = parser.to_string();

// Write file
let f = OpenOptions::new()
    .append(true)
    .open("output_file.snip")
    .expect("Unable to open file");
let mut f = BufWriter::new(f);
f.write_all(file_contents.as_bytes()).expect("Unable to write data");
```

## Installing
This crate is published to crates.io, so just add the following to your cargo.toml:
```toml
snippets-rs = "0.1.0"
```

## Contributing
There are a lot of optimizations that can be made to this crate, so feel free to open an issue and a pull request.
Just make sure the tests are working.

## License
This crate is licensed under the [MIT license](LICENSE)
