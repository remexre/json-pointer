use parser::{parse, ParseError};
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::parser;

/// A JSON Pointer.
///
/// Create a new JSON pointer with [`JsonPointer::new`](#method.new), or parse one from a
/// string with [`str::parse()`](https://doc.rust-lang.org/std/primitive.str.html#method.parse).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonPointer<S: AsRef<str>, C: AsRef<[S]>> {
    pub(crate) ref_toks: C,
    _phantom: PhantomData<S>,
}

impl<S: AsRef<str>, C: AsRef<[S]>> JsonPointer<S, C> {
    /// Creates a new JsonPointer from the given reference tokens.
    pub fn new(ref_toks: C) -> JsonPointer<S, C> {
        JsonPointer {
            ref_toks,
            _phantom: PhantomData,
        }
    }

    /// Converts a JSON pointer to a string in URI Fragment Identifier
    /// Representation, including the leading `#`.
    pub fn uri_fragment(&self) -> String {
        fn legal_fragment_byte(b: u8) -> bool {
            matches!(b, 0x21 | 0x24 | 0x26..=0x3b | 0x3d | 0x3f..=0x5a | 0x5f | 0x61..=0x7a)
        }

        let mut s = "#".to_string();
        for part in self.ref_toks.as_ref().iter() {
            s += "/";
            for b in part.as_ref().bytes() {
                if legal_fragment_byte(b) {
                    s.push(b as char)
                } else {
                    write!(s, "%{:02x}", b).unwrap()
                }
            }
        }
        s
    }
}

impl<S: AsRef<str>> JsonPointer<S, Vec<S>> {
    /// Adds a component to the JSON pointer.
    pub fn push(&mut self, component: S) {
        self.ref_toks.push(component);
    }

    /// Removes and returns the last component from the JSON pointer.
    pub fn pop(&mut self) -> Option<S> {
        self.ref_toks.pop()
    }
}

impl<S: AsRef<str>, C: AsRef<[S]>> Display for JsonPointer<S, C> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        for part in self.ref_toks.as_ref().iter() {
            write!(fmt, "/")?;
            for ch in part.as_ref().chars() {
                match ch {
                    '~' => write!(fmt, "~0"),
                    '/' => write!(fmt, "~1"),
                    c => write!(fmt, "{}", c),
                }?
            }
        }
        Ok(())
    }
}

impl FromStr for JsonPointer<String, Vec<String>> {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

/// An error that can be encountered when indexing using a JSON pointer.
#[derive(Clone, Debug, PartialEq)]
pub enum IndexError {
    /// The pointer pointed to a nonexistent key.
    NoSuchKey(String),
    /// The pointer resulted in trying to index a non-indexable type.
    NotIndexable,
    /// The pointer pointed to an out-of-bounds value in an array.
    OutOfBounds(usize),
}
