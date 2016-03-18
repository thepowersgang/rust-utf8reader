// UTF8Reader
// - by John Hodge (thePowersGang)
//
// 
// Reads a stream of UTF-8 encoded codepoints from a "Reader"
//#![feature(associated_types)]
use std::io;
use std::io::Read;

/// Unicode replacement character
static BADCHAR: char = '\u{FFFD}';

macro_rules! try_some_or {
	($e:expr, $fail:expr) => ( match try!($e) { Some(v) => v, None => $fail } );
}

/// UTF8 reader structure
pub struct UTF8Reader<T: Read>
{
	stream: T,
}	

fn tochar(codepoint: u32) -> char
{
	match ::std::char::from_u32(codepoint)
	{
	Some(c) => c,
	None => BADCHAR,
	}
}

impl<T: Read> UTF8Reader<T>
{
	pub fn new(reader: T) -> UTF8Reader<T>
	{
		UTF8Reader {
			stream: reader,
		}
	}
	
	fn getb(&mut self) -> io::Result<Option<u8>> {
		let mut b = [0];
		if try!(self.stream.read(&mut b)) == 0 {
			Ok(None)
		}
		else {
			Ok(Some(b[0]))
		}
	}
	
	/// Read a single codepoint from the stream.
	/// On an encoding error, it returns '\uFFFD' (the unicode replacement character)
	pub fn getc(&mut self) -> io::Result<Option<char>>
	{
		let ch1 = try_some_or!(self.getb(), return Ok(None)) as u32;
		if ch1 & 0xC0 == 0x80 {
			return Ok( Some(BADCHAR) )
		}
		if ch1 & 0x80 == 0x00
		{
			// Single-byte
			Ok( Some(tochar(ch1)) )
		}
		else if ch1 & 0xE0 == 0xC0
		{
			// Two-byte sequence
			let ch2 = try_some_or!(self.getb(), return Ok(Some(BADCHAR))) as u32;
			if ch2 & 0xC0 != 0x80 {
				return Ok( Some(BADCHAR) );
			}
			
			let ret = (ch1 & 0x1F << 6) | (ch2 & 0x3F << 0);
			Ok( Some(tochar(ret)) )
		}
		else if ch1 & 0xF0 == 0xE0
		{
			// Three-byte sequence
			let ch2 = try_some_or!(self.getb(), return Ok(Some(BADCHAR))) as u32;
			if ch2 & 0xC0 != 0x80 {
				return Ok( Some(BADCHAR) );
			}
			let ch3 = try_some_or!(self.getb(), return Ok(Some(BADCHAR))) as u32;
			if ch3 & 0xC0 != 0x80 {
				return Ok( Some(BADCHAR) );
			}
			
			let ret = (ch1 & 0x0F << 12) | (ch2 & 0x3F << 6) | (ch3 & 0x3F << 0);
			Ok( Some(tochar(ret)) )
		}
		else if ch1 & 0xF8 == 0xF0
		{
			// Four-byte sequence
			let ch2 = try_some_or!(self.getb(), return Ok(Some(BADCHAR))) as u32;
			if ch2 & 0xC0 != 0x80 {
				return Ok( Some(BADCHAR) );
			}
			let ch3 = try_some_or!(self.getb(), return Ok(Some(BADCHAR))) as u32;
			if ch3 & 0xC0 != 0x80 {
				return Ok( Some(BADCHAR) );
			}
			let ch4 = try_some_or!(self.getb(), return Ok(Some(BADCHAR))) as u32;
			if ch4 & 0xC0 != 0x80 {
				return Ok( Some(BADCHAR) );
			}
			
			let ret = (ch1 & 0x07 << 18) | (ch2 & 0x3F << 12) | (ch3 & 0x3F << 6) | (ch4 & 0x3F << 0);
			Ok( Some(tochar(ret)) )
		}
		else
		{
			// More than four bytes is invalid
			Ok( Some(BADCHAR) )
		}
	}
}

/// Implmentation of the same interface as 'Chars' provides, returns None at the end of the stream
impl<T: Read> Iterator for UTF8Reader<T>
{
	type Item = io::Result<char>;
	fn next(&mut self) -> Option<io::Result<char>>
	{
		// Get result from decoder
		match self.getc()
		{
		// - All good, return a character
		Ok(None) => None,
		Ok(Some(c)) => Some( Ok(c) ),
		Err(e) => Some( Err( e ) ),
		}
	}
}

#[test]
fn it_works() {
}

// vim: ft=rust
