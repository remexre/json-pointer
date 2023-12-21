//! A crate for parsing and using JSON pointers with [simd_json] values, as specified in [RFC
//! 6901](https://tools.ietf.org/html/rfc6901). Unlike the `pointer` method
//! built into `serde_json`, this handles both validating JSON Pointers before
//! use and the URI Fragment Identifier Representation.
//!
//! ## Creating a JSON Pointer
//! 
//! JSON pointers can be created with a literal `[&str]`, or parsed from a `String`.
//! 
//! ```rust
//! use json_pointer_simd::{JsonPointer,JsonPointerValueGetter};
//!
//! let from_strs = JsonPointer::new([
//!     "foo",
//!     "bar",
//! ]);
//! let parsed = "/foo/bar".parse::<JsonPointer<_, _>>().unwrap();
//! 
//! assert_eq!(from_strs.to_string(), parsed.to_string());
//! ```
//! 
//! ## Using a JSON Pointer
//! 
//! The `JsonPointer` type provides `.get()` and `.get_mut()`, to get references
//! and mutable references to the appropriate value, respectively.
//! 
//! ```rust
//! use simd_json::json;
//! use json_pointer_simd::{JsonPointer,JsonPointerValueGetter};
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
//! let indexed = ptr.get(&document).unwrap();
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
//! use json_pointer_simd::{JsonPointer,JsonPointerValueGetter};
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
mod ptr;

pub use parser::ParseError;
pub use ptr::IndexError;
pub use ptr::JsonPointer;

///
/// The trait that provides access to the data referenced by the JsonPointer.
/// This trait is implemented for both [OwnedValue] and [BorrowedValue].
/// 
pub trait JsonPointerValueGetter<V> 
	where V: simd_json::base::TypedValue {

    /// Attempts to get a reference to a value from the given JSON value,
    /// returning an error if it can't be found.
	fn get<'json>(&self, val: &'json V) -> Result<&'json V, IndexError>;
    /// Attempts to get a mutable reference to a value from the given JSON
    /// value, returning an error if it can't be found.
	fn get_mut<'json>(&self, val: &'json mut V) -> Result<&'json mut V, IndexError>;
    /// Attempts to get an owned value from the given JSON value, returning an
    /// error if it can't be found.
	fn get_owned(&self, val: V) -> Result<V, IndexError>;
}
