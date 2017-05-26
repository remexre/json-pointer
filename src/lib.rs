//! A crate for parsing and using JSON pointers, as specified in RFC 6901.

#![deny(missing_docs)]

#[cfg(hyper)]
extern crate hyper;
#[cfg(url)]
extern crate url;
extern crate serde_json;

mod parser;

use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

/// A JSON Pointer.
///
/// Create a new JSON pointer with [`JsonPointer::new`](#method.new), or parse one from a
/// string with [`str::parse()`](https://doc.rust-lang.org/std/primitive.str.html#method.parse).
pub struct JsonPointer<S: AsRef<str>, C: AsRef<[S]>> {
    ref_toks: C,
    _phantom: PhantomData<S>,
}

impl<S: AsRef<str>, C: AsRef<[S]>> JsonPointer<S, C> {
    /// Creates a new JsonPointer from the given reference tokens.
    pub fn new(ref_toks: C) -> JsonPointer<S, C> {
        JsonPointer {
            ref_toks: ref_toks,
            _phantom: PhantomData,
        }
    }

    /// Attempts to get a reference to a value from the given JSON value,
    /// returning an error if it can't be found.
    pub fn get<'json>(&self, val: &'json Value) -> Result<&'json Value, Error> {
        self.ref_toks.as_ref().iter().fold(Ok(val), |val, tok| val.and_then(|val| {
            let tok = tok.as_ref();
            match *val {
                Value::Object(ref obj) => obj.get(tok).ok_or_else(|| Error::NoSuchKey(tok.to_owned())),
                Value::Array(ref arr) => {
                    let idx = if tok == "-" {
                        arr.len()
                    } else if let Ok(idx) = tok.parse() {
                        idx
                    } else {
                        return Err(Error::NoSuchKey(tok.to_owned()));
                    };
                    arr.get(idx).ok_or(Error::OutOfBounds(idx))
                },
                _ => Err(Error::NotIndexable),
            }
        }))
    }

    /// Attempts to get a mutable reference to a value from the given JSON
    /// value, returning an error if it can't be found.
    pub fn get_mut<'json>(&self, val: &'json mut Value) -> Result<&'json mut Value, Error> {
        self.ref_toks.as_ref().iter().fold(Ok(val), |val, tok| val.and_then(|val| {
            let tok = tok.as_ref();
            match *val {
                Value::Object(ref mut obj) => obj.get_mut(tok).ok_or_else(|| Error::NoSuchKey(tok.to_owned())),
                Value::Array(ref mut arr) => {
                    let idx = if tok == "-" {
                        arr.len()
                    } else if let Ok(idx) = tok.parse() {
                        idx
                    } else {
                        return Err(Error::NoSuchKey(tok.to_owned()));
                    };
                    arr.get_mut(idx).ok_or(Error::OutOfBounds(idx))
                },
                _ => Err(Error::NotIndexable),
            }
        }))
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

impl<S: AsRef<str>> FromIterator<S> for JsonPointer<S, Vec<S>> {
    fn from_iter<T: IntoIterator<Item=S>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl FromStr for JsonPointer<String, Vec<String>> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s.chars())
    }
}

/// An error that can be encountered by using this crate.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// The pointer ended with a tilde (`~`), which is illegal because the tilde
    /// is part of an escape sequence.
    EndInEscape,
    /// An invalid escape sequence was encountered.
    InvalidEscape(char),
    /// An error caused by not having a leading slash on the JSON pointer.
    ///
    /// For example, the string `a/b/c` is not a valid JSON pointer, while
    /// `/a/b/c` is.
    ///
    /// This error will therefore be triggered by trying to use the URI
    /// Fragment Identifier Representation.
    NoLeadingSlash,
    /// The pointer pointed to a nonexistent key.
    NoSuchKey(String),
    /// The pointer resulted in trying to index a non-indexable type.
    NotIndexable,
    /// The pointer pointed to an out-of-bounds value in an array.
    OutOfBounds(usize),
}

impl<'a, S: AsRef<str>, C: AsRef<[S]>> Index<&'a JsonPointer<S, C>> for Value {
    type Output = Value;
    fn index(&self, ptr: &'a JsonPointer<S, C>) -> &Value {
        ptr.get(self).unwrap()
    }
}

impl<'a, S: AsRef<str>, C: AsRef<[S]>> IndexMut<&'a JsonPointer<S, C>> for Value {
    fn index_mut(&mut self, ptr: &'a JsonPointer<S, C>) -> &mut Value {
        ptr.get_mut(self).unwrap()
    }
}
