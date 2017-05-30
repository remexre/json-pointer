//! A crate for parsing and using JSON pointers, as specified in RFC 6901.

#![deny(missing_docs)]

extern crate serde_json;

mod parser;
mod ptr;

pub use parser::ParseError;
pub use ptr::IndexError;
pub use ptr::JsonPointer;
