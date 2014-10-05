UTF8Reader - Provides a wrapper around an implementation of std::io::Reader that parses the stream as a sequence of UTF-8 encoded codepoints.

It provides an implementation of `Iterator<IoError<char>>` for ease of access

Using
```rust
let mut reader = utf8reader::UTF8Reader::new( file_handle );
for codepoint in reader.map(|cp| cp.unwrap())
{
	print!("{}", codepoint);
}
```

=== Licencing ===
This code is distributed under the terms of the zlib licence (see COPYING).
