# json-pointer

This crate is a straight conversion of the [json-pointer]() crate, but for use
with `simd_json` instead of `serde_json`. While simde_json and serde_json expose
similar APIs, neither attempts to implement the traits of the other, making an 
across-the-board generic implementation quite difficult to achieve. So, instead, I
opted to fork the original project and adapt it to simd_json Value types. The fact
is that if you persuaded by the benefits of simd_json, you can pretty much drop it
in to standard ser/deser workloads in place of serde_json.

I also made some updates to the code to use the 2021 semantics of Rust.

Otherwise, all the code, examples, and tests are those of the original author.

A crate for parsing and using JSON pointers, as specified in [RFC
6901](https://tools.ietf.org/html/rfc6901). Unlike the `pointer` method
built into `serde_json`, this handles both validating JSON Pointers before
use and the URI Fragment Identifier Representation.

## Creating a JSON Pointer

JSON pointers can be created with a literal `[&str]`, or parsed from a `String`.

```rust
let from_strs = JsonPointer::new([
    "foo",
    "bar",
]);
let parsed = "/foo/bar".parse::<JsonPointer<_, _>>().unwrap();

assert_eq!(from_strs.to_string(), parsed.to_string());
}
```

## Using a JSON Pointer

The `JsonPointer` type provides `.get()` and `.get_mut()`, to get references
and mutable references to the appropriate value, respectively.

```rust
let ptr = "/foo/bar".parse::<JsonPointer<_, _>>().unwrap();

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

## URI Fragment Identifier Representation

JSON Pointers can be embedded in the fragment portion of a URI. This is the
reason why most JSON pointer libraries require a `#` character at the beginning
of a JSON pointer. The crate will detect the leading `#` as an indicator to
parse in URI Fragment Identifier Representation. Note that this means that this
crate does not support parsing full URIs.

```rust
let str_ptr = "/f%o".parse::<JsonPointer<_, _>>().unwrap();
let uri_ptr = "#/f%25o".parse::<JsonPointer<_, _>>().unwrap();

assert_eq!(str_ptr, uri_ptr);
```
