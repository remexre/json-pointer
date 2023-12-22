//! A crate for parsing and using JSON pointers with [simd_json] and [serde_json] values.
//! 
//! The functionality is specified in [RFC 6901](https://tools.ietf.org/html/rfc6901). 
//! 
//! In the case of [serde_json], unlike nlike the `pointer` method, this handles both 
//! validating JSON Pointers before use and the URI Fragment Identifier Representation.
//! 
//! In the case of [simd_json], this crate provides that missing functionality.
//!
//! ## Creating a JSON Pointer
//! 
//! JSON pointers can be parsed from any thing that can interpreted a s string slice 
//! expressed in standard JSON Pointer syntax, or created from anything that can be 
//! loosely represented as a vector or array of `&str`.
//! 
//! ```rust
//! use json_pointer_simd::{JsonPointer,JsonPointerTarget};
//!
//! let from_strs = JsonPointer::new([
//!     "foo",
//!     "bar",
//! ]);
//! let parsed = "/foo/bar".parse::<JsonPointer<_, _>>().unwrap();
//! let from_dotted_notation = JsonPointer::new("foo.bar".split('.').collect::<Vec<&str>>());
//! 
//! assert_eq!(from_strs.to_string(), parsed.to_string());
//! assert_eq!(from_strs.to_string(), from_dotted_notation.to_string());
//! ```
//! 
//! ## Using a JSON Pointer
//! 
//! The `JsonPointerTarget` trait provides `.get()` and `.get_mut()`, to get references
//! and mutable references to the appropriate value, respectively.
//! 
//! As delivered, this is implemented on [serde_json] values and [simd_json] values, though
//! the former is a little more verbose to use than the latter due to the pre-existence of
//! these methods on [serde_json] values 
//! 
//! For [simd_json]:
//! 
//! ```rust
//! use simd_json::json;
//! use json_pointer_simd::{JsonPointer,JsonPointerTarget};
//!
//! let ptr = "/foo/bar".parse::<JsonPointer<_, _>>().unwrap();
//! 
//! let document = json!({
//!     "foo": {
//!         "bar": 0,
//!         "baz": 1,
//!     },
//!     "quux": "xyzzy"
//! });
//! let indexed = document.get(&ptr).unwrap();
//! 
//! assert_eq!(indexed, &json!(0));
//! ```
//! 
//! For [serde_json]:
//! 
//! ```rust
//! use serde_json::{json, Value};
//! use json_pointer_simd::{JsonPointer,JsonPointerTarget};
//!
//! let ptr = "/foo/bar".parse::<JsonPointer<_, _>>().unwrap();
//! 
//! let document = json!({
//!     "foo": {
//!         "bar": 0,
//!         "baz": 1,
//!     },
//!     "quux": "xyzzy"
//! });
//! let indexed = <Value as JsonPointerTarget>::get(&document,&ptr).unwrap();
//! 
//! assert_eq!(indexed, &json!(0));
//! ```
//! 
//! ## URI Fragment Identifier Representation
//! 
//! JSON Pointers can be embedded in the fragment portion of a URI. This is the
//! reason why most JSON pointer libraries require a `#` character at the beginning
//! of a JSON pointer. The crate will detect the leading `#` as an indicator to
//! parse in URI Fragment Identifier Representation. Note that this means that this
//! crate does not support parsing full URIs.
//! 
//! ```rust
//! use json_pointer_simd::{JsonPointer,JsonPointerTarget};
//!
//! let str_ptr = "/f%o".parse::<JsonPointer<_, _>>().unwrap();
//! let uri_ptr = "#/f%25o".parse::<JsonPointer<_, _>>().unwrap();
//! 
//! assert_eq!(str_ptr, uri_ptr);
//! ```

#![deny(missing_docs)]

mod parser;
mod owned;
mod borrowed;
mod value;
mod ptr;

pub use parser::ParseError;
pub use ptr::IndexError;
pub use ptr::JsonPointer;

///
/// The trait that provides access to the data referenced by the JsonPointer.
///
pub trait JsonPointerTarget 
    where Self: Sized{

    /// Attempts to get a reference to a value from self,
    /// returning an error if it can't be found.
	fn get<'json,S: AsRef<str>, C: AsRef<[S]>>(&'json self, ptr: &JsonPointer<S,C>) -> Result<&'json Self, IndexError>;
    /// Attempts to get a mutable reference to a value from self
    /// returning an error if it can't be found.
	fn get_mut<'json,S: AsRef<str>, C: AsRef<[S]>>(&'json mut self, ptr: &JsonPointer<S,C>) -> Result<&'json mut Self, IndexError>;
    /// Attempts to get an owned value from self, returning an
    /// error if it can't be found.
	fn get_owned<S: AsRef<str>, C: AsRef<[S]>>(self, ptr: &JsonPointer<S,C>) -> Result<Self, IndexError>;
}
