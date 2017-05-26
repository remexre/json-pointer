# json-pointer

A crate for parsing and using JSON pointers, as specified in RFC 6901.

[![crates.io](https://img.shields.io/crates/v/json-pointer.svg)](https://crates.io/crates/json-pointer)
[![Build Status](https://travis-ci.org/remexre/json-pointer.svg?branch=master)](https://travis-ci.org/remexre/json-pointer)
[![Documentation](https://docs.rs/json-pointer/badge.svg)](https://docs.rs/json-pointer)

## Creating a JSON Pointer

JSON pointers can be created with a literal `[&str]`, or parsed from a `String`.

```rust
let from_strs = JsonPointer::new([
    "foo"
    "bar",
]);
let parsed = "/foo/bar".parse::<JsonPointer<_, _>>().unwrap();

assert_eq!(from_strs.unwrap(), parsed.unwrap());
```

## Using a JSON Pointer

The `JsonPointer` type provides `.get()` and `.get_mut()`, to get references
and mutable references to the appropriate value, respectively.

```rust
// where ptr is either of the JsonPointers from above.

let document = json!({
    "foo": {
        "bar": 0,
        "baz": 1,
    },
    "quux": "xyzzy"
});

let indexed = ptr.get(&document).unwrap();

assert_eq!(indexed, &json!(0));
```
